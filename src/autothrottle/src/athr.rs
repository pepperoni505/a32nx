#[derive(Debug)]
pub struct AutoThrottleInput {
    pub throttles: [f64; 2],
    pub airspeed: f64,
    pub altitude: f64,
    pub airspeed_hold: f64,
    pub altitude_lock: f64,
    pub radio_height: f64,
    pub autopilot: bool,
    pub pushbutton: bool,
    pub instinctive_disconnect: bool,
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Speed,
    ThrustClimb,
    ThrustDescent,
}

#[derive(Debug)]
pub struct AutoThrottleOutput {
    pub mode: Mode,
    pub engaged: bool,
    pub active: bool,
    pub commanded: f64,
}

pub struct Gates {}
impl Gates {
    pub const IDLE: f64 = 0.0;
    pub const CL: f64 = 89.0;
    pub const FLEX_MCT: f64 = 95.0;
    pub const TOGA: f64 = 100.0;
}

#[derive(Debug, PartialEq)]
enum Instinctive {
    Released,
    Pushed(std::time::Instant),
    Lockout,
}

#[derive(Debug)]
pub struct AutoThrottle {
    speed_mode_pid: crate::pid::PID,
    thrust_rate_limiter: crate::rl::RateLimiter,
    last_altitude_lock: f64,
    last_t: std::time::Instant,
    instinctive: Instinctive,
    input: AutoThrottleInput,
    output: AutoThrottleOutput,
}

impl AutoThrottle {
    pub fn new() -> Self {
        AutoThrottle {
            speed_mode_pid: crate::pid::PID::new(10.0, 1.0, 0.3, 10.0, 0.0, 100.0),
            thrust_rate_limiter: crate::rl::RateLimiter::new(),
            last_altitude_lock: 0.0,
            last_t: std::time::Instant::now(),
            instinctive: Instinctive::Released,
            input: AutoThrottleInput {
                throttles: [0.0, 0.0],
                airspeed: 0.0,
                altitude: 0.0,
                airspeed_hold: 0.0,
                altitude_lock: 0.0,
                radio_height: 0.0,
                autopilot: false,
                pushbutton: false,
                instinctive_disconnect: false,
            },
            output: AutoThrottleOutput {
                mode: Mode::ThrustDescent,
                engaged: false,
                active: false,
                commanded: 0.0,
            },
        }
    }

    pub fn update(&mut self) {
        match self.instinctive {
            Instinctive::Released => {
                if self.input.instinctive_disconnect {
                    self.instinctive = Instinctive::Pushed(std::time::Instant::now());
                }
            }
            Instinctive::Pushed(t) => {
                if self.input.instinctive_disconnect {
                    if t.elapsed().as_secs_f64() >= 15.0 {
                        self.instinctive = Instinctive::Lockout;
                    }
                } else {
                    self.instinctive = Instinctive::Released;
                }
            }
            Instinctive::Lockout => {
                return;
            }
        }

        self.engage_logic();

        if self.output.active {
            self.active_logic();
        } else {
            self.output.commanded = 100.0;
        }
    }

    // FIGURE 22-31-00-130000-A SHEET 1
    // A/THR Engage Logic
    fn engage_logic(&mut self) {
        // - two ADIRS must be valid
        // - one LGCIU must be healthy
        // - guidance portion healthy
        // - management portion healthy
        let ap_fd_athr_common_cond = true;

        // - the two ECUs/EECs must be healthy
        // - the FCU must be healthy
        // - no discrepancy between the N1/EPR target computed in the FMGC and the N1/EPR feedback from each ECU/EEC
        // - VLS, VMAX, etc. must be healthy
        // - instinctive disconnect not held for 15s
        let athr_specific_cond = true;

        // is in alpha floor zone
        let alpha_floor_cond = false;

        let athr_common_or_specific = ap_fd_athr_common_cond || athr_specific_cond;
        let s = athr_common_or_specific
            && (
                // Action on A/THR pushbutton switch
                self.input.pushbutton
                // >MCT ?? idk
                || false
                || alpha_floor_cond
            );

        let r = !athr_common_or_specific
            // if the A/THR function on the opposite FMGC is disengaged and on condition that this FMGC has priority.
            || false
            // Action on the A/THR pushbutton switch, with the A/THR function already engaged.
            || self.input.pushbutton && self.output.engaged
            // Action on one of the A/THR instinctive disconnect pushbuton switches.
            || self.input.instinctive_disconnect
            // ECU/EEC autothrust control feedback i.e. the A/THR being active at level of the
            // FMGCs, one of the two ECUs/EECs indicates that it is not in autothrust control mode.
            || false
            // AP/FD loss condition i.e. total loss of AP/FD below 100ft with the RETARD mode not engaged.
            || false
            // One engine start on the ground
            || false
            // Go around condition i.e. one throttle control lever is placed in the non active
            // area (> MCT) below 100ft without engagement of the GO AROUND mode on the AP/FD.
            || (self.input.radio_height < 100.0 && self.input.throttles.iter().any(|t| *t > Gates::FLEX_MCT))
            // Both throttle control levers placed in the IDLE position.
            // Both throttle control levers placed in the REVERSE position.
            || self.input.throttles.iter().all(|t| *t <= Gates::IDLE);

        // SR flip-flop
        self.output.engaged = if s {
            !r
        } else if r {
            false
        } else {
            self.output.engaged
        };

        // After engagement, A/THR is active if:
        // the Alpha floor protection is active whatever the position of the throttle control levers.
        self.output.active = self.output.engaged
            && (alpha_floor_cond || {
                // The two throttle control levers are between IDLE and CL (CL included)
                let mut all_between_idle_cl = true;

                // one throttle control lever is between IDLE and CL (including CL), and the other is between
                // IDLE and MCT (including MCT) with FLEX TO limit mode not selected
                let mut one_between_idle_flex_mct = false;
                let mut one_between_idle_cl = false;

                for t in &self.input.throttles {
                    if *t > Gates::IDLE && *t <= Gates::CL {
                        one_between_idle_cl = true;
                    } else {
                        all_between_idle_cl = false;
                    }
                    if *t > Gates::IDLE && *t <= Gates::FLEX_MCT {
                        one_between_idle_flex_mct = true;
                    }
                }

                all_between_idle_cl || (one_between_idle_flex_mct && one_between_idle_cl)
            });
    }

    // RukusDM
    fn active_logic(&mut self) {
        let dt = self.last_t.elapsed().as_secs_f64();
        self.last_t = std::time::Instant::now();

        // detect pauses
        #[allow(clippy::float_cmp)]
        if dt == 0.0 {
            return;
        }

        #[allow(clippy::float_cmp)]
        if self.input.autopilot && self.last_altitude_lock != self.input.altitude_lock {
            self.last_altitude_lock = self.input.altitude_lock;
            if self.input.altitude_lock > self.input.altitude {
                self.output.mode = Mode::ThrustClimb;
            } else {
                self.output.mode = Mode::ThrustDescent;
            }
        }

        match self.output.mode {
            Mode::Speed => {
                self.output.commanded = self.thrust_rate_limiter.iterate(
                    self.speed_mode_pid
                        .update(self.input.airspeed_hold, self.input.airspeed, dt),
                    10.0,
                    10.0,
                    dt,
                );
            }
            Mode::ThrustClimb | Mode::ThrustDescent => {
                self.output.commanded = self.thrust_rate_limiter.iterate(
                    if self.output.mode == Mode::ThrustClimb {
                        80.0
                    } else {
                        0.0
                    },
                    4.0,
                    4.0,
                    dt,
                );

                if !self.input.autopilot
                    || (self.input.altitude_lock - self.input.altitude).abs() < 1000.0
                {
                    self.output.mode = Mode::Speed;
                    self.speed_mode_pid.reset(
                        self.input.airspeed_hold,
                        self.input.airspeed,
                        dt,
                        self.output.commanded,
                    );
                    self.thrust_rate_limiter.reset(self.output.commanded);
                }
            }
        }
    }

    pub fn input(&mut self) -> &mut AutoThrottleInput {
        &mut self.input
    }

    pub fn output(&self) -> &AutoThrottleOutput {
        &self.output
    }
}

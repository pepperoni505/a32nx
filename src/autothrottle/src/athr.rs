#[derive(Debug)]
pub struct AutoThrottleInput {
    pub delta_time: std::time::Duration,
    pub throttles: [f64; 2],
    pub airspeed: f64,
    pub altitude: f64,
    pub airspeed_hold: f64,
    pub altitude_lock: f64,
    pub autopilot: bool,
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

#[derive(Debug)]
pub struct AutoThrottle {
    speed_mode_pid: crate::pid::PID,
    thrust_rate_limiter: crate::rl::RateLimiter,
    last_altitude_lock: f64,
    input: AutoThrottleInput,
    output: AutoThrottleOutput,
}

impl AutoThrottle {
    pub fn new() -> Self {
        AutoThrottle {
            // speed_mode_pid: crate::pid::PID::new(10.0, 2.0, 0.3, 50.0, 0.0, 100.0),
            speed_mode_pid: crate::pid::PID::new(10.0, 1.0, 0.0, 10.0, 0.0, 100.0),
            thrust_rate_limiter: crate::rl::RateLimiter::new(),
            last_altitude_lock: 0.0,
            input: AutoThrottleInput {
                delta_time: std::time::Duration::from_millis(0),
                throttles: [0.0, 0.0],
                airspeed: 0.0,
                altitude: 0.0,
                airspeed_hold: 0.0,
                altitude_lock: 0.0,
                autopilot: false,
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

        // pushbutton is pushed
        let athr_pb = true;
        // >MCT ?? idk
        let toga_cond = false;

        // is in alpha floor zone
        let alpha_floor_cond = false;

        let athr_common_or_specific = ap_fd_athr_common_cond || athr_specific_cond;
        let s = athr_common_or_specific && (athr_pb || toga_cond || alpha_floor_cond);

        // if the A/THR function on the opposite FMGC is disengaged and on condition that this FMGC has priority.
        let athr_opp_cond = false;
        // Action on the A/THR pushbutton switch, with the A/THR function already engaged.
        let athr_pb_cond = !athr_pb;
        // Action on one of the A/THR instinctive disconnect pushbutton switches.
        let inst_disconnect = false;
        // ECU/EEC autothrust control feedback i.e. the A/THR being active at level of the FMGCs, one of the two ECUs/EECs indicates that it is not in autothrust control mode.
        let athr_active_feedback_cond = false;
        // Go around condition i.e. one throttle control lever is placed in the non active area (> MCT) below 100ft without engagement of the GO AROUND mode on the AP/FD.
        let ga_cond = false;
        // AP/FD loss condition i.e. total loss of AP/FD below 100ft with the RETARD mode not engaged.
        let loss_of_apfd_cond = false;
        // One engine start on the ground.
        let engine_ground_cond = false;
        // Both throttle control levers placed in the IDLE position.
        let idle_cond = false;

        let r = !athr_common_or_specific
            || athr_opp_cond
            || athr_pb_cond
            || inst_disconnect
            || athr_active_feedback_cond
            || ga_cond
            || loss_of_apfd_cond
            || engine_ground_cond
            || idle_cond;

        // SR flip-flop
        self.output.engaged = if s {
            !r
        } else if r {
            false
        } else {
            self.output.engaged
        };

        let throttle_control_levels_between_idle_and_mct_gates =
            self.input.throttles.iter().all(|t| *t > 0.0 && *t < 100.0);

        self.output.active = self.output.engaged
            && (throttle_control_levels_between_idle_and_mct_gates || alpha_floor_cond);
    }

    // RukusDM
    fn active_logic(&mut self) {
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
                let c = self.speed_mode_pid.update(
                    self.input.airspeed_hold,
                    self.input.airspeed,
                    self.input.delta_time.as_secs_f64(),
                );
                self.output.commanded = self.thrust_rate_limiter.iterate(
                    c,
                    10.0,
                    10.0,
                    self.input.delta_time.as_secs_f64(),
                );
            }
            Mode::ThrustClimb | Mode::ThrustDescent => {
                if !self.input.autopilot
                    || (self.input.altitude_lock - self.input.altitude).abs() < 1000.0
                {
                    self.output.mode = Mode::Speed;
                    self.speed_mode_pid
                        .reset(self.input.airspeed_hold, self.input.airspeed);
                }

                let c = if self.output.mode == Mode::ThrustClimb {
                    80.0
                } else {
                    0.0
                };

                self.output.commanded = self.thrust_rate_limiter.iterate(
                    c,
                    2.5,
                    2.5,
                    self.input.delta_time.as_secs_f64(),
                );
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

/*
fn ramp(setpoint: f64, output: f64, rate: f64) -> f64 {
    if setpoint > output {
        (output + rate).min(setpoint)
    } else {
        (output - rate).max(setpoint)
    }
}
*/

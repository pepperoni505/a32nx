use ::msfs::{
    msfs,
    sim_connect::{data_definition, Period, SimConnectRecv, SIMCONNECT_OBJECT_ID_USER},
};
mod athr;
mod pid;
mod rl;

#[data_definition]
#[derive(Debug)]
struct Flight {
    #[name = "AIRSPEED INDICATED"]
    #[unit = "knots"]
    airspeed: f64,
    #[name = "INDICATED ALTITUDE"]
    #[unit = "feet"]
    altitude: f64,
    #[name = "AUTOPILOT AIRSPEED HOLD VAR"]
    #[unit = "knots"]
    airspeed_hold: f64,
    #[name = "AUTOPILOT ALTITUDE LOCK VAR"]
    #[unit = "feet"]
    altitude_lock: f64,
    #[name = "RADIO HEIGHT"]
    #[unit = "feet"]
    radio_height: f64,
    #[name = "AUTOPILOT MASTER"]
    #[unit = "bool"]
    autopilot: bool,
}

#[data_definition]
#[derive(Debug)]
struct Output {
    #[name = "GENERAL ENG THROTTLE LEVER POSITION:1"]
    #[unit = "percent"]
    t1: f64,
    #[name = "GENERAL ENG THROTTLE LEVER POSITION:2"]
    #[unit = "percent"]
    t2: f64,
}

fn map(n: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    (n - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

const MAX_THROTTLE: f64 = 100.0;
const MIN_THROTTLE: f64 = -20.0;

fn map_virtual(n: std::os::raw::c_ulong) -> f64 {
    let n = n as std::os::raw::c_long as f64;
    if n > 0.0 {
        map(n, 0.0, 16384.0, 0.0, MAX_THROTTLE)
    } else {
        map(n, -3277.0, 0.0, MIN_THROTTLE, 0.0)
    }
}

fn map_real(n: std::os::raw::c_ulong, reverse_toggle: bool, reverse_hold: bool) -> f64 {
    const MAX: f64 = 16384.0;
    let n = n as std::os::raw::c_long as f64;
    if reverse_toggle || reverse_hold {
        map(n, -MAX, MAX, 0.0, MIN_THROTTLE)
    } else {
        map(n, -MAX, MAX, 0.0, MAX_THROTTLE)
    }
}

const INC_DELTA: f64 = 2.5;
const INC_DELTA_SMALL: f64 = 1.0;
fn inc(t: &mut f64, d: f64) {
    let n = *t + d;
    if (n - 1.0).abs() <= 0.5 {
        *t = 0.0;
    } else {
        *t = n;
    }
}

fn dec(t: &mut f64, d: f64) {
    let n = *t - d;
    if (n - 1.0).abs() <= 0.5 {
        *t = 0.0;
    } else {
        *t = n;
    }
}

#[msfs::standalone_module]
pub async fn module(mut module: msfs::StandaloneModule) -> Result<(), Box<dyn std::error::Error>> {
    let mut sim = module.open_simconnect("ATHR")?;
    let mut athr = athr::AutoThrottle::new();

    let tset_id = sim.map_client_event_to_sim_event("THROTTLE_SET", true)?;
    let t1set_id = sim.map_client_event_to_sim_event("THROTTLE1_SET", true)?;
    let t2set_id = sim.map_client_event_to_sim_event("THROTTLE2_SET", true)?;

    let exset_id = sim.map_client_event_to_sim_event("THROTTLE_AXIS_SET_EX1", true)?;
    let ex1set_id = sim.map_client_event_to_sim_event("THROTTLE1_AXIS_SET_EX1", true)?;
    let ex2set_id = sim.map_client_event_to_sim_event("THROTTLE2_AXIS_SET_EX1", true)?;

    let revtog_id = sim.map_client_event_to_sim_event("THROTTLE_REVERSE_THRUST_TOGGLE", true)?;
    let revhold_id = sim.map_client_event_to_sim_event("THROTTLE_REVERSE_THRUST_HOLD", true)?;

    let thrinc_id = sim.map_client_event_to_sim_event("THROTTLE_INCR", true)?;
    let thrdec_id = sim.map_client_event_to_sim_event("THROTTLE_DECR", true)?;
    let thrincs_id = sim.map_client_event_to_sim_event("THROTTLE_INCR_SMALL", true)?;
    let thrdecs_id = sim.map_client_event_to_sim_event("THROTTLE_DECR_SMALL", true)?;

    let thr1inc_id = sim.map_client_event_to_sim_event("THROTTLE1_INCR", true)?;
    let thr1dec_id = sim.map_client_event_to_sim_event("THROTTLE1_DECR", true)?;
    let thr1incs_id = sim.map_client_event_to_sim_event("THROTTLE1_INCR_SMALL", true)?;
    let thr1decs_id = sim.map_client_event_to_sim_event("THROTTLE1_DECR_SMALL", true)?;

    let thr2inc_id = sim.map_client_event_to_sim_event("THROTTLE2_INCR", true)?;
    let thr2dec_id = sim.map_client_event_to_sim_event("THROTTLE2_DECR", true)?;
    let thr2incs_id = sim.map_client_event_to_sim_event("THROTTLE2_INCR_SMALL", true)?;
    let thr2decs_id = sim.map_client_event_to_sim_event("THROTTLE2_DECR_SMALL", true)?;

    sim.request_data_on_sim_object::<Flight>(0, SIMCONNECT_OBJECT_ID_USER, Period::SimFrame)?;

    let mut reverse_toggle = false;
    let mut reverse_hold = false;
    let mut t1 = 0.0;
    let mut t2 = 0.0;

    while let Some(recv) = module.next_event().await {
        match recv {
            SimConnectRecv::Event(event) => match event.id() {
                x if x == tset_id => {
                    let data = map_virtual(event.data());
                    t1 = data;
                    t2 = data;
                }
                // FIXME: virtual cockpit throttle levers need to be supported.
                //        this involves changes to the XML.
                x if x == t1set_id => {
                    // t1 = map_virtual(event.data());
                }
                x if x == t2set_id => {
                    // t2 = map_virtual(event.data());
                }
                x if x == exset_id => {
                    let data = map_real(event.data(), reverse_toggle, reverse_hold);
                    t1 = data;
                    t2 = data;
                }
                x if x == ex1set_id => {
                    t1 = map_real(event.data(), reverse_toggle, reverse_hold);
                }
                x if x == ex2set_id => {
                    t2 = map_real(event.data(), reverse_toggle, reverse_hold);
                }
                x if x == revtog_id => {
                    reverse_toggle = !reverse_toggle;
                }
                x if x == revhold_id => {
                    reverse_hold = event.data() == 1;
                }
                x if x == thrinc_id => {
                    inc(&mut t1, INC_DELTA);
                    inc(&mut t2, INC_DELTA);
                }
                x if x == thrdec_id => {
                    dec(&mut t1, INC_DELTA);
                    dec(&mut t2, INC_DELTA);
                }
                x if x == thrincs_id => {
                    inc(&mut t1, INC_DELTA_SMALL);
                    inc(&mut t2, INC_DELTA_SMALL);
                }
                x if x == thrdecs_id => {
                    dec(&mut t1, INC_DELTA_SMALL);
                    dec(&mut t2, INC_DELTA_SMALL);
                }
                x if x == thr1inc_id => inc(&mut t1, INC_DELTA),
                x if x == thr1dec_id => dec(&mut t1, INC_DELTA),
                x if x == thr1incs_id => inc(&mut t1, INC_DELTA_SMALL),
                x if x == thr1decs_id => dec(&mut t1, INC_DELTA_SMALL),
                x if x == thr2inc_id => inc(&mut t2, INC_DELTA),
                x if x == thr2dec_id => dec(&mut t2, INC_DELTA),
                x if x == thr2incs_id => inc(&mut t2, INC_DELTA_SMALL),
                x if x == thr2decs_id => inc(&mut t2, INC_DELTA_SMALL),
                _ => unreachable!(),
            },
            SimConnectRecv::SimObjectData(data) => match data.id() {
                0 => {
                    let data = data.into::<Flight>(&sim).unwrap();
                    let input = athr.input();
                    input.airspeed = data.airspeed;
                    input.altitude = data.altitude;
                    input.airspeed_hold = data.airspeed_hold;
                    input.altitude_lock = data.altitude_lock;
                    input.radio_height = data.radio_height;
                    input.autopilot = data.autopilot;
                }
                _ => unreachable!(),
            },
            _ => {}
        }

        // FIXME: missing ATHR momentary and instinctive disconnect inputs

        {
            let mut input = athr.input();

            let map = |t| {
                for (low, high, target) in &[
                    (86.6, 90.0, athr::Gates::CL),
                    (94.0, 96.0, athr::Gates::FLEX_MCT),
                    (99.5, 100.0, athr::Gates::TOGA),
                ] {
                    if t >= *low && t <= *high {
                        return *target;
                    }
                }
                t
            };

            input.throttles = [map(t1), map(t2)];
        }

        athr.update();

        {
            let output = athr.output();
            let odata = Output {
                t1: output.commanded.min(t1),
                t2: output.commanded.min(t2),
            };
            sim.set_data_on_sim_object(SIMCONNECT_OBJECT_ID_USER, &odata)?;
        }
    }

    Ok(())
}

#[derive(Debug)]
pub struct PID {
    kp: f64,
    ki: f64,
    kd: f64,
    n: f64,
    min: f64,
    max: f64,
    y: [f64; 3],
    e: [f64; 3],
}

impl PID {
    pub(crate) const fn new(kp: f64, ki: f64, kd: f64, n: f64, min: f64, max: f64) -> Self {
        PID {
            kp,
            kd,
            ki,
            n,
            min,
            max,
            y: [0.0, 0.0, 0.0],
            e: [0.0, 0.0, 0.0],
        }
    }

    pub(crate) fn update(&mut self, setpoint: f64, value: f64, ts: f64, reset: bool) -> f64 {
        if reset {
            self.y = [value, value, value];
            let e = setpoint - value;
            self.e = [e, e, e];
        }

        // Calculate rollup parameters
        let k: f64 = 2.0 / ts;
        let b0 = k.powf(2.0) * self.kp
            + k * self.ki
            + self.ki * self.n
            + k * self.kp * self.n
            + k.powf(2.0) * self.kd * self.n;
        let b1 = 2.0 * self.ki * self.n
            - 2.0 * k.powf(2.0) * self.kp
            - 2.0 * k.powf(2.0) * self.kd * self.n;
        let b2 = k.powf(2.0) * self.kp - k * self.ki + self.ki * self.n - k * self.kp * self.n
            + k.powf(2.0) * self.kd * self.n;
        let a0 = k.powf(2.0) + self.n * k;
        let a1 = -2.0 * k.powf(2.0);
        let a2 = k.powf(2.0) - k * self.n;

        // Age errors and output history
        self.e[2] = self.e[1]; // Age errors one iteration
        self.e[1] = self.e[0]; // Age errors one iteration
        self.e[0] = setpoint - value; // Compute new error
        self.y[2] = self.y[1]; // Age outputs one iteration
        self.y[1] = self.y[0]; // Age outputs one iteration
        self.y[0] = -a1 / a0 * self.y[1] - a2 / a0 * self.y[2]
            + b0 / a0 * self.e[0]
            + b1 / a0 * self.e[1]
            + b2 / a0 * self.e[2]; // Calculate current output

        clamp(self.y[0], self.min, self.max)
    }
}

fn clamp(mut x: f64, min: f64, max: f64) -> f64 {
    debug_assert!(min < max);
    if x < min {
        x = min;
    }
    if x > max {
        x = max;
    }
    x
}

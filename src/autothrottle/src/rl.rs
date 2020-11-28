#[derive(Debug)]
pub struct RateLimiter {
    y_slope: f64,
    in_: [f64; 2],
    out: [f64; 2],
}

impl RateLimiter {
    pub fn new() -> Self {
        RateLimiter {
            y_slope: 0.0,
            in_: [0.0, 0.0],
            out: [0.0, 0.0],
        }
    }

    pub fn initialize(&mut self, i: f64) {
        self.in_ = [i, i];
        self.out = [i, i];
    }

    pub fn iterate(&mut self, i: f64, a_rate: f64, d_rate: f64, ts: f64) -> f64 {
        self.in_[0] = i;

        let slope = (self.in_[0] - self.out[1]) / ts;

        if slope <= -d_rate {
            self.y_slope = -d_rate;
        }

        if -d_rate <= slope && slope <= a_rate {
            self.y_slope = slope;
        }

        if a_rate <= slope {
            self.y_slope = a_rate;
        }

        self.out[0] = self.out[1] + (ts * self.y_slope);
        self.out[1] = self.out[0];
        self.in_[1] = self.in_[0];

        self.out[0]
    }
}

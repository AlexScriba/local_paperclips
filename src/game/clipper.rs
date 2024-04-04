use crate::interval::IntervalTimer;

const BASE_CLIPPER_PRICE: f64 = 10.0;

#[derive(Debug)]
pub struct ClipperManager {
    interval: IntervalTimer,
    num_clippers: u32,
}

impl ClipperManager {
    pub fn new() -> ClipperManager {
        ClipperManager {
            interval: IntervalTimer::new(1000),
            num_clippers: 0,
        }
    }

    pub fn increment(&mut self) {
        self.num_clippers += 1;
    }

    pub fn num_clippers(&self) -> u32 {
        self.num_clippers
    }

    pub fn produce_clips(&mut self) -> u128 {
        if self.interval.elapsed_and_clear() {
            return self.num_clippers as u128;
        }

        0
    }

    pub fn buy_price(&self) -> f64 {
        BASE_CLIPPER_PRICE + (1.1 as f64).powf(self.num_clippers as f64) - 1.0
    }
}

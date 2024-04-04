use std::time::SystemTime;


#[derive(Debug)]
pub struct IntervalTimer {
    interval_duration: u128,
    interval_start: SystemTime
}

impl IntervalTimer {
    pub fn new(duration: u128) -> IntervalTimer {
        let start = SystemTime::now();

        IntervalTimer{
            interval_start: start,
            interval_duration: duration
        }
    }

    pub fn elapsed_and_clear(&mut self) -> bool {
        let now = SystemTime::now();

        let elapsed = now
            .duration_since(self.interval_start)
            .expect("Interval timer error.");

        if elapsed.as_millis() >= self.interval_duration {
            self.interval_start = now;
            return true;
        }

        false
    }
}

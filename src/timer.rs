use std::time::Instant;

pub struct Timer {
    time: Instant
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            time: Instant::now()
        }
    }

    pub fn current_time(&self) -> f64 {
        let secs = self.time.elapsed().as_secs() as f64;
        let nanos = self.time.elapsed().subsec_nanos() as f64;

        secs * 1_000.0 + nanos / 1_000_000.0
    }
}

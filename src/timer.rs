extern crate time;

pub fn current_time() -> f64 {
    (time::precise_time_ns() / 1_000_000) as f64
}

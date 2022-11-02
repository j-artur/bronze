use std::{
    mem::replace,
    time::{Duration, Instant},
};

pub struct Timer {
    last_instant: Instant,
}

impl Timer {
    pub fn new() -> Timer {
        let last_instant = Instant::now();
        Timer { last_instant }
    }

    pub fn start(&mut self) {
        self.last_instant = Instant::now();
    }

    pub fn reset(&mut self) -> Duration {
        let now = Instant::now();
        now - replace(&mut self.last_instant, now)
    }

    pub fn elapsed(&self) -> Duration {
        Instant::now() - self.last_instant
    }

    pub fn has_elapsed(&self, duration: Duration) -> bool {
        self.elapsed() >= duration
    }
}

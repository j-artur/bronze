use winapi::um::{profileapi::*, winnt::*};

pub struct Timer {
    start: LARGE_INTEGER,
    end: LARGE_INTEGER,
    frequency: LARGE_INTEGER,
    stopped: bool,
}

impl Timer {
    pub fn new() -> Self {
        unsafe {
            Timer {
                start: std::mem::zeroed(),
                end: std::mem::zeroed(),
                frequency: {
                    let mut frequency = std::mem::zeroed();
                    QueryPerformanceFrequency(&mut frequency);
                    frequency
                },
                stopped: false,
            }
        }
    }

    pub fn start(&mut self) {
        unsafe {
            if self.stopped {
                let elapsed = self.end.QuadPart() - self.start.QuadPart();
                QueryPerformanceCounter(&mut self.start);
                *self.start.QuadPart_mut() -= elapsed;
            } else {
                QueryPerformanceCounter(&mut self.start);
            }
        }
    }

    pub fn stop(&mut self) {
        if !self.stopped {
            unsafe { QueryPerformanceCounter(&mut self.end) };
            self.stopped = true;
        }
    }

    pub fn reset(&mut self) -> f64 {
        unsafe {
            let elapsed = if self.stopped {
                let elapsed = self.end.QuadPart() - self.start.QuadPart();
                QueryPerformanceCounter(&mut self.start);
                self.stopped = false;
                elapsed
            } else {
                QueryPerformanceCounter(&mut self.end);
                let elapsed = self.end.QuadPart() - self.start.QuadPart();
                self.start = self.end;
                elapsed
            };

            elapsed as f64 / *self.frequency.QuadPart() as f64
        }
    }

    pub fn elapsed(&mut self) -> f64 {
        unsafe {
            let elapsed = if self.stopped {
                self.end.QuadPart() - self.start.QuadPart()
            } else {
                QueryPerformanceCounter(&mut self.end);
                self.end.QuadPart() - self.start.QuadPart()
            };

            elapsed as f64 / *self.frequency.QuadPart() as f64
        }
    }

    pub fn has_elapsed(&mut self, seconds: f64) -> bool {
        self.elapsed() >= seconds
    }
}

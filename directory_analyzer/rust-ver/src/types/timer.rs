use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Timer {
    start_time: Instant,
    //How long from start_time needs to pass before the timer ends
    duration: Duration,
    ended: bool,
}

impl Timer {
    pub fn new(duration: u64) -> Self {
        let duration = Duration::from_secs(duration);
        Self {
            start_time: Instant::now(),
            duration,
            ended: false,
        }
    }

    ///Used to know if the timer has run as long as required
    pub fn ended(&self) -> bool {
        self.ended
    }

    ///Determines how much time has passed since the start time.
    ///
    ///If this time is longer than the duration the timer ends
    pub fn update(&mut self) {
        let now = self.start_time.elapsed();
        if now.as_secs_f64() >= self.duration.as_secs_f64() {
            self.ended = true;
        }
    }

    ///Sets the start time to now and ended to false
    pub fn reset(&mut self) {
        self.start_time = Instant::now();
        self.ended = false;
    }
}

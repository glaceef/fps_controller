use std::time::{Instant, Duration};
use std::thread::sleep;

pub struct FPSController {
    fps: f64,
    frame_time: f64,
    frame_count: u64,
    time: Instant,
    extra_time: f64,
}

impl FPSController {
    /// Create a default struct for 60 fps.
    pub fn new() -> Self {
        Self::from_fps(60)
    }

    pub fn from_fps(fps: u64) -> Self {
        let fps = fps as f64;
        FPSController {
            fps,
            frame_time: 1.0 / fps,
            frame_count: 0,
            time: Instant::now(),
            extra_time: 0.0
        }
    }

    pub fn tick(&mut self) -> u64 {
        self.frame_count += 1;
        self.frame_count
    }

    pub fn run<F: FnMut(u64)>(&mut self, mut update: F) -> f64 {
        let time = self.time.elapsed();
        let time_as_millis = time.subsec_millis(); // 0 ~ 999
        let time_as_secs = time.as_secs(); // 0 ~
        let current_time = time_as_secs as f64 + time_as_millis as f64 / 1000.0;
        let true_time = self.frame_count as f64 / self.fps as f64;
        let sleep_time = true_time - current_time;
        if sleep_time >= 0.0 {
            if sleep_time > self.extra_time {
                let sleep_time = sleep_time - self.extra_time;
                self.extra_time = 0.0;
                sleep(Duration::from_millis((sleep_time * 1000.0) as u64));
            } else {
                self.extra_time -= sleep_time;
            }
        } else {
            self.extra_time += -sleep_time;
            while self.extra_time >= self.frame_time {
                self.frame_count += 1;
                update(self.frame_count);
                self.extra_time -= self.frame_time;
            }
        }
        let time = self.time.elapsed();
        let time_as_secs = time.as_secs() as f64 + time.subsec_millis() as f64 / 1000.0;
        let fps = self.frame_count as f64 / time_as_secs;
        fps
    }
}

use std::thread::sleep;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub(crate) struct Timer
{
    frame_start: Instant,
    frame_time: Duration,
}

impl Timer
{
    pub fn new(fps: f32) -> Self
    {
        let estimated_frame_time = 1.0 / fps;
        Self {
            frame_start: Instant::now(),
            frame_time: Duration::from_secs_f32(estimated_frame_time),
        }
    }

    pub fn start_frame(&mut self) -> f32
    {
        let dt = self.frame_start.elapsed().as_secs_f32();
        self.frame_start = Instant::now();
        dt
    }

    pub fn end_frame(&mut self)
    {
        let sleep_time = self.frame_time.saturating_sub(self.frame_start.elapsed());
        sleep(sleep_time);
    }
}

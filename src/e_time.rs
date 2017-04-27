
use std::time::{Instant, Duration};


pub struct Time {
    last_time: Instant,
    delta_time: u32,
}


impl Time {
    pub fn new() -> Self {
        let last_time = Instant::now();
        let delta_time = 0 as u32;

        Time {last_time: last_time, delta_time: delta_time}
    }

    pub fn update(&mut self){
        //std time based delta time calculation
        let current_time =  Instant::now();
        self.delta_time = self.last_time.elapsed().subsec_nanos();
        self.last_time = current_time;
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time as f32 / 1_000_000_000.0
    }

}

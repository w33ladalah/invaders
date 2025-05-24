use std::time::Duration;
use rusty_time::timer::Timer;

pub struct Laser {
    pub x: usize,
    pub y: usize,
    pub exploded: bool,
    timer: Timer,
}

impl Laser {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploded: false,
            timer: Timer::from_millis(60),
        }
    }
    
    pub fn update(&mut self, d: Duration) {
        self.timer.update(d);

        if self.timer.ready && !self.exploded {
            if self.y > 0 {
                self.y -= 1;
            }
            self.timer.reset();
        }
    }
    
    pub fn explode(&mut self) {
        self.exploded = true;
        self.timer = Timer::from_millis(300);
    }
    
    pub fn destroyed(&self) -> bool {
        (self.exploded && self.timer.ready) || (self.y == 0)
    }
}
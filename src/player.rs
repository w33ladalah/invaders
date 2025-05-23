use std::time::Duration;
use crate::{NUM_COLS, NUM_ROWS};
use crate::laser::Laser;

pub struct Player {
    pub x: usize,
    pub y: usize,
    pub shots: Vec<Laser>,
}

impl Player {
    pub fn new() -> Self {
        // Set player position at the center bottom of the frame
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            shots: Vec::new(),
        }
    }

    pub fn go_to_left(&mut self) -> bool {
        if self.x > 0 {
            self.x -= 1;
            true
        } else {
            false
        }
    }

    pub fn go_to_right(&mut self) -> bool {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
            true
        } else {
            false
        }
    }

    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < 2 {
            self.shots.push(Laser::new(self.x, self.y - 1));
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, d: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(d);
        }
        self.shots.retain(|shot| !shot.destroyed());
    }
}

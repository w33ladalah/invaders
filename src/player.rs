use std::time::Duration;
use crate::{NUM_COLS, NUM_ROWS};
use crate::frame::{Drawable, Frame};
use crate::laser::Laser;

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Laser>,
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
    pub fn go_to_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }
    pub fn go_to_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
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
        self.shots.retain(|shot| !shot.destroyed() );
    }
}

// Make player rendered to the screen
impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "H";
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}
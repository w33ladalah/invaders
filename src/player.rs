use crate::{NUM_COLS, NUM_ROWS};
use crate::frame::{Drawable, Frame};

pub struct Player {
    x: usize,
    y: usize,
}

impl Player {
    pub fn new() -> Self {
        // Set player position at the center bottom of the frame
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1
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
}

// Make player rendered to the screen
impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "H";
    }
}
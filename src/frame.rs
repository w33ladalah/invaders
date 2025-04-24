use crate::{NUM_COLS, NUM_ROWS};

// Use 'static lifetime because we want this Frame variable always available
// in the entire duration of the program's execution.
pub type Frame = Vec<Vec<&'static str>>;

pub fn new_frame() -> Frame {
    let mut cols: Vec<Vec<&str>> = Vec::with_capacity(NUM_COLS);

    for _ in 0..NUM_COLS {
        let mut row = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS {
            row.push(" ");
        }
        cols.push(row);
    }

    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}

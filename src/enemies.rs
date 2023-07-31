use rusty_time::timer::Timer;

pub struct Enemy {
    x: usize,
    y: usize,
}

pub struct Enemies {
    pub army: Vec<Enemy>,
    move_timer: Timer,
}
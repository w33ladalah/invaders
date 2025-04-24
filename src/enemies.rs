use rusty_time::timer::Timer;

pub struct Enemy {
    pub x: usize,
    pub y: usize,
    pub alive: bool,
}

pub struct Enemies {
    pub army: Vec<Enemy>,
    move_timer: Timer,
    direction: i32,
}

impl Enemies {
    pub fn new() -> Self {
        let mut army = Vec::new();
        // Create a grid of enemies at the top of the screen
        for x in 5..35 {
            for y in 0..3 {
                army.push(Enemy { x, y, alive: true });
            }
        }
        Self {
            army,
            move_timer: Timer::from_millis(500),
            direction: 1,
        }
    }

    pub fn update(&mut self, d: std::time::Duration) {
        self.move_timer.update(d);
        if self.move_timer.ready {
            for enemy in self.army.iter_mut() {
                if enemy.alive {
                    let new_x = (enemy.x as i32 + self.direction) as usize;
                    enemy.x = new_x;
                }
            }
            self.move_timer.reset();
        }
    }

    pub fn draw(&self, frame: &mut crate::frame::Frame) {
        for enemy in self.army.iter() {
            if enemy.alive {
                frame[enemy.x][enemy.y] = "V";
            }
        }
    }

    pub fn all_dead(&self) -> bool {
        self.army.iter().all(|e| !e.alive)
    }

    pub fn reached_bottom(&self) -> bool {
        self.army.iter().any(|e| e.alive && e.y >= crate::NUM_ROWS - 1)
    }

    pub fn hit_by(&mut self, player_shots: &mut Vec<crate::laser::Laser>) -> bool {
        let mut hit = false;
        for enemy in self.army.iter_mut() {
            if enemy.alive {
                for shot in player_shots.iter_mut() {
                    if !shot.exploded && shot.x == enemy.x && shot.y == enemy.y {
                        enemy.alive = false;
                        shot.explode();
                        hit = true;
                    }
                }
            }
        }
        hit
    }
}

impl crate::frame::Drawable for Enemies {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        self.draw(frame);
    }
}
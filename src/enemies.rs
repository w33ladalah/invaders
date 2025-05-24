use rusty_time::timer::Timer;
use std::time::Duration;

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

    pub fn update(&mut self, d: Duration) {
        self.move_timer.update(d);
        if self.move_timer.ready {
            let mut change_direction = false;
            // Check if any enemy would go out of bounds
            for enemy in self.army.iter() {
                if enemy.alive {
                    let next_x = enemy.x as i32 + self.direction;
                    if next_x < 0 || next_x >= crate::NUM_COLS as i32 {
                        change_direction = true;
                        break;
                    }
                }
            }
            if change_direction {
                self.direction *= -1;
                // Move enemies down one row
                for enemy in self.army.iter_mut() {
                    if enemy.alive {
                        enemy.y += 1;
                    }
                }
            } else {
                for enemy in self.army.iter_mut() {
                    if enemy.alive {
                        enemy.x = (enemy.x as i32 + self.direction) as usize;
                    }
                }
            }
            self.move_timer.reset();
        }
    }

    pub fn all_dead(&self) -> bool {
        self.army.iter().all(|e| !e.alive)
    }

    pub fn reached_bottom(&self) -> bool {
        self.army.iter().any(|e| e.alive && e.y >= crate::NUM_ROWS - 1)
    }

    pub fn hit_by(&mut self, player_shots: &mut Vec<crate::laser::Laser>) -> usize {
        let mut destroyed = 0;
        for enemy in self.army.iter_mut() {
            if enemy.alive {
                for shot in player_shots.iter_mut() {
                    if !shot.exploded && shot.x == enemy.x && shot.y == enemy.y {
                        enemy.alive = false;
                        shot.explode();
                        destroyed += 1;
                    }
                }
            }
        }
        destroyed
    }
}

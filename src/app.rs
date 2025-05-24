use std::time::Instant;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use invaders::enemies::Enemies;
use invaders::player::Player;
use invaders::{NUM_COLS, NUM_ROWS};

pub enum GameState {
    Playing,
    GameOver,
    Win,
}

pub struct App {
    pub player: Player,
    pub enemies: Enemies,
    pub destroyed_count: usize,
    pub high_score: usize,
    pub state: GameState,
    pub should_quit: bool,
    pub last_tick: Instant,
}

impl App {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            enemies: Enemies::new(),
            destroyed_count: 0,
            high_score: 0,
            state: GameState::Playing,
            should_quit: false,
            last_tick: Instant::now(),
        }
    }

    pub fn restart(&mut self) {
        self.player = Player::new();
        self.enemies = Enemies::new();
        self.destroyed_count = 0;
        self.state = GameState::Playing;
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        let delta = now - self.last_tick;
        self.last_tick = now;

        match self.state {
            GameState::Playing => {
                // Update game elements
                self.player.update(delta);
                self.enemies.update(delta);

                // Check for collisions
                let destroyed = self.enemies.hit_by(&mut self.player.shots);
                if destroyed > 0 {
                    self.destroyed_count += destroyed;
                }

                // Check win/lose conditions
                if self.enemies.all_dead() {
                    self.state = GameState::Win;
                    if self.destroyed_count > self.high_score {
                        self.high_score = self.destroyed_count;
                    }
                } else if self.enemies.reached_bottom() {
                    self.state = GameState::GameOver;
                    if self.destroyed_count > self.high_score {
                        self.high_score = self.destroyed_count;
                    }
                }
            }
            _ => {}
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        match self.state {
            GameState::Playing => self.render_game(frame),
            GameState::GameOver => self.render_game_over(frame, "Game Over!"),
            GameState::Win => self.render_game_over(frame, "You Win!"),
        }
    }

    fn render_game(&self, frame: &mut Frame) {
        let game_area = frame.size();

        // Create a game board with borders
        let block = Block::default()
            .title("Space Invaders")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White));

        let inner_area = block.inner(game_area);
        frame.render_widget(block, game_area);

        // Calculate cell size
        let cell_width = inner_area.width as f32 / NUM_COLS as f32;
        let cell_height = inner_area.height as f32 / NUM_ROWS as f32;

        // Render player
        let player_x = (self.player.x as f32 * cell_width) as u16 + inner_area.x;
        let player_y = (self.player.y as f32 * cell_height) as u16 + inner_area.y;

        if player_x < inner_area.x + inner_area.width && player_y < inner_area.y + inner_area.height {
            let player_cell = Rect::new(player_x, player_y, 1, 1);
            frame.render_widget(
                Paragraph::new("H").style(Style::default().fg(Color::Green)),
                player_cell,
            );
        }

        // Render player shots
        for shot in &self.player.shots {
            let shot_x = (shot.x as f32 * cell_width) as u16 + inner_area.x;
            let shot_y = (shot.y as f32 * cell_height) as u16 + inner_area.y;

            if shot_x < inner_area.x + inner_area.width && shot_y < inner_area.y + inner_area.height {
                let shot_cell = Rect::new(shot_x, shot_y, 1, 1);
                let shot_char = if shot.exploded { "*" } else { "|" };
                frame.render_widget(
                    Paragraph::new(shot_char).style(Style::default().fg(Color::Yellow)),
                    shot_cell,
                );
            }
        }

        // Render enemies
        for enemy in &self.enemies.army {
            if enemy.alive {
                let enemy_x = (enemy.x as f32 * cell_width) as u16 + inner_area.x;
                let enemy_y = (enemy.y as f32 * cell_height) as u16 + inner_area.y;

                if enemy_x < inner_area.x + inner_area.width && enemy_y < inner_area.y + inner_area.height {
                    let enemy_cell = Rect::new(enemy_x, enemy_y, 1, 1);
                    frame.render_widget(
                        Paragraph::new("V").style(Style::default().fg(Color::Red)),
                        enemy_cell,
                    );
                }
            }
        }

        // Render score
        let score_text = format!("Score: {}", self.destroyed_count);
        let score_widget = Paragraph::new(score_text)
            .style(Style::default().fg(Color::White));

        let score_area = Rect::new(
            inner_area.x,
            inner_area.y + inner_area.height - 1,
            inner_area.width,
            1,
        );

        frame.render_widget(score_widget, score_area);
    }

    fn render_game_over(&self, frame: &mut Frame, title: &str) {
        let area = frame.size();

        // Create layout for game over screen
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Length(7),
                Constraint::Percentage(30),
            ])
            .split(area);

        let game_over_block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Yellow));

        let inner_area = game_over_block.inner(chunks[1]);
        frame.render_widget(game_over_block, chunks[1]);

        // Create layout for text
        let text_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(inner_area);

        // Render score info
        let score_text = Line::from(vec![
            Span::styled("Enemies destroyed: ", Style::default().fg(Color::White)),
            Span::styled(self.destroyed_count.to_string(), Style::default().fg(Color::Green)),
        ]);

        let high_score_text = Line::from(vec![
            Span::styled("High score: ", Style::default().fg(Color::White)),
            Span::styled(self.high_score.to_string(), Style::default().fg(Color::Green)),
        ]);

        let restart_text = Line::from(vec![
            Span::styled("Press ", Style::default().fg(Color::White)),
            Span::styled("R", Style::default().fg(Color::Yellow)),
            Span::styled(" to restart or ", Style::default().fg(Color::White)),
            Span::styled("Q", Style::default().fg(Color::Yellow)),
            Span::styled(" to quit", Style::default().fg(Color::White)),
        ]);

        frame.render_widget(Paragraph::new(score_text), text_chunks[0]);
        frame.render_widget(Paragraph::new(high_score_text), text_chunks[1]);
        frame.render_widget(Paragraph::new(restart_text), text_chunks[3]);
    }
}

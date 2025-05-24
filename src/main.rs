mod app;

use std::io;
use std::time::{Duration, Instant};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, ExecutableCommand};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use app::{App, GameState};
use invaders::sound::init_sounds;

fn main() -> Result<()> {
    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;

    // Create terminal with crossterm backend
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();
    
    // Initialize audio
    let mut audio = init_sounds();
    audio.play("startup");

    // Main loop
    let tick_rate = Duration::from_millis(16); // ~60 FPS
    let mut last_tick = Instant::now();

    loop {
        // Draw UI
        terminal.draw(|frame| app.render(frame))?;

        // Handle input
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match app.state {
                    GameState::Playing => match key.code {
                        KeyCode::Left => {
                            if app.player.go_to_left() {
                                audio.play("move");
                            }
                        },
                        KeyCode::Right => {
                            if app.player.go_to_right() {
                                audio.play("move");
                            }
                        },
                        KeyCode::Char(' ') | KeyCode::Enter => {
                            if app.player.shoot() {
                                audio.play("pew");
                            }
                        },
                        KeyCode::Char('q') | KeyCode::Esc => {
                            app.should_quit = true;
                            audio.play("lose");
                        },
                        _ => {},
                    },
                    GameState::GameOver | GameState::Win => match key.code {
                        KeyCode::Char('r') | KeyCode::Char('R') => {
                            app.restart();
                            audio = init_sounds();
                            audio.play("startup");
                        },
                        KeyCode::Char('q') | KeyCode::Esc | KeyCode::Char('Q') => {
                            app.should_quit = true;
                        },
                        _ => {},
                    },
                }
            }
        }

        // Check if we should quit
        if app.should_quit {
            break;
        }

        // Tick
        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
            
            // Play sounds based on state changes
            match app.state {
                GameState::GameOver => audio.play("lose"),
                GameState::Win => audio.play("win"),
                _ => {}
            }
            
            // Play explosion sound if enemies were destroyed
            let destroyed = app.enemies.hit_by(&mut app.player.shots);
            if destroyed > 0 {
                audio.play("explode");
            }
        }
    }

    // Restore terminal
    terminal::disable_raw_mode()?;
    stdout = io::stdout();
    stdout.execute(LeaveAlternateScreen)?;
    stdout.execute(cursor::Show)?;

    Ok(())
}

use std::error::Error;
use std::{io, thread};
use std::sync::mpsc;
use std::time::{Duration, Instant};
use crossterm::{event, ExecutableCommand, terminal};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use invaders::{frame, render};
use invaders::frame::{Drawable, new_frame};
use invaders::player::Player;
use invaders::sound::init_sounds;
use invaders::enemies::Enemies;

fn main() -> Result<(), Box<dyn Error>> {
    // Setup screen
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    'outer: loop {
        let mut audio = init_sounds();

        // Create a child thread for rendering.
        let (render_tx, render_rx) = mpsc::channel();
        let render_handle = thread::spawn(move || {
            let mut last_frame = frame::new_frame();
            let mut stdout = io::stdout();

            render::render(&mut stdout, &last_frame, &last_frame, true);

            loop {
                let current_frame = match render_rx.recv() {
                    Ok(x ) => x,
                    Err(_) => break,
                };

                render::render(&mut stdout, &last_frame, &current_frame, false);

                last_frame = current_frame;
            }
        });

        let mut player = Player::new();
        let mut enemies = Enemies::new();
        let mut destroyed_count = 0;
        let mut instant = Instant::now();

        // Main loop
        'mainloop: loop {
        let d = instant.elapsed();
        instant = Instant::now();
        let mut current_frame = new_frame();

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left =>  {
                        if player.go_to_left() {
                            audio.play("move");
                        }
                    },
                    KeyCode::Right => {
                        if player.go_to_right() {
                            audio.play("move");
                        }
                    },
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'mainloop;
                    }
                    _ => {}
                }
            }
        }

        player.update(d);
        enemies.update(d);
        // Check for collisions
        let destroyed = enemies.hit_by(&mut player.shots);
        if destroyed > 0 {
            destroyed_count += destroyed;
            audio.play("explode");
        }
        // Draw everything
        player.draw(&mut current_frame);
        enemies.draw(&mut current_frame);
        // Win condition
        if enemies.all_dead() {
            audio.play("win");
            break 'mainloop;
        }
        // Lose condition
        if enemies.reached_bottom() {
            audio.play("lose");
            break 'mainloop;
        }
        // Don't do anything if an error occurred
        let _ = render_tx.send(current_frame);
        // Delay the loop to prevent too much frame per second
        thread::sleep(Duration::from_millis(1));
    }

        // Cleanup
        drop(render_tx);
        render_handle.join().unwrap();
        audio.wait();

        println!("\nGame Over!\nEnemies destroyed: {}\nTotal score: {}", destroyed_count, destroyed_count);
        println!("Press R to restart or Q to quit...");

        // Wait for user input to restart or quit
        loop {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Char('r') | KeyCode::Char('R') => {
                            break; // restart outer loop
                        }
                        KeyCode::Char('q') | KeyCode::Esc | KeyCode::Char('Q') => {
                            // Clean up terminal and exit
                            stdout.execute(Show)?;
                            stdout.execute(LeaveAlternateScreen)?;
                            terminal::disable_raw_mode()?;
                            return Ok(());
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

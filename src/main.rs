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

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = init_sounds();

    // Create a child thread for rendering.
    // It's unnecessary but will be useful in the real world apps for a better performance.
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

    // Setup screen
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    let mut player = Player::new();
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

        player.draw(&mut current_frame);

        // Don't do anything if an error occurred
        let _ = render_tx.send(current_frame);

        // Delay the loop to prevent to much frame per second
        thread::sleep(Duration::from_millis(1));
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

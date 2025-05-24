# Invaders

A Space Invaders-inspired terminal game built in Rust using the ratatui library for a rich terminal user interface.

## Features

- Modern terminal UI using ratatui
- Smooth gameplay with proper frame timing
- Sound effects
- Enemy movement patterns
- Player shooting mechanics
- Game state management (playing, game over, win conditions)
- High score tracking

## How to Build

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. Clone this repository.
3. In the project directory, run:
   ```sh
   cargo build --release
   ```

## How to Run

From the project directory, run:
```sh
cargo run --release
```

## Controls

### During Gameplay
- Left Arrow: Move left
- Right Arrow: Move right
- Space or Enter: Shoot
- Q or Esc: Quit

### After Game Over/Win
- R: Restart game
- Q or Esc: Quit

## Technical Details

This game uses several Rust crates:
- **ratatui**: Terminal UI library for rendering the game interface
- **crossterm**: Cross-platform terminal manipulation
- **rusty_audio**: Audio playback for game sounds
- **rusty_time**: Timing utilities for game loops and animations
- **anyhow**: Error handling

## Project Structure

- **app.rs**: Application state and UI rendering logic
- **main.rs**: Game loop and input handling
- **player.rs**: Player entity and movement
- **enemies.rs**: Enemy entities and movement patterns
- **laser.rs**: Projectile mechanics
- **sound.rs**: Audio management

## Credits
- Programming: Hendro Wibowo
- Sound effects: My kids (thank you for your voices!)

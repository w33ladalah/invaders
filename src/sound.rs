use rusty_audio::Audio;

pub fn init_sounds() -> Audio {
    // Setup audio
    let mut audio = Audio::new();

    audio.add("explode", "./sounds/explode.wav");
    audio.add("lose", "./sounds/lose.wav");
    audio.add("move", "./sounds/move.wav");
    audio.add("pew", "./sounds/pew.wav");
    audio.add("startup", "./sounds/startup.wav");
    audio.add("win", "./sounds/win.wav");

    audio.play("startup");

    audio
}
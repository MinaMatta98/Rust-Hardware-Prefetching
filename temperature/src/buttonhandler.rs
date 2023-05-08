use std::env;
use std::process::Command;

pub enum ButtonHandler {
    Empty(),
    NonEmpty(std::process::Child),
}

pub fn button_handler() -> ButtonHandler {
    match env::var("BLOCK_BUTTON")
        .unwrap_or_default()
        .parse::<i32>()
        .unwrap_or_default()
    {
        1 => ButtonHandler::NonEmpty(
            Command::new("alacritty")
                .arg("-e")
                .arg("zsh")
                .arg("-c")
                .arg("watch sensors")
                .spawn()
                .unwrap(),
        ),
        3 => ButtonHandler::NonEmpty(
            Command::new("alacritty")
                .arg("-e")
                .arg("nvim")
                .arg("/home/mina/Downloads/dwmscripts/Rust/temp/src/main.rs")
                .spawn()
                .unwrap(),
        ),
        _ => ButtonHandler::Empty(),
    }
}

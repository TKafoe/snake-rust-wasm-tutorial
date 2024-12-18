#[cfg(feature = "buddy-alloc")]
mod alloc;
mod game;
mod models;
mod palette;
mod renderers;
mod controllers;
mod utils;
mod wasm4;
use game::Game;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref SNAKE_GAME: Mutex<Game> = Mutex::new(Game::new());
}

#[no_mangle]
fn start() {
    palette::set_palette([
        // 0xfbf7f3, 0xe5b083, 0x426e5d, 0x20283d
        0xfff6d3, 0x7c3f58, 0xeb6b6f, 0xf9a875,
    ]);

    palette::set_draw_color(0, 0);
}

#[no_mangle]
fn update() {
    SNAKE_GAME.lock().expect("game_state").update();
}

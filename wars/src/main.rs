#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use wars::Game;

fn main() {
    let mut game = Game::new();
    game.init();

    while game.is_running() {
        game.update();
    }
}

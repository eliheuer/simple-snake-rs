mod command;
mod direction;
mod game;
mod point;
mod snake;

use crate::game::Game;
use std::io::stdout;

fn main() {
    Game::new(stdout(), 20, 20).run();
}

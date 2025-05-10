use std::io::stdin;

use board::{coordinate::Coordinate3, Board};
use game::{hand, Game};
mod board;
mod game;

fn main() {
    let mut game = Game::random(2);
    game.start();
    game.print();
}
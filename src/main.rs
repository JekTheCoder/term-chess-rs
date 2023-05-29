#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

use crate::{game::board::Board, traits::clone_as::CloneAs};

mod game;
mod traits;

fn main() {
    let board = Board::default();
    println!("{}", board.clone_as());
}

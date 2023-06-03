#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf
)]

use crate::game::board::Board;

mod game;
mod traits;
mod utils;

fn main() {
    let board = Board::default();
    println!("{}", board);
}

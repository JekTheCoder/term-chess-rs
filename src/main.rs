#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf
)]

use crate::{
    game::board::Board,
    traits::{clone_as::CloneAs, get_two_points_mut::Point},
};

mod game;
mod traits;
mod utils;

fn main() {
    let mut board = Board::default();
    println!("{}", board.clone_as());

    board.mov(Point { x: 0, y: 0 }, Point { x: 2, y: 2 });

    println!("{}", board.clone_as());
}

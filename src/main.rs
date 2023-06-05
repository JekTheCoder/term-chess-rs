#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf
)]

use std::io::stdin;

use crate::{game::board::Board, mov::Mov};

mod game;
mod mov;
mod traits;
mod utils;

fn main() {
    let mut board = Board::default();
    println!("{board}");

    let moves = stdin().lines().flatten().flat_map(|line| {
        let mov = line.parse::<Mov>();
        if mov.is_err() {
            println!("What?");
        }

        mov
    });

    for mov in moves {
        let Mov { from, to } = mov;
        if board.mov(from, to).is_ok() {
            println!("{board}");
        }
    }
}

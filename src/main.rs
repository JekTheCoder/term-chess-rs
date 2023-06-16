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

use crate::{
    game::{board::move_board, board::Board},
    mov::Mov,
};

mod arrays;
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
        if let Err(err) = board.mov(from, to) {
            match err {
                move_board::Error::InvalidMove => println!("Invalid move"),
				_ => {}
            }
        } else {
            println!("{board}");
        }
    }
}

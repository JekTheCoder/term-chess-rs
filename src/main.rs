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

use crate::{game::board::{Board, self}, mov::Mov};

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
        if let Err(err) = board.mov(&from, &to) {
            match err {
                board::mov::Error::InvalidMove => println!("Invalid move"),
                board::mov::Error::SamePoint => println!("Same point"),
                board::mov::Error::StartOutOfBounds => println!("Start out of bounds"),
                board::mov::Error::EndOutOfBounds => println!("End out of bounds"),
                board::mov::Error::CantEat => println!("Cant eat"),
                board::mov::Error::FromStartEmpty => println!("From start empty"),
            }
        } else {
            println!("{board}");
        }
    }
}

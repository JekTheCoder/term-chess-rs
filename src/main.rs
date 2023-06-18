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

use crate::{game::{board::{Board, self}, Game}, mov::Mov};

mod game;
mod mov;
mod traits;
mod utils;

fn main() {
    let mut game = Game::with_board(Board::default());
    println!("{game}");

    let moves = stdin().lines().flatten().flat_map(|line| {
        let mov = line.parse::<Mov>();
        if mov.is_err() {
            println!("What?");
        }

        mov
    });

    for mov in moves {
        let Mov { from, to } = mov;
        if let Err(err) = game.mov(&from, &to) {
            match err {
                board::mov::Error::InvalidMove => println!("Invalid move"),
                board::mov::Error::SamePoint => println!("Same point"),
                board::mov::Error::StartOutOfBounds => println!("Start out of bounds"),
                board::mov::Error::EndOutOfBounds => println!("End out of bounds"),
                board::mov::Error::CantEat => println!("Cant eat"),
                board::mov::Error::FromStartEmpty => println!("From start empty"),
                board::mov::Error::PieceNotOfTeam => println!("Piece not of team"),
            }
        } else {
            println!("{game}");
        }
    }
}

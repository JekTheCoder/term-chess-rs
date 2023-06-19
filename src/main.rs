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

use crate::{game::{board::{Board, self}, Game}, input::mov::Mov};

mod macros;
mod game;
mod mov;
mod traits;
mod utils;
mod input;

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
        let crate::mov::Mov { from, to } = mov.into_points();
        if let Err(err) = game.mov(&from, &to) {
            let message = match err {
                board::mov::Error::InvalidMove =>"Invalid move",
                board::mov::Error::SamePoint =>"Same point",
                board::mov::Error::StartOutOfBounds =>"Start out of bounds",
                board::mov::Error::EndOutOfBounds =>"End out of bounds",
                board::mov::Error::CantEat =>"Cant eat",
                board::mov::Error::FromStartEmpty =>"From start empty",
                board::mov::Error::PieceNotOfTeam =>"Piece not of team",
            };
            println!("{message}");
        } else {
            println!("{game}");
        }
    }
}

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
    game::{
        board::{self, Board},
        Game,
    },
    input::mov::Mov,
};

mod game;
mod input;
mod macros;
mod mov;
mod traits;
mod utils;

fn main() {
    let mut game = Game::with_board(Board::default());
    println!("{game}");

    let moves = stdin().lines().flatten().flat_map(|line| {
        let mov = line.parse::<Mov>();
        if let Err(mov_err) = &mov {
            match mov_err {
                input::mov::ParseError::Coord { kind, err } => {
                    let ident = match kind {
                        input::mov::Kind::From => "from",
                        input::mov::Kind::To => "to",
                    };

                    let axis = match err {
                        input::mov::coord::ParseError::Col => "col",
                        input::mov::coord::ParseError::Row => "row",
                    };

                    println!("Invalid {ident}: {axis}");
                }
                input::mov::ParseError::NotFound => println!("Second coord not found"),
            };
        }

        mov
    });

    for mov in moves {
        let crate::mov::Mov { from, to } = mov.into_points();
        match game.mov(&from, &to) {
            Ok(event) => {
                separator();
                println!("{game}");
                match event {
                    game::Event::None => {}
                    game::Event::Win(color) => {
                        println!("{color} Wins!");
                        separator();
                        break;
                    }
                    game::Event::QueenArrive(color) => {
                        println!("A {color} Queen has arrived!");
                        separator();
                    }
                }
            }
            Err(err) => {
                let message = match err {
                    board::mov::Error::InvalidMove => "Invalid move",
                    board::mov::Error::SamePoint => "Same point",
                    board::mov::Error::StartOutOfBounds => "Start out of bounds",
                    board::mov::Error::EndOutOfBounds => "End out of bounds",
                    board::mov::Error::CantEat => "Cant eat",
                    board::mov::Error::FromStartEmpty => "From start empty",
                    board::mov::Error::PieceNotOfTeam => "Piece not of team",
                };
                println!("{message}");
            }
        }
    }
}

fn separator() {
    println!("---------------------------------------------");
}

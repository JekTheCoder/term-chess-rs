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
    println!("{}", board);

    for mov in stdin().lines().flatten().flat_map(|line| {
        let mov = line.parse::<Mov>();
        if let Err(_) = &mov {
            println!("What?");
        }

        mov
    }) {
		let Mov { from, to } = mov;
		if let Ok(_) = board.mov(from, to) {
			println!("{}", board);
		}
    }
}

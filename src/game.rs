use crate::traits::get_two_points_mut::Point;

use self::{
    board::Board,
    color::Color,
    piece::{Kind, Piece},
};

pub mod board;
pub mod cell;
pub mod color;
pub mod display;
pub mod mov;
pub mod piece;
pub mod query;

pub struct Game {
    board: Board,
    turn: Color,
    local: Color,

    eaten: Eaten,
}

struct Eaten {
    local: Vec<Piece>,
    rival: Vec<Piece>,
}

pub enum Event {
    Win(Color),
    None,
}

impl Game {
    pub const fn with_board(board: Board) -> Self {
        Self {
            board,
            local: Color::White,
            turn: Color::White,
            eaten: Eaten::empty(),
        }
    }

    pub fn mov(&mut self, from: &Point, to: &Point) -> Result<Event, board::mov::Error> {
        self.board.mov(from, to, self.turn).map(|info| match info {
            board::mov::Info::Eaten(Piece {
                kind: Kind::King, ..
            }) => Event::Win(self.turn),
            board::mov::Info::Eaten(piece) => {
                self.eaten.push(self.local, piece);
                Event::None
            }
            board::mov::Info::Moved => Event::None,
        })
    }
}

impl Eaten {
    pub const fn empty() -> Self {
        Self {
            local: Vec::new(),
            rival: Vec::new(),
        }
    }

    pub fn push(&mut self, local: Color, piece: Piece) {
        let v = if piece.color == local {
            &mut self.local
        } else {
            &mut self.rival
        };
        v.push(piece);
    }
}

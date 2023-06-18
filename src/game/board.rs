mod create;
mod display;
pub mod mov;
mod piece_ray;

use crate::traits::get_two_points_mut::Point;

use self::create::{create_board, BoardArray};

use super::{color::Color, query::sign::Sign};

pub struct Board {
    board: BoardArray,
    local: Color,
}
impl Default for Board {
    fn default() -> Self {
        Self {
            board: create_board(),
            local: Color::White,
        }
    }
}

impl Board {
    pub fn mov(&mut self, from: &Point, to: &Point, turn: Color) -> Result<mov::Info, mov::Error> {
        mov::board(self, from, to, turn)
    }

    pub fn pawn_first_mov(&self, row: usize, color: Color) -> bool {
        if self.local == color {
            row == 1
        } else {
            row == 6
        }
    }

    pub fn direction_of(&self, color: Color) -> Sign {
        if self.local == color {
            Sign::Positive
        } else {
            Sign::Negative
        }
    }

    pub const fn array(&self) -> &BoardArray {
        &self.board
    }
}

#[cfg(test)]
impl Board {
    pub fn empty() -> Self {
        Self {
            board: Default::default(),
            local: Color::White,
        }
    }

    pub const fn with_array(board: BoardArray) -> Self {
        Self {
            board,
            local: Color::White,
        }
    }
}

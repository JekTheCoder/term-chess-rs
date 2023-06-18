mod create;
mod display;
pub mod mov;
mod piece_ray;

use crate::traits::get_two_points_mut::Point;

use self::create::{create_board, BoardArray};

use super::{piece::Color, query::sign::Sign};

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
    pub fn mov(&mut self, from: &Point, to: &Point) -> Result<mov::Info, mov::Error> {
        mov::board(self, from, to)
    }

    pub fn pawn_first_mov(&self, row: usize, color: &Color) -> bool {
        if self.local == *color {
            row == 1
        } else {
            row == 6
        }
    }

    pub fn direction_of(&self, color: &Color) -> Sign {
        if self.local == *color {
            Sign::Positive
        } else {
            Sign::Negative
        }
    }
}

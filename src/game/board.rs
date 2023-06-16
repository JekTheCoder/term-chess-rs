mod display;
mod piece_ray;

use crate::traits::get_two_points_mut::Point;

use self::{
    create_board::{create_board, BoardArray},
    move_board::{mov_board, Error, Info},
};

use super::{piece::Color, query::sign::Sign};

mod create_board;
pub mod move_board;

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
    pub fn mov(&mut self, from: Point, to: Point) -> Result<Info, Error> {
        mov_board(self, from, to)
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
		}
		else {
			Sign::Negative
		}
	}
}

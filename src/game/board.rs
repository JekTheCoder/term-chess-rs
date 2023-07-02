mod create;
mod display;
pub mod mov;
mod piece_ray;

use self::create::{create_board, BoardArray};

pub struct Board {
    board: BoardArray,
}
impl Default for Board {
    fn default() -> Self {
        Self {
            board: create_board(),
        }
    }
}

impl Board {
    pub const fn array(&self) -> &BoardArray {
        &self.board
    }
}

#[cfg(test)]
use crate::traits::get_two_points_mut::Point;

#[cfg(test)]
impl Board {
    pub const fn with_array(board: BoardArray) -> Self {
        Self { board }
    }

    pub fn mov(&mut self, from: &Point, to: &Point) -> Result<mov::Info, mov::Error> {
        mov::board(self, from, to)
    }
}

use crate::traits::{clone_as::CloneAs, cast::Cast, get_two_points_mut::Point};

use self::{create_board::{create_board, BoardArray}, move_board::{MoveError, mov_board}};

mod create_board;
mod move_board;

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
    pub fn mov(&mut self, from: Point, to: Point) -> Result<(), MoveError>  {
        mov_board(self, from, to)
    }
}

const BOARD_LEN: usize = 154;

impl CloneAs<String> for Board {
    fn clone_as(&self) -> String {
        let mut s = String::with_capacity(BOARD_LEN);
		
		s.push(' ');
		for i in 0..8 {
			s.push((65usize + i).cast());
		}
		s.push('\n');

        self.board.iter().enumerate().for_each(|(i, row)| {
            s.push((49usize + i).cast());
            for cell in row.iter() {
                s.push(cell.clone_as());
            }
            s.push('\n');
        });
        s
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        self.clone_as()
    }
}

mod tests {
    use super::*;

    #[test]
    fn board_len() {
        let board = Board::default();
        assert_eq!(BOARD_LEN, board.clone_as().len());
    }
}

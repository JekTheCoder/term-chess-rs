use crate::traits::clone_as::CloneAs;

use self::create_board::{create_board, BoardArray};

mod create_board;

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

const BOARD_LEN: usize = 136;

impl CloneAs<String> for Board {
    fn clone_as(&self) -> String {
        let mut s = String::with_capacity(BOARD_LEN);
        self.board.iter().for_each(|row| {
            row.iter().for_each(|cell| {
                s.push(cell.clone_as());
            });
            s.push('\n');
        });
        s
    }
}


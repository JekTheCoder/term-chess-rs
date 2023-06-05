use std::fmt::Display;

use crate::traits::{cast::Cast, clone_as::CloneAs, get_two_points_mut::Point};

use self::{
    create_board::{create_board, BoardArray},
    move_board::{mov_board, MoveError, MoveRes},
};

use super::{cell::Cell, piece::Color};

mod create_board;
mod move_board;

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
    pub fn mov(&mut self, from: Point, to: Point) -> Result<MoveRes, MoveError> {
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

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "   ")?;
        for i in 0..8 {
            let letter: char = (65usize + i).cast();
            write!(f, " {letter} ")?;
        }
        writeln!(f)?;

        for (i, row) in self.board.iter().enumerate() {
            write_row_delimiter(f)?;
            write_row_cell(f, row, i + 1)?;
        }

        write_row_delimiter(f)?;

        Ok(())
    }
}

fn write_row_delimiter(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "   +--+--+--+--+--+--+--+--+")
}

fn write_row_cell(f: &mut std::fmt::Formatter<'_>, cells: &[Cell], row: usize) -> std::fmt::Result {
    write!(f, " {row} |")?;
    for cell in cells {
        write!(
            f,
            "{} |",
            cell.piece
                .as_ref()
                .map_or(' ', |piece| -> char { piece.clone_as() })
        )?;
    }
    writeln!(f)
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn board_len() {
        let board = Board::default();
        assert_eq!(BOARD_LEN, board.clone_as().len());
    }
}

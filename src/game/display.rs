use std::fmt::Display;

use crate::traits::{cast::Cast, clone_as::CloneAs};

use super::{cell::Cell, Eaten, Game, piece::Piece};

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        dbg!(&self.eaten.local);

        write!(f, "          ")?;
        for i in 0..8 {
            let letter: char = (65usize + i).cast();
            write!(f, " {letter} ")?;
        }
        writeln!(f)?;

        for (i, row) in self.board.array().iter().enumerate() {
            write_row_delimiter(f)?;
            write_row_cell(f, &self.eaten, row, i + 1)?;
        }

        write_row_delimiter(f)?;

        Ok(())
    }
}

fn write_row_delimiter(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "+--+--+   +--+--+--+--+--+--+--+--+")
}

fn write_row_cell(
    f: &mut std::fmt::Formatter<'_>,
    eaten: &Eaten,
    cells: &[Cell],
    row: usize,
) -> std::fmt::Result {
    let Eaten { local, rival } = eaten;
    let p = row * 2;

	write!(f, "|")?;
	write_piece(f, local.get(p))?;
	write_piece(f, local.get(p + 1))?;
    write!(
        f,
        " {row} |",
    )?;


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

fn write_piece(f: &mut std::fmt::Formatter<'_>, piece: Option<&Piece>) -> std::fmt::Result {
	if let Some(piece) = piece {
		write!(f, "{}|", piece.clone_as())
	}
	else {
		write!(f, "  |")
	}
}

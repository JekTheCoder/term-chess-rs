use crate::traits::clone_as::CloneAs;

use super::piece::Piece;

#[derive(Debug, Clone, Default)]
pub struct Cell {
    pub piece: Option<Piece>,
}

impl CloneAs<char> for &Cell {
    fn clone_as(&self) -> char {
        match &self.piece {
            Some(piece) => piece.clone_as(),
            None => ' ',
        }
    }
}

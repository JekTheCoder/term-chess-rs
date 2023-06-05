use crate::{
    game::{cell::Cell, piece::Piece},
    traits::get_two_points_mut::{GetTwoPointsMut, Point},
};

use super::Board;

pub enum MoveError {
    StartOutOfBounds,
    FromStartEmpty,
    EndOutOfBounds,
    CantEat,
    SamePoint,
}

pub enum MoveRes {
    Eaten(Piece),
    Moved,
}

pub fn mov_board(board: &mut Board, from: Point, to: Point) -> Result<MoveRes, MoveError> {
    let (from, to) = board
        .board
        .get_two_points_mut(from, to)
        .ok_or(MoveError::SamePoint)?;

    let to = to.ok_or(MoveError::StartOutOfBounds)?;
    let from = from.ok_or(MoveError::EndOutOfBounds)?;

    mov_cells(from, to)
}

pub fn mov_cells(from: &mut Cell, to: &mut Cell) -> Result<MoveRes, MoveError> {
    if let Some(to_piece) = to.piece.as_mut() {
        let from_piece = from.piece.take().ok_or(MoveError::FromStartEmpty)?;
        if from_piece.can_eat(to_piece) {
            Ok(MoveRes::Eaten(std::mem::replace(to_piece, from_piece)))
        } else {
            from.piece = Some(from_piece);
            Err(MoveError::CantEat)
        }
    } else {
        std::mem::swap(from, to);
        Ok(MoveRes::Moved)
    }
}

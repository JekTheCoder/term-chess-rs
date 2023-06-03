use crate::traits::get_two_points_mut::{GetTwoPointsMut, Point};

use super::Board;

pub enum MoveError {
    StartOutOfBounds,
    FromStartEmpty,
    EndOutOfBounds,
    ToEndFilled,
    SamePoint,
}

pub fn mov_board(board: &mut Board, from: Point, to: Point) -> Result<(), MoveError> {
    let (from, to) = board
        .board
        .get_two_points_mut(from, to)
        .ok_or(MoveError::SamePoint)?;

	let to = to.ok_or(MoveError::StartOutOfBounds)?;
	let from = from.ok_or(MoveError::EndOutOfBounds)?;

    if from.piece.is_none() {
        return Err(MoveError::FromStartEmpty);
    }

    if to.piece.is_some() {
        return Err(MoveError::ToEndFilled);
    }

    std::mem::swap(from, to);

    Ok(())
}

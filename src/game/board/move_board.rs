use crate::{
    arrays::MatrixQuadSlice,
    game::{cell::Cell, mov::MoveContext, piece::Piece},
    traits::get_two_points_mut::{GetTwoPointsMut, Point},
};

use super::Board;

#[derive(Debug)]
pub enum Error {
    StartOutOfBounds,
    FromStartEmpty,
    EndOutOfBounds,
    CantEat,
    SamePoint,
    InvalidMove,
}

pub enum Info {
    Eaten(Piece),
    Moved,
}

pub fn mov_board(board: &mut Board, from_point: Point, to_point: Point) -> Result<Info, Error> {
    let mov_context = MoveContext {
        to: to_point.clone(),
        from: from_point.clone(),
        board: &board,
    };

    if let Some(piece) = board
        .board
        .get(from_point.y)
        .and_then(|arr| arr.get(from_point.x))
        .and_then(|cell| cell.piece.as_ref())
    {
        if !piece.is_valid_move(mov_context) {
            return Err(Error::InvalidMove);
        }
    }

    let (from, to) = board
        .board
        .get_two_points_mut(from_point.clone(), to_point.clone())
        .ok_or(Error::SamePoint)?;

    let to = to.ok_or(Error::StartOutOfBounds)?;
    let from = from.ok_or(Error::EndOutOfBounds)?;

    mov_cells(from, to)
}

pub fn mov_cells(from: &mut Cell, to: &mut Cell) -> Result<Info, Error> {
    if let Some(to_piece) = to.piece.as_mut() {
        let from_piece = from.piece.as_ref().ok_or(Error::FromStartEmpty)?;
        if from_piece.can_eat(to_piece) {
            let from_piece = from.piece.take().unwrap();
            Ok(Info::Eaten(std::mem::replace(to_piece, from_piece)))
        } else {
            Err(Error::CantEat)
        }
    } else {
        std::mem::swap(from, to);
        Ok(Info::Moved)
    }
}

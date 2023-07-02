use crate::{
    game::{cell::Cell, piece::Piece},
    traits::get_two_points_mut::{GetTwoPointsMut, Point},
};

use super::Board;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    StartOutOfBounds,
    FromStartEmpty,
    EndOutOfBounds,
    CantEat,
    SamePoint,
    InvalidMove,
    PieceNotOfTeam,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Info<'a> {
    pub kind: InfoKind,
    pub moved: &'a mut Piece,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InfoKind {
    Eaten(Piece),
    Moved,
}

pub fn board<'a>(
    board: &'a mut Board,
    from_point: &Point,
    to_point: &Point,
) -> Result<Info<'a>, Error> {
    let (from, to) = board
        .board
        .get_two_points_mut(from_point, to_point)
        .ok_or(Error::SamePoint)?;

    let to = to.ok_or(Error::StartOutOfBounds)?;
    let from = from.ok_or(Error::EndOutOfBounds)?;

    cells(from, to)
}

pub fn cells<'a>(from: &'a mut Cell, to: &'a mut Cell) -> Result<Info<'a>, Error> {
    let from_piece = from.piece.take().ok_or(Error::FromStartEmpty)?;
    let info = if let Some(to_piece) = to.piece.as_mut() {
        if !from_piece.can_eat(to_piece) {
            return Err(Error::CantEat);
        }

        let eaten = std::mem::replace(to_piece, from_piece);

        // It's ok to unwrap here because we know that to_point is not empty
        let moved = to.piece.as_mut().unwrap();

        Info {
            kind: InfoKind::Eaten(eaten),
            moved,
        }
    } else {
        to.piece = Some(from_piece);

        // It's ok to unwrap here because we know that to_point is not empty
        let moved = to.piece.as_mut().unwrap();

        Info {
            moved,
            kind: InfoKind::Moved,
        }
    };

    Ok(info)
}

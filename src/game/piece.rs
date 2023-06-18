mod diagonal_ray;
mod pawn_valid_mov;

use std::cmp::Ordering;

use crate::traits::{clone_as::CloneAs, get_two_points_mut::Point, is_some_and::IsSomeAnd};

use self::{diagonal_ray::diagonal_ray, pawn_valid_mov::pawn_is_valid_move};

use super::{
    board::Board,
    mov::MoveContext,
    query::{
        ray2d::{Ray2D, RayQuery},
        sign::Sign,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    pub color: Color,
    pub kind: Kind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl Piece {
    pub const fn new(color: Color, kind: Kind) -> Self {
        Self { color, kind }
    }

    pub fn can_eat(&self, other: &Self) -> bool {
        self.color != other.color
    }

    pub fn is_valid_move(&self, mov: MoveContext<'_>) -> bool {
        match self.kind {
            Kind::King => king_is_valid_move(&mov.from, &mov.to),
            Kind::Queen => queen_is_valid_move(mov.board, mov.from, &mov.to),
            Kind::Rook => rook_is_valid_move(mov.board, mov.from, &mov.to),
            Kind::Bishop => bishop_is_valid_move(mov.board, &mov.from, &mov.to),
            Kind::Knight => knight_is_valid_move(&mov.from, &mov.to),
            Kind::Pawn => pawn_is_valid_move(mov, &self.color),
        }
    }
}

impl CloneAs<char> for Piece {
    fn clone_as(&self) -> char {
        match (&self.color, &self.kind) {
            (Color::White, Kind::King) => '♔',
            (Color::White, Kind::Queen) => '♕',
            (Color::White, Kind::Rook) => '♖',
            (Color::White, Kind::Bishop) => '♗',
            (Color::White, Kind::Knight) => '♘',
            (Color::White, Kind::Pawn) => '♙',

            (Color::Black, Kind::King) => '♚',
            (Color::Black, Kind::Queen) => '♛',
            (Color::Black, Kind::Rook) => '♜',
            (Color::Black, Kind::Bishop) => '♝',
            (Color::Black, Kind::Knight) => '♞',
            (Color::Black, Kind::Pawn) => '♟',
        }
    }
}

macro_rules! or_return {
    ($x:expr, $y:expr) => {{
        if let Some(x) = $x {
            x
        } else {
            return $y;
        }
    }};
}

const fn king_is_valid_move(from: &Point, to: &Point) -> bool {
    from.x.abs_diff(to.x) == 1 && from.y.abs_diff(to.y) == 1
}

fn bishop_is_valid_move(board: &Board, from: &Point, to: &Point) -> bool {
    let ray = or_return!(diagonal_ray(from, to), false);

    board
        .ray2d(from.clone(), ray)
        .map_or(true, |col| col.point == *to)
}

const fn knight_is_valid_move(from: &Point, to: &Point) -> bool {
    from.x.abs_diff(to.x) == 2 && from.y.abs_diff(to.y) == 1
}

fn queen_is_valid_move(board: &Board, from: Point, to: &Point) -> bool {
    bishop_is_valid_move(board, &from, to) || rook_is_valid_move(board, from, to)
}

fn rook_is_valid_move(board: &Board, from: Point, to: &Point) -> bool {
    if from.x == to.x {
        let dir = match from.y.cmp(&to.y) {
            Ordering::Greater => Sign::Positive,
            _ => Sign::Negative,
        };

        let ray = Ray2D::new(Sign::Zero, dir);

        return board
            .ray2d(from, ray)
            .prev_is_some_and(|coll| coll.point.y == to.y);
    }

    from.y == to.y && {
        let dir = match from.x.cmp(&to.x) {
            Ordering::Greater => Sign::Positive,
            _ => Sign::Negative,
        };

        let ray = Ray2D::new(dir, Sign::Zero);
        board
            .ray2d(from, ray)
            .prev_is_some_and(|coll| coll.point.x == to.x)
    }
}

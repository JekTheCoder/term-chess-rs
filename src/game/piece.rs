use std::cmp::Ordering;

use crate::traits::{clone_as::CloneAs, get_two_points_mut::Point};

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
            Kind::King => king_is_valid_move(mov.from, mov.to),
            Kind::Queen => queen_is_valid_move(mov.board, mov.from, mov.to),
            Kind::Rook => rook_is_valid_move(mov.board, mov.from, mov.to),
            Kind::Bishop => bishop_is_valid_move(mov.from, mov.to),
            Kind::Knight => knight_is_valid_move(mov.from, mov.to),
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

fn king_is_valid_move(from: Point, to: Point) -> bool {
    from.x.abs_diff(to.x) == 1 && from.y.abs_diff(to.y) == 1
}

fn bishop_is_valid_move(from: Point, to: Point) -> bool {
    from.x.abs_diff(to.x) == from.y.abs_diff(to.y)
}

fn knight_is_valid_move(from: Point, to: Point) -> bool {
    from.x.abs_diff(to.x) == 2 && from.y.abs_diff(to.y) == 1
}

macro_rules! has_to {
    ($x:expr) => {
        if (!$x) {
            return false;
        }
    };
}

fn pawn_is_valid_move(context: MoveContext<'_>, color: &Color) -> bool {
    let MoveContext { board, from, to } = context;

    let vertical = board.direction_of(color);
    dbg!(&vertical);
    if vertical == Sign::Positive {
        has_to!(to.y > from.y);
    }
    if vertical == Sign::Negative {
        has_to!(to.y < from.y);
    }

    has_to!(from.x == to.x || from.x + 1 == to.x || to.x + 1 == from.x);

    dbg!(&from);

    match to.x.cmp(&from.x) {
        Ordering::Greater => board
            .ray2d_limit(from, Ray2D::new(Sign::Positive, vertical), 1)
            .is_some(),
        Ordering::Less => board
            .ray2d_limit(from, Ray2D::new(Sign::Negative, vertical), 1)
            .is_some(),
        Ordering::Equal => board
            .ray2d_limit(from, Ray2D::new(Sign::Zero, vertical), 1)
            .is_none(),
    }
}

fn queen_is_valid_move(board: &Board, from: Point, to: Point) -> bool {
    rook_is_valid_move(board, from.clone(), to.clone()) || bishop_is_valid_move(from, to)
}

fn rook_is_valid_move(board: &Board, from: Point, to: Point) -> bool {
    if from.x == to.x {
        let dir = match from.y.cmp(&to.y) {
            Ordering::Greater => Sign::Positive,
            _ => Sign::Negative,
        };

        let ray = Ray2D::new(Sign::Zero, dir);
        return match board.ray2d(from.clone(), ray) {
            Some(coll) if coll.point.y == to.y => true,
            _ => false,
        };
    }
    if from.y == to.y {
        let dir = match from.x.cmp(&to.x) {
            Ordering::Greater => Sign::Positive,
            _ => Sign::Negative,
        };

        let ray = Ray2D::new(dir, Sign::Zero);
        return match board.ray2d(from.clone(), ray) {
            Some(coll) if coll.point.x == to.x => true,
            _ => false,
        };
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pawn_validates_direction() {
        let board = Board::default();
        let piece = Piece::new(Color::White, Kind::Pawn);
        let valid_mov = piece.is_valid_move(MoveContext {
            board: &board,
            to: Point { x: 0, y: 2 },
            from: Point { x: 0, y: 1 },
        });
        let invalid_mov = piece.is_valid_move(MoveContext {
            board: &board,
            to: Point { x: 4, y: 3 },
            from: Point { x: 0, y: 1 },
        });

        assert!(valid_mov);
        assert!(!invalid_mov);
    }
}

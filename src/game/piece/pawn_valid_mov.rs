use std::{cmp::Ordering, ops::RangeInclusive};

use crate::game::{
    mov::MoveContext,
    query::{
        ray2d::{Ray2D, RayQuery},
        sign::Sign,
    },
};

use super::Color;

macro_rules! has_to {
    ($x:expr) => {
        if (!$x) {
            return false;
        }
    };
}

pub fn pawn_is_valid_move(context: MoveContext<'_>, color: &Color) -> bool {
    let MoveContext { board, from, to } = context;

    let vertical = board.direction_of(color);

    has_to!(from.x == to.x || from.x + 1 == to.x || to.x + 1 == from.x);

    match to.x.cmp(&from.x) {
        Ordering::Greater => board
            .ray2d_limit(from, Ray2D::new(Sign::Positive, vertical), 1)
            .is_some(),
        Ordering::Less => board
            .ray2d_limit(from, Ray2D::new(Sign::Negative, vertical), 1)
            .is_some(),
        Ordering::Equal => {
            let mov_limit = usize::from(board.pawn_first_mov(from.y, color)) + 1;
            let from_y = from.y;

            board
                .ray2d_limit(from, Ray2D::new(Sign::Zero, vertical), mov_limit)
                .is_none()
                && vertical_range(vertical, from_y, mov_limit).contains(&to.y)
        }
    }
}

fn vertical_range(sign: Sign, from_y: usize, mov_limit: usize) -> RangeInclusive<usize> {
    if sign == Sign::Positive {
        from_y + 1..=from_y + mov_limit
    } else {
        let start = from_y.checked_sub(mov_limit);
        let end = from_y.checked_sub(1);

        if let Some((start, end)) = start.zip(end) {
            start..=end
        } else {
            #[allow(clippy::reversed_empty_ranges)]
            let range = 1..=0;
            range
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        game::{
            board::Board,
            piece::{Kind, Piece},
        },
        traits::get_two_points_mut::Point,
    };

    #[test]
    fn validates_direction() {
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

    #[test]
    fn can_mov_non_local() {
        let board = Board::default();
        let piece = Piece::new(Color::Black, Kind::Pawn);

        assert!(piece.is_valid_move(MoveContext {
            board: &board,
            from: Point { x: 0, y: 6 },
            to: Point { x: 0, y: 5 },
        }));
    }

    #[test]
    fn range_not_includes_start() {
        let range = vertical_range(Sign::Positive, 0, 1);
        assert!(!range.contains(&0));
    }

    #[test]
    fn range_includes_positive() {
        let range = vertical_range(Sign::Positive, 0, 1);
        assert!(range.contains(&1));
    }

    #[test]
    fn range_includes_positive_with_offset() {
        let range = vertical_range(Sign::Positive, 0, 2);
        assert!(range.contains(&2));
    }

    #[test]
    fn range_includes_negative() {
        let range = vertical_range(Sign::Negative, 6, 1);
        assert!(range.contains(&5));
    }

    #[test]
    fn range_includes_negative_with_offset() {
        let range = vertical_range(Sign::Negative, 4, 2);

        dbg!(&range);
        assert!(range.contains(&2));
    }
}

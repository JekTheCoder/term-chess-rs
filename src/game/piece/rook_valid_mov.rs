use std::cmp::Ordering;

use crate::{
    game::{
        board::Board,
        query::{
            ray2d::{Ray2D, RayQuery},
            sign::Sign,
        },
    },
    traits::get_two_points_mut::Point,
};

pub fn rook_is_valid_move(board: &Board, from: Point, to: &Point) -> bool {
    let (ray, get_axis, coord) = if from.x == to.x {
        let dir = match to.y.cmp(&from.y) {
            Ordering::Greater => Sign::Positive,
            _ => Sign::Negative,
        };

        let ray = Ray2D::new(Sign::Zero, dir);
        let f: fn(&Point) -> usize = |point: &Point| point.y;

        (ray, f, to.y)
    } else if from.y == to.y {
        let dir = match to.x.cmp(&from.x) {
            Ordering::Greater => Sign::Positive,
            _ => Sign::Negative,
        };

        let ray = Ray2D::new(dir, Sign::Zero);
        let f: fn(&Point) -> usize = |point: &Point| point.x;

        (ray, f, to.x)
    } else {
        return false;
    };

    board
        .ray2d(from, ray)
        .map_or(true, |col| (get_axis)(&col.point) == coord)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        game::{
            board::mov,
            color::Color,
            piece::{Kind, Piece}, Game,
        },
        macros::board,
    };

    #[test]
    fn rook_can_eat() {
        let board = board!(
            ['R', ..];
            empty;
            empty;
            empty;
            empty;
            empty;
            empty;
            ['r', ..];
        );
        let mut board = Board::with_array(board);
        let res = board.mov(&Point { x: 0, y: 0 }, &Point { x: 0, y: 7 });

        assert_eq!(
            res.map(|res| res.kind),
            Ok(mov::InfoKind::Eaten(Piece {
                kind: Kind::Rook,
                color: Color::Black
            }))
        );
    }

    #[test]
    fn do_not_surpass() {
        let board = board!(
            ['R', ..];
            ['P', ..];
            empty;
            empty;
            empty;
            empty;
            empty;
            ['q', ..];
        );
        let board = Board::with_array(board);
        let mut game = Game::with_board(board);
        let res = game.mov(&Point { x: 0, y: 0 }, &Point { x: 0, y: 7 });

        assert_eq!(res, Err(mov::Error::InvalidMove));
    }
}

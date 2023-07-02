use crate::{
    game::{
        cell::Cell,
        piece::Piece,
        query::{
            grid_2d::Grid2D,
            ray2d::{Collision, Ray2D, RayQuery},
            step2d::{Info, IteratorFabric},
        },
    },
    traits::get_two_points_mut::Point,
};

use super::Board;

impl Grid2D for Board {
    type Item = Cell;

    fn get(&self, x: usize, y: usize) -> Option<&Self::Item> {
        self.board.get(y).and_then(|row| row.get(x))
    }
}

impl<'a> RayQuery<'a> for Board {
    type Item = &'a Piece;

    fn ray2d(&'a self, from: Point, ray: Ray2D) -> Option<Collision<Self::Item>> {
        let it = self.step_ray2d(from, ray);
        find_piece(it)
    }

    fn ray2d_limit(
        &'a self,
        from: Point,
        ray: Ray2D,
        limit: usize,
    ) -> Option<Collision<Self::Item>> {
        let it = self.step_ray2d(from, ray).take(limit);
        find_piece(it)
    }
}

fn find_piece<'a>(mut cells: impl Iterator<Item = Info<&'a Cell>>) -> Option<Collision<&'a Piece>> {
    cells.find_map(|cell| {
        cell.value.piece.as_ref().map(|piece| Collision {
            point: cell.point,
            value: piece,
        })
    })
}

#[cfg(test)]
mod tests {
    use crate::game::{color::Color, piece::Kind, query::sign::Sign};

    use super::*;

    #[test]
    fn ray_finds() {
        let board = Board::default();
        let col = board.ray2d(Point { x: 0, y: 3 }, Ray2D::new(Sign::Zero, Sign::Positive));

        assert_eq!(
            col,
            Some(Collision {
                point: Point { x: 0, y: 6 },
                value: &Piece::new(Color::Black, Kind::Pawn),
            })
        );
    }

    #[test]
    fn ray_limit_finds() {
        let board = Board::default();
        let col = board.ray2d_limit(
            Point { x: 0, y: 3 },
            Ray2D::new(Sign::Zero, Sign::Positive),
            1,
        );

        assert_eq!(col, None);
    }
}

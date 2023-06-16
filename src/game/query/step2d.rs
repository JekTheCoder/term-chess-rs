use super::{grid_2d::Grid2D, ray2d::Ray2D};
use crate::traits::get_two_points_mut::Point;

use std::iter::Iterator as StdIterator;

pub struct Iterator<T> {
    grid: T,
    point: Point,
    ray: Ray2D,
}

pub trait IteratorFabric<'a>
where
    Self: Sized,
{
    fn step_ray2d(&'a self, from: Point, ray: Ray2D) -> self::Iterator<&'a Self>;
}

pub struct Info<T> {
    pub point: Point,
    pub value: T,
}

impl<'a, G: Grid2D> IteratorFabric<'a> for G {
    fn step_ray2d(&'a self, from: Point, ray: Ray2D) -> self::Iterator<&'a G> {
        Iterator {
            ray,
            point: from,
            grid: self,
        }
    }
}

impl<'a, G: Grid2D> StdIterator for Iterator<&'a G> {
    type Item = Info<&'a G::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let Self { ray, .. } = &self;
        let (x_mut, y_mut) = ray.clone().values();

        self.point.x = self.point.x.checked_add_signed(x_mut)?;
        self.point.y = self.point.y.checked_add_signed(y_mut)?;

        let Point { x, y } = self.point;

        let res = self.grid.get(x, y).map(|piece| Info {
            point: Point {
                x: self.point.x,
                y: self.point.y,
            },
            value: piece,
        });

        res
    }
}

use crate::traits::get_two_points_mut::Point;

use super::sign::Sign;

#[derive(Clone)]
pub struct Ray2D {
    pub horizontal: Sign,
    pub vertical: Sign,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Collision<T> {
    pub point: Point,
    pub value: T,
}

impl Ray2D {
    pub const fn new(horizontal: Sign, vertical: Sign) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }

    pub const fn values(self) -> (isize, isize) {
        (self.horizontal.into_isize(), self.vertical.into_isize())
    }
}

impl From<(Sign, Sign)> for Ray2D {
    fn from((horizontal, vertical): (Sign, Sign)) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }
}

pub trait RayQuery<'a> {
    type Item;
    fn ray2d(&'a self, from: Point, ray: Ray2D) -> Option<Collision<Self::Item>>;
    fn ray2d_limit(&'a self, from: Point, ray: Ray2D, limit: usize) -> Option<Collision<Self::Item>>;
}

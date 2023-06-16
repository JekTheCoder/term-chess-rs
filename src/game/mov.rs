use crate::traits::get_two_points_mut::Point;

use super::board::Board;

pub struct MoveContext<'a> {
    pub board: &'a Board,
    pub from: Point,
    pub to: Point,
}

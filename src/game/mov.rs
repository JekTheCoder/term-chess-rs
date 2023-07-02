use crate::traits::get_two_points_mut::Point;

use super::Game;

pub struct MoveContext<'a> {
    pub game: &'a Game,
    pub from: Point,
    pub to: Point,
}

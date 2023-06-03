use crate::traits::get_both_mut::MayBoth;

use super::{get_both_mut::GetBothMut, get_two_mut::GetTwoMut};

pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
	pub fn new(x: usize, y: usize) -> Self {
		Self { x, y }
	}
}

pub trait GetTwoPointsMut<T> {
    fn get_two_points_mut(
        &mut self,
        first: Point,
        second: Point,
    ) -> Option<(Option<&mut T>, Option<&mut T>)>;
}

impl<T, const N: usize> GetTwoPointsMut<T> for [[T; N]] {
    fn get_two_points_mut(
        &mut self,
        first: Point,
        second: Point,
    ) -> Option<(Option<&mut T>, Option<&mut T>)> {
        match self.get_both_mut(first.y, second.y) {
            Some(MayBoth::Two(a, b)) => Some((a.get_mut(first.x), b.get_mut(second.x))),
            Some(MayBoth::One(a)) => a.get_two_mut(first.y, second.y),
			None => Some((None, None)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_separated_points() {
        let mut table = [[1, 2, 3], [4, 5, 6]];
        let (first, second) = table
            .get_two_points_mut(Point { x: 0, y: 0 }, Point { x: 1, y: 1 })
            .unwrap();

        assert_eq!(1, *first.unwrap());
        assert_eq!(5, *second.unwrap());
    }

    #[test]
    fn get_points_in_same_row() {
        let mut table = [[1, 2, 3], [4, 5, 6]];
        let (first, second) = table
            .get_two_points_mut(Point { x: 0, y: 0 }, Point { x: 0, y: 1 })
            .unwrap();

        assert_eq!(1, *first.unwrap());
        assert_eq!(2, *second.unwrap());
    }

	#[test]
	fn get_points_in_same_column() {
		let mut table = [[1, 2, 3], [4, 5, 6]];
		let (first, second) = table
			.get_two_points_mut(Point { x: 0, y: 0 }, Point { x: 1, y: 0 })
			.unwrap();

		assert_eq!(1, *first.unwrap());
		assert_eq!(4, *second.unwrap());
	}

	#[test]
	fn empty_on_same_point() {
		let mut table = [[1, 2, 3], [4, 5, 6]];
		let res = table
			.get_two_points_mut(Point { x: 0, y: 0 }, Point { x: 0, y: 0 });

		assert!(res.is_none());	
	}
	
	#[test]
	fn empty_out_of_bounds() {
		let mut table = [[1, 2, 3], [4, 5, 6]];
		let (first, second) = table
			.get_two_points_mut(Point { x: 2, y: 0 }, Point { x: 0, y: 0 }).unwrap();
			
		assert!(first.is_none());
		assert!(second.is_none());
	}
}

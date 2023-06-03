use std::cmp::Ordering;

use crate::utils::ord;

pub trait GetTwoMut<T> {
    fn get_two_mut(&mut self, i: usize, j: usize) -> Option<(Option<&mut T>, Option<&mut T>)>;
}

impl<T> GetTwoMut<T> for [T] {
    fn get_two_mut(&mut self, i: usize, j: usize) -> Option<(Option<&mut T>, Option<&mut T>)> {
        let (min, max, ordering) = ord(i, j);
        if max > self.len() || i == j {
            return None;
        }

        let (first, rest) = self.split_at_mut(max);

		match (ordering, first.get_mut(min), rest.get_mut(0)) {
			(Ordering::Less, first, second) => Some((first, second)),
			(Ordering::Greater, first, second) => Some((second, first)),
			_ => None,
		}
    }
}


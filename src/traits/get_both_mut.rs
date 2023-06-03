use std::cmp::Ordering;

use crate::utils::ord;

pub enum MayBoth<T> {
    One(T),
    Two(T, T),
}

pub trait GetBothMut<T> {
    fn get_both_mut(&mut self, i: usize, j: usize) -> Option<MayBoth<&mut T>>;
}

impl<T> GetBothMut<T> for [T] {
    fn get_both_mut(&mut self, i: usize, j: usize) -> Option<MayBoth<&mut T>> {
        let (min, max, ordering) = ord(i, j);
        if max > self.len() {
            return None;
        }

        let (first, rest) = self.split_at_mut(max);

        match (ordering, first.get_mut(min), rest.get_mut(0)) {
            (Ordering::Less, Some(first), Some(second)) => Some(MayBoth::Two(first, second)),
            (Ordering::Greater, Some(first), Some(second)) => Some(MayBoth::Two(second, first)),
            (Ordering::Equal, _, Some(second)) => Some(MayBoth::One(second)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut slice = [1, 2, 3, 4, 5];
        let (a, b) = match slice.get_both_mut(1, 3).unwrap() {
            MayBoth::Two(a, b) => (a, b),
            _ => unreachable!(),
        };

        *a = 10;
        *b = 20;

        assert_eq!(slice[1], 10);
        assert_eq!(slice[3], 20);
    }

    #[test]
    fn works_reverse() {
        let mut slice = ['a', 'b', 'c', 'd', 'e'];
        let (a, b) = match slice.get_both_mut(4, 2).unwrap() {
            MayBoth::Two(a, b) => (a, b),
            _ => unreachable!(),
        };

        *a = 'z';
        *b = 'y';

        assert_eq!(slice[4], 'z');
        assert_eq!(slice[2], 'y');
    }

    #[test]
    fn can_mut_one() {
        let mut slice = ['a', 'b', 'c', 'd', 'e'];
        let a = match slice.get_both_mut(4, 4).unwrap() {
            MayBoth::One(a) => a,
            _ => unreachable!(),
        };

        *a = 'z';
        assert_eq!(slice[4], 'z');
    }
}

use core::num;
use std::cmp::Ordering;

pub fn ord<T: Ord>(n1: T, n2: T) -> (T, T, Ordering) {
    let ordering = n1.cmp(&n2);
    match &ordering {
        Ordering::Greater => (n2, n1, Ordering::Greater),
        Ordering::Less => (n1, n2, Ordering::Less),
        Ordering::Equal => (n1, n2, Ordering::Equal),
    }
}


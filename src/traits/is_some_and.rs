pub trait IsSomeAnd<T> {
    fn prev_is_some_and<F: Fn(T) -> bool>(self, f: F) -> bool;
}

impl<T> IsSomeAnd<T> for Option<T> {
    fn prev_is_some_and<F: Fn(T) -> bool>(self, f: F) -> bool {
        self.map_or(false, f) 
    }
}

pub trait Grid2D {
    type Item;
    fn get(&self, x: usize, y: usize) -> Option<&Self::Item>;
}

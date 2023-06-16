#[derive(Debug, Clone, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl PartialEq<Self> for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}


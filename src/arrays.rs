pub struct MatrixQuadSlice<'a, T> {
    matrix: &'a [&'a [T]],
}

impl<'a, T> MatrixQuadSlice<'a, T> {
    pub fn get(&self, x: usize, y: usize) -> Option<&'a T> {
        self.matrix.get(x)?.get(y)
    }

    pub fn row(&self, y: usize) -> Option<&'a [T]> {
        self.matrix.get(y).map(|row| *row)
    }

    pub const fn new(matrix: &'a [&'a [T]]) -> Self {
        Self { matrix }
    }
}


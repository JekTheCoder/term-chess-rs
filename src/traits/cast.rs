// unsafe code to cast a type to another
// It would panic if the cast fails
// Use with precaution
pub trait Cast<T> {
    fn cast(self) -> T;
}

impl Cast<u8> for usize {
    fn cast(self) -> u8 {
        self.try_into().unwrap()
    }
}

impl Cast<char> for usize {
    fn cast(self) -> char {
        Cast::<u8>::cast(self) as char
    }
}

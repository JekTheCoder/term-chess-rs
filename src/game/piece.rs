use crate::traits::clone_as::CloneAs;

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone)]
pub enum Piece {
    King(Color),
    Queen(Color),
    Rook(Color),
    Bishop(Color),
    Knight(Color),
    Pawn(Color),
}

impl CloneAs<char> for Piece {
    fn clone_as(&self) -> char {
        match self {
            Self::King(Color::White) => '♔',
            Self::Queen(Color::White) => '♕',
            Self::Rook(Color::White) => '♖',
            Self::Bishop(Color::White) => '♗',
            Self::Knight(Color::White) => '♘',
            Self::Pawn(Color::White) => '♙',

            Self::King(Color::Black) => '♚',
            Self::Queen(Color::Black) => '♛',
            Self::Rook(Color::Black) => '♜',
            Self::Bishop(Color::Black) => '♝',
            Self::Knight(Color::Black) => '♞',
            Self::Pawn(Color::Black) => '♟',
        }
    }
}


impl Piece {
    pub fn color(&self) -> &Color {
        match self {
            Self::King(c) => c,
            Self::Queen(c) => c,
            Self::Rook(c) => c,
            Self::Bishop(c) => c,
            Self::Knight(c) => c,
            Self::Pawn(c) => c,
        }
    }

    pub fn can_eat(&self, other: &Piece) -> bool {
        self.color() != other.color()
    }
}

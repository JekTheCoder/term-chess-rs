use crate::traits::clone_as::CloneAs;

#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub const fn color(&self) -> &Color {
        match self {
            Self::King(c)
            | Self::Queen(c)
            | Self::Rook(c)
            | Self::Bishop(c)
            | Self::Knight(c)
            | Self::Pawn(c) => c,
        }
    }

    pub fn can_eat(&self, other: &Self) -> bool {
        self.color() != other.color()
    }
}

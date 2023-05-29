use crate::traits::clone_as::CloneAs;

#[derive(Debug, Clone)]
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
            Piece::King(Color::White) => '♔',
            Piece::Queen(Color::White) => '♕',
            Piece::Rook(Color::White) => '♖',
            Piece::Bishop(Color::White) => '♗',
            Piece::Knight(Color::White) => '♘',
            Piece::Pawn(Color::White) => '♙',

            Piece::King(Color::Black) => '♚',
            Piece::Queen(Color::Black) => '♛',
            Piece::Rook(Color::Black) => '♜',
            Piece::Bishop(Color::Black) => '♝',
            Piece::Knight(Color::Black) => '♞',
            Piece::Pawn(Color::Black) => '♟',
        }
    }
}


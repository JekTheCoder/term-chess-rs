use crate::game::{
    cell::Cell,
    piece::{Color, Piece},
};

pub type BoardArray = [[Cell; 8]; 8];

pub fn create_board() -> BoardArray {
    let white_back_row: [Cell; 8] = [
        Cell {
            piece: Some(Piece::Rook(Color::White)),
        },
        Cell {
            piece: Some(Piece::Knight(Color::White)),
        },
        Cell {
            piece: Some(Piece::Bishop(Color::White)),
        },
        Cell {
            piece: Some(Piece::Queen(Color::White)),
        },
        Cell {
            piece: Some(Piece::King(Color::White)),
        },
        Cell {
            piece: Some(Piece::Bishop(Color::White)),
        },
        Cell {
            piece: Some(Piece::Knight(Color::White)),
        },
        Cell {
            piece: Some(Piece::Rook(Color::White)),
        },
    ];

    let white_pawns: [Cell; 8] = [
        Cell {
            piece: Some(Piece::Pawn(Color::White)),
        },
        Cell {
            piece: Some(Piece::Pawn(Color::White)),
        },
        Cell {
            piece: Some(Piece::Pawn(Color::White)),
        },
        Cell {
            piece: Some(Piece::Pawn(Color::White)),
        },
        Cell {
            piece: Some(Piece::Pawn(Color::White)),
        },
        Cell {
            piece: Some(Piece::Pawn(Color::White)),
        },
        Cell {
            piece: Some(Piece::Pawn(Color::White)),
        },
        Cell {
            piece: Some(Piece::Pawn(Color::White)),
        },
    ];

    let mut board: BoardArray = Default::default();

    board[0] = white_back_row;
    board[1] = white_pawns;

    // Set up black pieces on the last two rows
    board[7][0].piece = Some(Piece::Rook(Color::Black));
    board[7][1].piece = Some(Piece::Knight(Color::Black));
    board[7][2].piece = Some(Piece::Bishop(Color::Black));
    board[7][3].piece = Some(Piece::Queen(Color::Black));
    board[7][4].piece = Some(Piece::King(Color::Black));
    board[7][5].piece = Some(Piece::Bishop(Color::Black));
    board[7][6].piece = Some(Piece::Knight(Color::Black));
    board[7][7].piece = Some(Piece::Rook(Color::Black));

    for i in 0..8 {
        board[6][i].piece = Some(Piece::Pawn(Color::Black));
    }

    board
}

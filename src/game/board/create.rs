use crate::game::{
    cell::Cell,
    piece::{Kind, Piece},
    color::Color,
};

pub type BoardArray = [[Cell; 8]; 8];

#[allow(clippy::module_name_repetitions)]
pub fn create_board() -> BoardArray {
    let white_back_row: [Cell; 8] = [
        Cell {
            piece: Some(Piece::new(Color::White, Kind::Rook)),
        },
        Cell {
            piece: Some(Piece::new(Color::White, Kind::Knight)),
        },
        Cell {
            piece: Some(Piece::new(Color::White, Kind::Bishop)),
        },
        Cell {
            piece: Some(Piece::new(Color::White, Kind::Queen)),
        },
        Cell {
            piece: Some(Piece::new(Color::White, Kind::King)),
        },
        Cell {
            piece: Some(Piece::new(Color::White, Kind::Bishop)),
        },
        Cell {
            piece: Some(Piece::new(Color::White, Kind::Knight)),
        },
        Cell {
            piece: Some(Piece::new(Color::White, Kind::Rook)),
        },
    ];

    let white_pawns: [Cell; 8] = [
        Cell {
            piece: Some(Piece::new(Color::White, Kind::Pawn)),
        },
        Cell {
            piece: Some(Piece::new(Color::White, Kind::Pawn)),
        },
        Cell {
            piece: Some(Piece::new(Color::White, Kind::Pawn)),
        },
        Cell {
            piece: Some(Piece::new(Color::White, Kind::Pawn)),
        },
        Cell {
            piece: Some(Piece::new(Color::White, Kind::Pawn)),
        },
        Cell {
            piece: Some(Piece::new(Color::White, Kind::Pawn)),
        },
        Cell {
            piece: Some(Piece::new(Color::White, Kind::Pawn)),
        },
        Cell {
            piece: Some(Piece::new(Color::White, Kind::Pawn)),
        },
    ];

    let mut board: BoardArray = Default::default();

    board[0] = white_back_row;
    board[1] = white_pawns;

    // Set up black pieces on the last two rows
    board[7][0].piece = Some(Piece::new(Color::Black, Kind::Rook));
    board[7][1].piece = Some(Piece::new(Color::Black, Kind::Knight));
    board[7][2].piece = Some(Piece::new(Color::Black, Kind::Bishop));
    board[7][3].piece = Some(Piece::new(Color::Black, Kind::Queen));
    board[7][4].piece = Some(Piece::new(Color::Black, Kind::King));
    board[7][5].piece = Some(Piece::new(Color::Black, Kind::Bishop));
    board[7][6].piece = Some(Piece::new(Color::Black, Kind::Knight));
    board[7][7].piece = Some(Piece::new(Color::Black, Kind::Rook));

    for i in 0..8 {
        board[6][i].piece = Some(Piece::new(Color::Black, Kind::Pawn));
    }

    board
}

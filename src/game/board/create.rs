use crate::{
    game::{
        cell::Cell,
        color::Color,
        piece::{Kind, Piece},
    },
    macros::board,
};

pub type BoardArray = [[Cell; 8]; 8];

#[allow(clippy::module_name_repetitions)]
pub const fn create_board() -> BoardArray {
	board! {
		['♖', '♘', '♗', '♕', '♔', '♗', '♘', '♖'];
		['♙', '♙', '♙', '♙', '♙', '♙', '♙', '♙'];
		empty;
		empty;
		empty;
		empty;
		['♟', '♟', '♟', '♟', '♟', '♟', '♟', '♟'];
		['♜', '♞', '♝', '♛', '♚', '♝', '♞', '♜'];
	}
}

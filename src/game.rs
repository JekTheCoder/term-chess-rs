use crate::traits::get_two_points_mut::Point;

use self::{
    board::Board,
    color::Color,
    piece::{Kind, Piece},
};

pub mod board;
pub mod cell;
pub mod color;
pub mod display;
pub mod mov;
pub mod piece;
pub mod query;

pub struct Game {
    board: Board,
    turn: Color,
    local: Color,

    eaten: Eaten,
}

pub struct Eaten {
    local: Vec<Piece>,
    rival: Vec<Piece>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Win(Color),
    None,
}

impl Game {
    pub const fn new(board: Board, local: Color, turn: Color, eaten: Eaten) -> Self {
        Self {
            board,
            local,
            turn,
            eaten,
        }
    }

    pub const fn with_board(board: Board) -> Self {
        Self {
            board,
            local: Color::White,
            turn: Color::White,
            eaten: Eaten::empty(),
        }
    }

    pub fn mov(&mut self, from: &Point, to: &Point) -> Result<Event, board::mov::Error> {
        self.board.mov(from, to, self.turn).map(|info| {
            let event = match info {
                board::mov::Info::Eaten(Piece {
                    kind: Kind::King, ..
                }) => Event::Win(self.turn),
                board::mov::Info::Eaten(piece) => {
                    self.eaten.push(self.local, piece);
                    Event::None
                }
                board::mov::Info::Moved => Event::None,
            };

            self.turn = self.turn.contrary();
            event
        })
    }
}

impl Eaten {
    pub const fn empty() -> Self {
        Self {
            local: Vec::new(),
            rival: Vec::new(),
        }
    }

    pub fn push(&mut self, local: Color, eaten: Piece) {
        let v = if eaten.color == local {
            &mut self.local
        } else {
            &mut self.rival
        };
        v.push(eaten);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::board;

    #[test]
    fn eat_ads_to_rival() {
        let board = board!(
           ['R', ..];
           empty;
           empty;
           empty;
           empty;
           empty;
           empty;
           ['r', ..];
        );
        let board = Board::with_array(board);
        let mut game = Game::with_board(board);
        let ev = game
            .mov(&Point { x: 0, y: 0 }, &Point { x: 0, y: 7 })
            .unwrap();

        let may_eaten = Piece {
            kind: Kind::Rook,
            color: Color::Black,
        };

        assert_eq!(ev, Event::None);
        assert_eq!(game.eaten.rival, vec![may_eaten]);
    }

    #[test]
    fn eat_do_not_ads_to_local() {
        let board = board!(
           ['R', ..];
           empty;
           empty;
           empty;
           empty;
           empty;
           empty;
           ['r', ..];
        );
        let board = Board::with_array(board);
        let mut game = Game::with_board(board);
        let ev = game
            .mov(&Point { x: 0, y: 0 }, &Point { x: 0, y: 7 })
            .unwrap();

        assert_eq!(ev, Event::None);
        assert_eq!(game.eaten.local, vec![]);
    }

    #[test]
    fn rival_eat_ads_to_local() {
        let board = board!(
           ['R', ..];
           empty;
           empty;
           empty;
           empty;
           empty;
           empty;
           ['r', ..];
        );
        let board = Board::with_array(board);
        let mut game = Game::new(board, Color::White, Color::Black, Eaten::empty());
        let ev = game
            .mov(&Point { x: 0, y: 7 }, &Point { x: 0, y: 0 })
            .unwrap();

        let may_eaten = Piece {
            kind: Kind::Rook,
            color: Color::White,
        };

        assert_eq!(ev, Event::None);
        assert_eq!(game.eaten.local, vec![may_eaten]);
    }

    #[test]
    fn can_win() {
        let board = board!(
           ['R', ..];
           empty;
           empty;
           empty;
           empty;
           empty;
           empty;
           ['k', ..];
        );
        let board = Board::with_array(board);
        let mut game = Game::with_board(board);
        let ev = game
            .mov(&Point { x: 0, y: 0 }, &Point { x: 0, y: 7 })
            .unwrap();

        assert_eq!(ev, Event::Win(Color::White));
    }
}

use std::borrow::BorrowMut;

use crate::traits::get_two_points_mut::Point;

use self::{
    board::{
        mov::{Info, InfoKind},
        Board,
    },
    color::Color,
    mov::MoveContext,
    piece::{Kind, Piece},
    query::sign::Sign,
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
    QueenArrive(Color),
    None,
}

impl Game {
    pub const fn with_board(board: Board) -> Self {
        Self {
            board,
            local: Color::White,
            turn: Color::White,
            eaten: Eaten::empty(),
        }
    }

    pub fn mov(&mut self, from: &Point, to: &Point) -> Result<Event, board::mov::Error> {
        if let Some(piece) = self
            .board
            .array()
            .get(from.y)
            .and_then(|arr| arr.get(from.x))
            .and_then(|cell| cell.piece.as_ref())
        {
            if piece.color != self.turn {
                return Err(board::mov::Error::PieceNotOfTeam);
            }

            let mov_context = MoveContext {
                to: to.clone(),
                from: from.clone(),
                game: self,
            };

            if !piece.is_valid_move(mov_context) {
                return Err(board::mov::Error::InvalidMove);
            }
        }

        let is_last_row = self.is_last_row(to.y);

        board::mov::board(self.board.borrow_mut(), from, to).map(|info| {
            let event = match info {
                Info {
                    kind:
                        InfoKind::Eaten(Piece {
                            kind: Kind::King, ..
                        }),
                    ..
                } => Event::Win(self.turn),

                Info { kind, moved } => {
                    if let InfoKind::Eaten(piece) = kind {
                        self.eaten.push(self.local, piece);
                    }

                    if moved.kind == piece::Kind::Pawn && is_last_row {
                        moved.kind = piece::Kind::Queen;
                        Event::QueenArrive(self.turn)
                    } else {
                        Event::None
                    }
                }
            };

            self.turn = self.turn.contrary();
            event
        })
    }

    pub fn pawn_first_mov(&self, row: usize, color: Color) -> bool {
        if self.local == color {
            row == 1
        } else {
            row == 6
        }
    }

    pub fn direction_of(&self, color: Color) -> Sign {
        if self.local == color {
            Sign::Positive
        } else {
            Sign::Negative
        }
    }

    pub fn is_last_row(&self, row: usize) -> bool {
        if self.direction_of(self.turn) == Sign::Positive {
            row + 1 == self.board.array().len()
        } else {
            row == 0
        }
    }
}

#[cfg(test)]
impl Game {
    pub const fn new(board: Board, local: Color, turn: Color, eaten: Eaten) -> Self {
        Self {
            board,
            turn,
            local,
            eaten,
        }
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

    #[test]
    fn converts_to_queen() {
        let board = board!(
            empty;
            empty;
            empty;
            empty;
            empty;
            empty;
            ['P', ..];
            empty;
        );

        let board = Board::with_array(board);
        let mut game = Game::with_board(board);

        let ev = game
            .mov(&Point { x: 0, y: 6 }, &Point { x: 0, y: 7 })
            .unwrap();

        assert_eq!(ev, Event::QueenArrive(Color::White));
    }
}

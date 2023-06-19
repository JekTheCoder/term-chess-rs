macro_rules! piece {
    ($color: tt, $kind: tt) => {
        crate::game::piece::Piece {
            color: crate::game::color::Color::$color,
            kind: crate::game::piece::Kind::$kind,
        }
    };
}

macro_rules! cell {
    (empty) => {
        crate::game::cell::Cell { piece: None }
    };
    (' ') => {
        cell!(empty)
    };
	('R') => {
		crate::game::cell::Cell {
			piece: Some(crate::macros::piece!(White, Rook)),
		}
	};
    ('K') => {
        cell!('♔')
    };
	('B') => {
		crate::game::cell::Cell {
			piece: Some(crate::macros::piece!(White, Bishop)),
		}
	};
	('R') => {
		crate::game::cell::Cell {
			piece: Some(crate::macros::piece!(White, Rook)),
		}
	};
	('N') => {
		crate::game::cell::Cell {
			piece: Some(crate::macros::piece!(White, Knight)),
		}
	};
	('Q') => {
		crate::game::cell::Cell {
			piece: Some(crate::macros::piece!(White, Queen)),
		}
	};
	('P') => {
		crate::game::cell::Cell {
			piece: Some(crate::macros::piece!(White, Pawn)),
		}
	};
    ('♔') => {
        crate::game::cell::Cell {
            piece: Some(crate::macros::piece!(White, King)),
        }
    };
	('♕') => {
		crate::game::cell::Cell {
			piece: Some(crate::macros::piece!(White, Queen)),
		}
	};
	('♗') => {
		crate::game::cell::Cell {
			piece: Some(crate::macros::piece!(White, Bishop)),
		}
	};
	('♖') => {
		crate::game::cell::Cell {
			piece: Some(crate::macros::piece!(White, Rook)),
		}
	};
	('♘') => {
		crate::game::cell::Cell {
			piece: Some(crate::macros::piece!(White, Knight)),
		}
	};
	('♙') => {
		crate::game::cell::Cell {
			piece: Some(crate::macros::piece!(White, Pawn)),
		}
	};
    ('♚') => {
        crate::game::cell::Cell {
            piece: Some(Piece::new(Color::Black, Kind::King)),
        }
    };
	('♛') => {
		crate::game::cell::Cell {
			piece: Some(crate::macros::piece!(Black, Queen)),
		}
	};
	('♜') => {
		crate::game::cell::Cell {
			piece: Some(crate::macros::piece!(Black, Rook)),
		}
	};
	('♝') => {
		crate::game::cell::Cell {
			piece: Some(crate::macros::piece!(Black, Bishop)),
		}
	};
	('♞') => {
		crate::game::cell::Cell {
			piece: Some(crate::macros::piece!(Black, Knight)),
		}
	};
	('♟') => {
		crate::game::cell::Cell {
			piece: Some(crate::macros::piece!(Black, Pawn)),
		}
	};
    ('k') => {
        crate::macros::cell!('♚')
    };
	('r') => {
		crate::game::cell::Cell {
			piece: Some(crate::macros::piece!(Black, Rook)),
		}
	};
	('b') => {
		crate::game::cell::Cell {
			piece: Some(crate::macros::piece!(Black, Bishop)),
		}
	};
	('n') => {
		crate::game::cell::Cell {
			piece: Some(crate::macros::piece!(Black, Knight)),
		}
	};
	('q') => {
		crate::game::cell::Cell {
			piece: Some(crate::macros::piece!(Black, Queen)),
		}
	};
	('p') => {
		crate::game::cell::Cell {
			piece: Some(crate::macros::piece!(Black, Pawn)),
		}
	}
}

macro_rules! row {
    (empty) => {
        [
            crate::macros::cell!(empty),
            crate::macros::cell!(empty),
            crate::macros::cell!(empty),
            crate::macros::cell!(empty),
            crate::macros::cell!(empty),
            crate::macros::cell!(empty),
            crate::macros::cell!(empty),
            crate::macros::cell!(empty),
        ]
    };
    ([$v1: tt, $v2: tt, $v3: tt, $v4: tt, $v5: tt, $v6: tt, $v7: tt, $v8: tt]) => {
        [
            crate::macros::cell!($v1),
            crate::macros::cell!($v2),
            crate::macros::cell!($v3),
            crate::macros::cell!($v4),
            crate::macros::cell!($v5),
            crate::macros::cell!($v6),
            crate::macros::cell!($v7),
            crate::macros::cell!($v8),
        ]
    };
    ([$v1: tt, ..]) => {
        crate::macros::row!([$v1, empty, empty, empty, empty, empty, empty, empty])
    };
    ([$v1: tt, $v2: tt, ..]) => {
        crate::macros::row!([$v1, $v2, empty, empty, empty, empty, empty, empty])
    };
    ([$v1: tt, $v2: tt, $v3: tt, ..]) => {
        crate::macros::row!([$v1, $v2, $v3, empty, empty, empty, empty, empty])
    };
}

macro_rules! board {
        ($($r:tt;)+) => {
            [
                $(
					crate::macros::row!($r),
                )+
            ]
        }
}

pub(crate) use board;
pub(crate) use cell;
pub(crate) use piece;
pub(crate) use row;

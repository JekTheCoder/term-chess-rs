use std::str::FromStr;

use self::coord::Coord;

pub struct Mov {
    pub from: Coord,
    pub to: Coord,
}

impl Mov {
	pub const fn into_points(self) -> crate::mov::Mov {
		crate::mov::Mov {
			from: self.from.into_point(),
			to: self.to.into_point(),
		}
	}
}

pub enum Kind {
    From,
    To,
}
pub enum ParseError {
    Coord { kind: Kind, err: coord::ParseError },
    NotFound,
}

impl FromStr for Mov {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (f, sec) = s.split_once('-').ok_or_else(|| ParseError::NotFound)?;
        let from = Coord::from_str(f).map_err(|err| ParseError::Coord {
            kind: Kind::From,
            err,
        })?;

        let to = Coord::from_str(sec).map_err(|err| ParseError::Coord {
            kind: Kind::To,
            err,
        })?;

		Ok(Self { from, to })
    }
}

pub mod coord {
    use std::{num::NonZeroUsize, str::FromStr};

    use crate::traits::get_two_points_mut::Point;

    pub struct Coord {
        pub col: char,
        pub row: NonZeroUsize,
    }

	impl Coord {
		pub const fn into_point(self) -> Point {
			let c_code = self.col as usize;	

			Point {
				x: c_code - 97,
				y: self.row.get() - 1,
			}
		}
	}

    pub enum ParseError {
        Col,
        Row,
    }

    impl FromStr for Coord {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut chars = s.chars();
            let c = chars.next().ok_or(ParseError::Col)?;
            let col = match c {
                'a'..='h' => c,
                'A'..='H' => c.to_ascii_lowercase(),
                _ => return Err(ParseError::Col),
            };

            let row_n = chars
                .next()
                .ok_or(ParseError::Row)?
                .to_digit(10)
                .ok_or(ParseError::Row)?;
            let row_n: usize = row_n.try_into().map_err(|_| ParseError::Row)?;

            let row = NonZeroUsize::new(row_n).ok_or(ParseError::Row)?;

            Ok(Self { col, row })
        }
    }
}

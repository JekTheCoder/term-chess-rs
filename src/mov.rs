use std::str::{FromStr, Chars};

use crate::traits::get_two_points_mut::Point;

pub struct Mov {
    pub from: Point,
    pub to: Point,
}

#[derive(Debug)]
pub struct ParseError;

impl FromStr for Mov {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut chars = s.chars();
        let from = parse_point(&mut chars)?;
		
		expect(chars.next(), ' ')?;
		expect(chars.next(), '-')?;
		expect(chars.next(), ' ')?;

		let to = parse_point(&mut chars)?;

		Ok(Self { from, to })
    }
}

const fn expect(char: Option<char>, to_be: char) -> Result<(), ParseError> {
    match char {
        Some(c) if c == to_be => Ok(()),
        _ => Err(ParseError),
    }
}

fn char_to_usize(c: Option<char>) -> Result<usize, ParseError> {
    c.ok_or(ParseError)?
        .to_digit(10)
        .map(|n| n.try_into().unwrap())
        .ok_or(ParseError)
}

fn parse_point(chars: &mut Chars<'_>) -> Result<Point, ParseError> {
    let x = char_to_usize(chars.next())?;
    expect(chars.next(), ' ')?;
    let y = char_to_usize(chars.next())?;

    Ok(Point { x, y })
}

mod tests {
	#[allow(unused_imports)]
    use super::*;

	#[test]
	fn it_works() {
		let input = "1 2 - 4 5";
		let mov = super::Mov::from_str(input);

		assert!(mov.is_ok());
	}

	#[test]
	fn it_gets_numbers() {
		let input = "1 2 - 4 5";
		let Mov { from, to } = super::Mov::from_str(input).unwrap();

		assert_eq!(from, Point { x: 1, y: 2 });
		assert_eq!(to, Point { x: 4, y: 5 });
	}
}

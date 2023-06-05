use crate::mov::Mov;

pub trait Valid {
    fn mov_valid(mov: Mov) -> bool;
}

struct Rook;
impl Valid for Rook {
    fn mov_valid(mov: Mov) -> bool {
		let Mov { from, to } = mov;
		from.x == to.x || from.y == to.y
	}
}

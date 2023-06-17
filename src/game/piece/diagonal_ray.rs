use crate::{
    game::query::{ray2d::Ray2D, sign::Sign},
    traits::get_two_points_mut::Point,
};

pub const fn diagonal_ray(from: Point, to: Point) -> Option<Ray2D> {
	let x_diff = to.x as isize - from.x as isize; 
	let y_diff = to.y as isize - from.y as isize;

	if x_diff.abs() != y_diff.abs() || x_diff == 0 {
		return None;
	}
	
	let h = if x_diff > 0 { Sign::Positive } else { Sign::Negative };
	let v = if y_diff > 0 { Sign::Positive } else { Sign::Negative };

	Some(Ray2D::new(h, v))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal_ray() {
        let from = Point { x: 0, y: 0 };
        let to = Point { x: 1, y: 1 };
        let ray = diagonal_ray(from, to);

        assert!(ray.is_some());
    }

	#[test] 
	fn gets_with_distance() {
		let from = Point { x: 0, y: 0 };
		let to = Point { x: 6, y: 6 };

		let ray = diagonal_ray(from, to);
		assert_eq!(ray.unwrap(), Ray2D::new(Sign::Positive, Sign::Positive));
	}

	#[test]
	fn gets_negative() {
		let from = Point { x: 5, y: 5 };
		let to = Point { x: 0, y: 0 };

		let ray = diagonal_ray(from, to);

		assert_eq!(ray.unwrap(), Ray2D::new(Sign::Negative, Sign::Negative));
	}

	#[test]
	fn gets_mixed() {
		let from = Point { x: 5, y: 5 };
		let to = Point { x: 10, y: 0 };

		let ray = diagonal_ray(from, to);
		assert_eq!(ray.unwrap(), Ray2D::new(Sign::Positive, Sign::Negative));
	}
}

use crate::{
    game::query::{ray2d::Ray2D, sign::Sign},
    traits::get_two_points_mut::Point,
};

pub fn diagonal_ray(from: &Point, to: &Point) -> Option<Ray2D> {
    let to_x = i8::try_from(to.x).ok()?;
    let to_y = i8::try_from(to.y).ok()?;

    let from_x = i8::try_from(from.x).ok()?;
    let from_y = i8::try_from(from.y).ok()?;

	let x_diff = to_x - from_x; 
	let y_diff = to_y - from_y;

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
        let ray = diagonal_ray(&from, &to);

        assert!(ray.is_some());
    }

	#[test] 
	fn gets_with_distance() {
		let from = Point { x: 0, y: 0 };
		let to = Point { x: 6, y: 6 };

		let ray = diagonal_ray(&from, &to);
		assert_eq!(ray.unwrap(), Ray2D::new(Sign::Positive, Sign::Positive));
	}

	#[test]
	fn gets_negative() {
		let from = Point { x: 5, y: 5 };
		let to = Point { x: 0, y: 0 };

		let ray = diagonal_ray(&from, &to);

		assert_eq!(ray.unwrap(), Ray2D::new(Sign::Negative, Sign::Negative));
	}

	#[test]
	fn gets_mixed() {
		let from = Point { x: 5, y: 5 };
		let to = Point { x: 10, y: 0 };

		let ray = diagonal_ray(&from, &to);
		assert_eq!(ray.unwrap(), Ray2D::new(Sign::Positive, Sign::Negative));
	}
}

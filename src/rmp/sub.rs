use std::ops::{Sub, Neg};

use super::Integer;

impl Sub for Integer {
	type Output = Integer;

	fn sub( self, rhs : Integer) -> Integer {
		self + rhs.neg()
	}
}

use std::ops::{Sub, Neg};

use super::Integer;

impl Integer {
	pub fn minus( &self, rhs : &Integer) -> Integer {
		panic!("TODO")
	}
}

impl Sub for Integer {
	type Output = Integer;

	fn sub( self, rhs : Integer) -> Integer {
		self + rhs.neg()
	}
}

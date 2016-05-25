use std::ops::{Sub, Neg};

use super::Integer;

impl Integer {
	// Borrowed subtraction.
	pub fn sub_borrow( &self, rhs : &Integer) -> Integer {
		panic!("TODO")
	}

	// Mutable borrowed subtraction.
	pub fn sub_mut( &mut self, rhs : &mut Integer) -> Integer {
		panic!("TODO")
	}
}

impl Sub for Integer {
	type Output = Integer;

	fn sub( self, rhs : Integer) -> Integer {
		self + rhs.neg()
	}
}

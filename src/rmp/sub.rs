use std::ops::{Sub};

use super::Integer;

impl Integer {
	// Borrowed subtraction.
	pub fn sub_borrow( &self, rhs : &Integer) -> Integer {
		let mut r = self.clone();
		r.sub_mut( rhs);
		r
	}

	// Mutable borrowed subtraction.
	pub fn sub_mut( &mut self, rhs : &Integer) {
		self.add_mut( &rhs.neg_borrow())
	}
}

impl Sub for Integer {
	type Output = Integer;

	fn sub( self, rhs : Integer) -> Integer {
		let mut r = self;
		r.sub_mut( &rhs);
		r
	}
}

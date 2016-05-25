use std::ops::Mul;

use super::{Integer};

impl Integer {
	pub fn mul_borrow( &self, rhs : &Integer) -> Integer {
		let mut r = self.clone();
		r.mul_mut( rhs);
		r
	}
	pub fn mul_mut( &mut self, rhs : &Integer) {
		// TODO: various algorithms dependent on the size of inputs. XXX

		panic!("TODO")
	}
}

impl Mul for Integer {
	type Output = Integer;

	fn mul(self, rhs : Integer) -> Integer {
		let mut r = self;
		r.mul_mut( &rhs);
		r
	}
}


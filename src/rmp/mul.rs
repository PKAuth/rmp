use std::ops::Mul;

use super::{Integer};

impl Integer {
	pub fn mul_borrow( &self, rhs : &Integer) -> Integer {
		// TODO: various algorithms dependent on the size of inputs. XXX

		panic!("TODO")
	}
}

impl Mul for Integer {
	type Output = Integer;

	fn mul(self, rhs : Integer) -> Integer {
		self.mul_borrow( &rhs)
	}
}


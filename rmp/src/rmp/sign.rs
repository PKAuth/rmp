use std::ops::{Neg};

use super::{Integer};

impl Integer {
	/// Check if the integer is positive.
	#[inline(always)]
	pub fn is_positive(&self) -> bool {
		self.positive
	}
	
	/// Check if the integer is negative.
	#[inline(always)]
	pub fn is_negative(&self) -> bool {
		!self.positive
	}

	/// Borrowed negation.
	pub fn neg_borrow( &self) -> Integer {
		let mut r = self.clone();
		r.neg_mut();
		r
	}

	/// Mutable negation.
	pub fn neg_mut(&mut self) {
		if !self.is_zero() {
			self.positive = !self.positive
		}
	}
}

impl Neg for Integer {
	type Output = Integer;

	fn neg(self) -> Integer {
		let mut r = self;
		r.neg_mut();
		r
	}
}


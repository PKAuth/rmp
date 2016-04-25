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

	/// Mutable negation.
	pub fn neg_m(&mut self) {
		if !self.is_zero() {
			self.positive = !self.positive
		}
	}
}

impl Neg for Integer {
	type Output = Integer;

	fn neg(self) -> Integer {
		if self.is_zero() {
			self
		}
		else {
			Integer{ positive : !self.positive, content : self.content}
		}
	}
}


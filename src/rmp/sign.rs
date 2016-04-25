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
}

impl Neg for Integer {
	type Output = Integer;

	fn neg(self) -> Integer {
		Integer{ positive : !self.positive, content : self.content}
	}
}


use std::cmp::{Ord, max, min, Ordering};
use std::convert::From;
use std::fmt;
// use std::isize as isize;
// use std::num::Wrapping;
use std::ops::{Neg};
// use std::string::ToString;

use rmp::div::{div_mod_u};
use rmp::internal::{div_by_zero, Block};


impl Integer {
	/// Checks whether the integer is a multiple of the argument.
	pub fn is_multiple_of(&self, i : &Integer) -> bool {
		i.divides( self)
	}

	/// Checks whether the integer divides the argument.
	pub fn divides(&self, a : &Integer) -> bool {
		a.modulus( self).is_zero()


		// Could we binary search this??
	}

	pub fn div_mod(&self, rhs : &Integer) -> (Integer, Integer) {
		// Check for div by 0.
		if rhs.is_zero() {
			div_by_zero()
		}

		// TODO: check other base conditions here... XXX


		let (q, r) = div_mod_u( &self.content, &rhs.content);
		match ( self.is_positive(), rhs.is_positive()) {
			(true, true) =>
				(Integer{positive : true, content : q}, Integer{ positive : true, content : r}),
			_ => 
				panic!("TODO")
		}
	}

}

// Trait implementations.


// impl ToString for Integer {
// 	fn to_string(&self) -> String {
// 		// TODO: Actually implement this XXX
// 		let mut s = String::new();
// 		for n in &self.content {
// 			s = n.to_string()
// 		}
// 		s
// 	}
// }


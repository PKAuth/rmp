
use std::cmp::{PartialEq, max, min};
use std::convert::From;
// use std::isize as isize;
// use std::num::Wrapping;
use std::ops::{Add, Neg};
// use std::string::ToString;

// Data type for multi precision integers.
#[derive(Debug)]
pub struct Integer {
	// _size : isize, // Absolute value of the size of the number. MP is negative is size is negative.
	content : Vec<u64>, // Contents of the number. If number is 0, length of content is 0.
	positive : bool,    // If the number is positive. If number is 0, positive is true.
}

impl Integer {
	// // Get the max index for the given size.
	// fn max_index(&self) -> isize {
	// 	isize::abs(self._size)
	// }

	// // Get the max index for the given size as a usize.
	// fn max_index_u(&self) -> usize {
	// 	self.max_index() as usize
	// }

	// Get the current size.
	#[inline(always)]
	fn size(&self) -> usize {
		self.content.len()
	}

	// Get the current capacity.
	#[inline(always)]
	fn capacity(&self) -> usize {
		self.content.capacity()
	}
	
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

// Trait implementations.

impl Neg for Integer {
	type Output = Integer;

	fn neg(self) -> Integer {
		Integer{ positive : !self.positive, content : self.content}
	}
}

impl Add for Integer {
	type Output = Integer;

	fn add( self, rhs : Integer) -> Integer {
		fn add_positives( lhs : Integer, rhs : Integer) -> Vec<u64> {
			let mut cs = Vec::with_capacity( max( lhs.capacity(), rhs.capacity())); // TODO: Improve this capacity?? Do we need to zero out memory? XXX
			let mut i = 0;
			let mut c = false;
			let self_s = lhs.size();
			let rhs_s = rhs.size();
			let s = min( self_s, rhs_s);
			while i < s {
				let (mut x, d) = lhs.content[i].overflowing_add( rhs.content[i]);

				if c {
					let (y, e) = lhs.content[i].overflowing_add( 1);
					c = d || e;
					x = y
				}
				else {
					c = d
				}

				cs.push( x);
				i += 1
			}

			let (g,s) = if self_s > rhs_s { (lhs, self_s)} else { (rhs, rhs_s)};
			while i < s {
				if c {
					let (y, e) = g.content[i].overflowing_add( 1);
					cs.push(y);
					c = e;
				}
				else {
					// Optimisation: we can break here and copy the rest of the bits? XXX
					cs.push( g.content[i]);
					c = false;
				}

				i += 1
			}

			if c {
				cs.push( 1)
			}

			cs
		}

		match ( self.is_positive(), rhs.is_positive()) {
			(true, true) => {
				let cs = add_positives( self, rhs);
				Integer{ positive : true, content : cs}
			},
			(false, false) => {
				let cs = add_positives( self, rhs);
				Integer{ positive : false, content : cs}
			},
			_ =>
				panic!("TODO"),
		}
	}
}

impl From<u64> for Integer {
	fn from(x : u64) -> Integer {
		if x == 0 {
			Integer{ positive : true, content : Vec::with_capacity(1)}
		}
		else {
			Integer{ positive : true, content : vec!{x}}
		}
	}
}

impl From<i64> for Integer {
	fn from(x : i64) -> Integer {
		if x == 0 {
			Integer{ positive : true, content : Vec::with_capacity(1)}
		}
		else if x > 0 {
			Integer{ positive : true, content : vec!{x as u64}}
		}
		else if x == i64::min_value(){
			let u = i64::max_value() as u64 + 1;
			Integer{ positive : false, content : vec!{u}}
		}
		else {
			Integer{ positive : false, content : vec!{-x as u64}}
		}
	}
}

// impl ToString for Integer {
// 	fn to_string(&self) -> String {
// 		let mut s = String::new();
// 		for n in &self.content {
// 			s = n.to_string()
// 		}
// 		s
// 	}
// }

impl PartialEq for Integer {
	fn eq(&self, rhs : &Integer) -> bool {
		// &self._size == &rhs._size && &self.content == &rhs.content

		if self.size() != rhs.size() {
			return false
		}

		for i in 0..self.size() {
			if &self.content[i] != &rhs.content[i] {
				return false
			}
		}

		true
	}
}


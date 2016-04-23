
use std::cmp::{Ord, max, min, Ordering};
use std::convert::From;
use std::fmt;
// use std::isize as isize;
// use std::num::Wrapping;
use std::ops::{Add, Neg};
// use std::string::ToString;

use rmp::div::{div_mod_u};
use rmp::internal::{div_by_zero, Block};

// Data type for multi precision integers.
#[derive(Debug)]
pub struct Integer {
	content : Vec<Block>, // Blocks of the number. If number is 0, number of blocks is 0.
	positive : bool,    // Whether the number is positive. 
	// Maybe?? If number is 0, positive is true. If not, we need to change PartialEq.
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

	/// Determine if the integer is zero.
	#[inline(always)]
	pub  fn is_zero(&self) -> bool {
		self.size() == 0
	}

	/// Determine if the integer is even.
	#[inline(always)]
	pub fn is_even(&self) -> bool {
		if self.is_zero() {
			true
		}
		else if self.content[0] & 1 == 1 {
			false
		}
		else {
			true
		}
	}

	pub fn modulus(&self, m : &Integer) -> Integer {
		let (_, r) = self.div_mod( m);
		r
	}

	/// Determine if the integer is odd.
	#[inline(always)]
	pub fn is_odd(&self) -> bool {
		!self.is_even()
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
		fn add_positives( lhs : Integer, rhs : Integer) -> Vec<Block> {
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

impl From<Block> for Integer {
	fn from(x : Block) -> Integer {
		if x == 0 {
			Integer{ positive : true, content : Vec::with_capacity(1)}
		}
		else {
			Integer{ positive : true, content : vec!{x}}
		}
	}
}

impl From<i32> for Integer {
	fn from(x : i32) -> Integer {
		if x == 0 {
			Integer{ positive : true, content : Vec::with_capacity(1)}
		}
		else if x > 0 {
			Integer{ positive : true, content : vec!{x as u32}}
		}
		else if x == i32::min_value(){
			let u = i32::max_value() as u32 + 1;
			Integer{ positive : false, content : vec!{u}}
		}
		else {
			Integer{ positive : false, content : vec!{-x as u32}}
		}
	}
}

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

impl fmt::Display for Integer {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
 		// TODO: Actually implement this XXX
		if self.is_zero() {
			write!(f, "0")
		}
		else {
			write!(f, "{}", self.content[0].to_string())
		}
	}
}

impl Ord for Integer {
	fn cmp(&self, rhs: &Integer) -> Ordering {
		fn helper( a : &Integer, b : &Integer) -> Ordering {
			let a_s = a.size();
			let b_s = b.size();
			if a_s > b_s {
				return Ordering::Greater
			}
			else if a_s < b_s {
				return Ordering::Less
			}

			for i in (0..a_s).rev() {
				let a_i = a.content[i];
				let b_i = b.content[i];
				if a_i > b_i {
					return Ordering::Greater
				}
				else if a_i < b_i {
					return Ordering::Less
				}
			}

			Ordering::Equal
		}

		match ( self.positive, rhs.positive) {
			(true, false) =>
				Ordering::Greater,
			(false, true) =>
				Ordering::Less,
			(true, true) =>
				helper( self, rhs),
			(false, false) =>
				helper( self, rhs).reverse(),
		}
	}
}

impl PartialEq for Integer {
	fn eq(&self, rhs : &Integer) -> bool {
		self.cmp( rhs) == Ordering::Equal
	}
}

impl Eq for Integer {
}

impl PartialOrd for Integer {
	fn partial_cmp(&self, rhs: &Integer) -> Option<Ordering> {
		Some( self.cmp( rhs))
	}
}


use std::ops::{Shl};

use super::{Integer, Block, LongBlock, BLOCK_SIZE};
use super::internal::{pos_integer, to_block, to_usize};
// use rmp::shift::internal;

mod internal;

impl Integer {
	// Shift left while borrowing.
	pub fn shl_borrow( &self, rhs : &Integer) -> Integer {
		if rhs.is_negative() {
			panic!( "shl: rhs is negative")
		}

		let ( emptyBlockCI, shiftCI) = rhs.div_mod( &Integer::from( BLOCK_SIZE));
		let shiftC = to_block( &shiftCI);
		let emptyBlockC = to_usize( &emptyBlockCI);

		self.shl_block_borrow( shiftC, emptyBlockC)
	}

	// Shift right while borrowing.
	pub fn shr_borrow( &self, rhs : &Integer) -> Integer {
		if rhs.is_negative() {
			panic!( "shr: rhs is negative")
		}

		let ( skipCI, shiftCI) = rhs.div_mod( &Integer::from( BLOCK_SIZE));
		let shiftC = to_block( &shiftCI);
		let skipC = to_usize( &skipCI);

		self.shr_block_borrow( shiftC, skipC)
	}
}

impl Shl<Integer> for Integer {
	type Output = Integer;

	// Assumes rhs is positive.
	fn shl( self, rhs : Integer) -> Integer {
		self.shl_borrow( &rhs)
	}
}



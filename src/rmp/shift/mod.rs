use std::ops::{Shl};

use super::{Integer, Block, LongBlock, BLOCK_SIZE, LG_BLOCK_SIZE};
use super::internal::{pos_integer, to_block, to_usize};
// use rmp::shift::internal;

mod internal;

impl Integer {
	// Shift left while borrowing.
	pub fn shl_borrow( &self, rhs : &Integer) -> Integer {
		if rhs.is_negative() {
			panic!( "shl: rhs is negative")
		}

		let ( emptyBlockC, shiftC) = div_mod_block_size( &rhs);
		self.shl_block_borrow( shiftC, emptyBlockC)
	}

	// Shift right while borrowing.
	pub fn shr_borrow( &self, rhs : &Integer) -> Integer {
		if rhs.is_negative() {
			panic!( "shr: rhs is negative")
		}

		let ( skipC, shiftC) = div_mod_block_size( &rhs);
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

// Helper function.
// Assumes input is positive.
fn div_mod_block_size( x : &Integer) -> ( usize, Block) {
	if x.is_zero() {
		return ( 0, 0)
	}

	let mask = BLOCK_SIZE - 1;
	let rem = x.content[0] & mask;
	let div = to_usize( &x.shr_block_borrow( LG_BLOCK_SIZE, 0));

	(div, rem)
}


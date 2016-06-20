use std::ops::{Shl};

use super::{Integer, Block, BLOCK_SIZE, LG_BLOCK_SIZE};
use super::internal::{to_usize};
// use rmp::shift::internal;

mod internal;

impl Integer {
	// Shift left while borrowing.
	pub fn shl_borrow( &self, rhs : &Integer) -> Integer {
		if rhs.is_negative() {
			panic!( "shl: rhs is negative")
		}

		let ( empty_block_c, shift_c) = div_mod_block_size( &rhs);
		self.shl_block_borrow( shift_c, empty_block_c)
	}

	// Shift right while borrowing.
	pub fn shr_borrow( &self, rhs : &Integer) -> Integer {
		if rhs.is_negative() {
			panic!( "shr: rhs is negative")
		}

		let ( skip_c, shift_c) = div_mod_block_size( &rhs);
		self.shr_block_borrow( shift_c, skip_c)
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


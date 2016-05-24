use std::ops::{Shl};

use super::{Integer, Block, LongBlock, BLOCK_SIZE};
use super::internal::{pos_integer, to_block};

impl Integer {
	pub fn shl_borrow( &self, rhs : &Integer) -> Integer {
		if rhs.is_negative() {
			panic!( "shl: rhs is negative")
		}

		let ( shiftCI, emptyBlockC) = rhs.div_mod( &Integer::from( BLOCK_SIZE));
		let shiftC = to_block( &shiftCI);

		// Prepend 0 blocks.
		//let mut v = vec![0;emptyBlockC];
		let mut v = Vec::with_capacity( self.capacity());
		let mut i = Integer::from( 0);
		while i < emptyBlockC {
			v.push( 0);
			i.plus_mut( &Integer::from( 1))
		}

		// Shift the existing blocks.
		let mut carry : Block = 0;
		for i in 0..self.size() {
			let lbi : LongBlock = (self.content[i] as LongBlock) << shiftC;
			
			v.push( (lbi as Block) | carry);
			carry = (lbi >> BLOCK_SIZE) as Block;
		}

		pos_integer( v)
	}
}

impl Shl<Integer> for Integer {
	type Output = Integer;

	// Assumes rhs is positive.
	fn shl( self, rhs : Integer) -> Integer {
		self.shl_borrow( &rhs)
	}
}


use std::ops::{Shl};

use super::{Integer, Block, LongBlock, BLOCK_SIZE};
use super::internal::{pos_integer, to_block, to_usize};

impl Integer {
	// Shift left while borrowing.
	pub fn shl_borrow( &self, rhs : &Integer) -> Integer {
		if rhs.is_negative() {
			panic!( "shl: rhs is negative")
		}

		let ( emptyBlockC, shiftCI) = rhs.div_mod( &Integer::from( BLOCK_SIZE));
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

	// Shift right while borrowing.
	pub fn shr_borrow( &self, rhs : &Integer) -> Integer {
		if rhs.is_negative() {
			panic!( "shr: rhs is negative")
		}

		let ( skipCI, shiftCI) = rhs.div_mod( &Integer::from( BLOCK_SIZE));
		let shiftC = to_block( &shiftCI);
		let skipC = to_usize( &skipCI);

		// Note: Can the following be simplified? 
		let upperMask = if shiftC == 0 {0} else {Block::max_value() << (BLOCK_SIZE - shiftC)};
		let lowerMask = Block::max_value() >> shiftC;

		let mut v = vec![0;self.size() - skipC];

		// Shift the existing blocks.
		let mut carry : Block = 0;
		for i in (skipC..self.size()).rev() {
			let rot_i = self.content[i].rotate_right( shiftC);

			let j = i - skipC;
			v[j] = (rot_i & lowerMask) | carry;
			carry = rot_i & upperMask;
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


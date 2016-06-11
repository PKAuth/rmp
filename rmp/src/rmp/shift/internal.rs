use super::super::{Integer, Block, LongBlock, BLOCK_SIZE};
use super::super::internal::{pos_integer, neg_integer, to_block, to_usize};

impl Integer {
	// Shift to the right `shiftC` bits, dropping the `skipC` lowest blocks.
	pub fn shr_block_borrow( &self, shiftC : Block, skipC : usize) -> Integer {
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
		
		if self.positive {
			pos_integer( v)
		}
		else {
			neg_integer( v)
		}
	}

	// Shift to the left `shiftC` bits with `emptyBlockC` 0 lsbs.
	pub fn shl_block_borrow( &self, shiftC : Block, emptyBlockC : usize) -> Integer {
		// Prepend 0 blocks.
		// let mut v = vec![0;emptyBlockC];
		let mut v = Vec::with_capacity( self.capacity() + emptyBlockC);
		for i in 0..emptyBlockC {
			v.push( 0);
		}

		// Shift the existing blocks.
		let mut carry : Block = 0;
		for i in 0..self.size() {
			let lbi : LongBlock = (self.content[i] as LongBlock) << shiftC;
			
			v.push( (lbi as Block) | carry);
			carry = (lbi >> BLOCK_SIZE) as Block;
		}

		if carry != 0 {
			v.push( carry);
		}

		if self.positive {
			pos_integer( v)
		}
		else {
			neg_integer( v)
		}
	}
}

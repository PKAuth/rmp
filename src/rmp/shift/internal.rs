use super::super::{Integer, Block, LongBlock, BLOCK_SIZE};
use super::super::internal::{pos_integer, neg_integer};

impl Integer {
	// Shift to the right `shift_c` bits, dropping the `skip_c` lowest blocks.
	pub fn shr_block_borrow( &self, shift_c : Block, skip_c : usize) -> Integer {
		// Note: Can the following be simplified? 
		let upper_mask = if shift_c == 0 {0} else {Block::max_value() << (BLOCK_SIZE - shift_c)};
		let lower_mask = Block::max_value() >> shift_c;

		let mut v = vec![0;self.size() - skip_c];

		// Shift the existing blocks.
		let mut carry : Block = 0;
		for i in (skip_c..self.size()).rev() {
			let rot_i = self.content[i].rotate_right( shift_c);

			let j = i - skip_c;
			v[j] = (rot_i & lower_mask) | carry;
			carry = rot_i & upper_mask;
		}
		
		if self.positive {
			pos_integer( v)
		}
		else {
			neg_integer( v)
		}
	}

	// Shift to the left `shift_c` bits with `empty_block_c` 0 lsbs.
	pub fn shl_block_borrow( &self, shift_c : Block, empty_block_c : usize) -> Integer {
		// Prepend 0 blocks.
		// let mut v = vec![0;empty_block_c];
		let mut v = Vec::with_capacity( self.capacity() + empty_block_c);
		for _ in 0..empty_block_c {
			v.push( 0);
		}

		// Shift the existing blocks.
		let mut carry : Block = 0;
		for i in 0..self.size() {
			let lbi : LongBlock = (self.content[i] as LongBlock) << shift_c;
			
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

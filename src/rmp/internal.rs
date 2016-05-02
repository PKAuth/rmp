// Module with internal helper functions.

use super::{Block, LongBlock, Integer};

pub fn remove_leading_zeroes( v : &mut Vec<Block>) {
    while v.len() > 0 && v[v.len() - 1] == 0 {
        v.pop();
    }
}
pub fn pos_integer( mut v : Vec<Block>) -> Integer {
	remove_leading_zeroes( &mut v);

	Integer{ positive: true, content: v}
}

pub fn neg_integer( mut v : Vec<Block>) -> Integer {
	remove_leading_zeroes( &mut v);

	if v.len() == 0 {
		Integer{ positive: true, content: v}
	}
	else {
		Integer{ positive: false, content: v}
	}
}

pub fn dbl_to_bl( x : LongBlock) -> Block {
	x as Block // Looks like this just truncates. Don't need to mask?
}

pub fn div_by_zero() {
	panic!("division by zero")
}

impl Integer {
	// Get the current size.
	#[inline(always)]
	pub fn size(&self) -> usize {
		self.content.len()
	}

	// Get the current capacity.
	#[inline(always)]
	pub fn capacity(&self) -> usize {
		self.content.capacity()
	}
}


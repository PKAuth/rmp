// Module with internal helper functions.

use super::{Block, LongBlock, Integer, BLOCK_SIZE};

use std::mem::{size_of};

pub fn normalize_leading_zeroes( v : &mut Integer) {
	remove_leading_zeroes( &mut v.content);

	if v.size() == 0 {
		v.positive = true;
	}
}

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

	// Get the number of leading zeros in the binary of the most significant block.
	#[inline(always)]
	pub fn leading_zeros( &self) -> Block {
		self.content[self.size() - 1].leading_zeros()
	}

	// Get the number of trailing zeros in the binary representation.
	pub fn trailing_zeros( &self) -> Integer {
		let mut c = Integer::from( 0);

		for b in &self.content {
			let trailing = b.trailing_zeros();
			c.add_mut( &Integer::from( trailing));

			if trailing != BLOCK_SIZE {
				return c
			}
		}

		c
	}
}

// Get the ith bit of x. 0 is lsb.
#[inline(always)]
pub fn get_bit( x : Block, i : Block) -> Block {
	(x >> i) & 1
}

// Get n bits of x, starting at i (towards the lsb). 0 is lsb.
#[inline(always)]
pub fn get_bits( x : Block, i : Block, n : Block) -> Block {
	(x >> (i + 1 - n)) & ((1 << n) - 1)
}

/*
// Coerce to a Block. Panics if out of range.
pub fn to_block( x : &Integer) -> Block {
	if x < &Integer::from( 0) || x > &Integer::from( Block::max_value()) {
		panic!( "to_block: Invalid Integer")
	}
	else if x.is_zero() {
		0
	}
	else {
		x.content[0]
	}
}
*/

// Coerce to a usize. Panics if out of range.
pub fn to_usize( x : &Integer) -> usize {
	// TODO: Implement this check. XXX
	// if x < &Integer::from( 0) || x > &Integer::from( usize::max_value()) {
	// 	panic!( "to_usize: Invalid Integer")
	// }
	if x.is_zero() {
		0
	}
	else {
		let mut r : usize = 0;

		for i in 0..x.size() {
			let b = x.content[i] as usize;
			match r.checked_shl( BLOCK_SIZE) {
				None =>
					panic!("to_usize: overflow"),
				Some( s) => {
					match s.checked_add( b) {
						Some( next) => 
							r = next,
						None =>
							break,
					}
				},
			}
		}

		r
	}
}

// Return the ith block if it exists. Otherwise, return 0.
pub fn get_zero( x : &Integer, i : usize) -> Block {
	match x.content.get( i) {
		None => 0,
		Some( xi) => *xi,
	}
}

// Computes the ceiling of usize division. Assumes x != 0. 
// http://stackoverflow.com/a/2745086/382462
#[inline(always)]
pub fn ceiling( x : usize, y : usize) -> usize {
	1 + ((x - 1) / y)
}

// Checks if x is a power of 2. 
// http://stackoverflow.com/a/600306/382462
pub fn is_power_of_two( x : usize) -> bool {
	(x != 0) && (x & (x - 1) == 0)
}

// Computes the ceiling of the log (base two) of the input. Assumes x != 0.
pub fn ceiling_log_two( x : usize) -> usize {
	let h = 8 * size_of::<usize>() - (x.leading_zeros() as usize);
	if is_power_of_two( x) {
		h - 1
	}
	else {
		h
	}
}

#[inline(always)]
// JP: Move to a trait?
pub fn usize_is_odd(x : usize) -> bool {
	x & 1 == 1
}

// Internal module for division.

use super::{Integer, Block, LongBlock, BLOCK_SIZE};
use super::internal::{remove_leading_zeroes, dbl_to_bl, div_by_zero};

impl Integer {
	/// Checks whether the integer is a multiple of the argument.
	pub fn is_multiple_of(&self, i : &Integer) -> bool {
		i.divides( self)
	}

	/// Checks whether the integer divides the argument.
	pub fn divides(&self, a : &Integer) -> bool {
		a.modulus( self).is_zero()
	}

	// Returns the quotient and remainder.
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

	pub fn modulus(&self, m : &Integer) -> Integer {
		let (_, r) = self.div_mod( m);
		r
	}
}

// Divide and mod an unsigned integer.
fn div_mod_u( u : &Vec<Block>, v : &Vec<Block>) -> ( Vec<Block>, Vec<Block>) {
	// Check for Nx1.
	if v.len() == 1 {
		return div_mod_u_n_1( u, v[0])
	}

	panic!("TODO")
}

// Divide and mod an unsigned integer when the divisor is one block.
// From: Hacker's Delight, Second Edition by Henry Warren
fn div_mod_u_n_1( u : &Vec<Block>, v : Block) -> ( Vec<Block>, Vec<Block>) {
	#[inline(always)]
	fn mul_b( x : LongBlock) -> LongBlock {
		x << BLOCK_SIZE
	}

	let v = v as LongBlock;
	let mut q = Vec::with_capacity( u.capacity());
	let mut k : LongBlock = 0;

	// 2x1 division.
	for j in (0..u.len()).rev() {
		let kb_uj : LongBlock = mul_b( k) + u[j] as LongBlock;
		let qj : Block = dbl_to_bl(kb_uj / v);

		k = kb_uj - (qj as LongBlock) * v;
		q.push(qj);
	}

	// Convert k to vector.
	let mut k = vec![dbl_to_bl(k)];

	// Reverse vector.
	q.reverse();

	// Remove leading 0s.
	remove_leading_zeroes( &mut q);
	remove_leading_zeroes( &mut k);

	
	(q, k)
}


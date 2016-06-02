// Internal module for division.

use super::{Integer, Block, LongBlock, BLOCK_SIZE, SignedLongBlock};
use super::internal::{remove_leading_zeroes, dbl_to_bl, div_by_zero, pos_integer};

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

		let (mut q, mut r) = div_mod_u( &self.content, &rhs.content);
		match ( self.is_positive(), rhs.is_positive()) {
			(true, true) => {
				(q, r)
			},
			(false, false) => {
				r.neg_mut();
				( q, r)
			},
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
fn div_mod_u( u : &Vec<Block>, v : &Vec<Block>) -> ( Integer, Integer) {
	// Check if u is shorter than v.
	if u.len() < v.len() {
		// return (Integer::From(0), u.content)
		panic!("TODO")
	}

	// Check for Nx1.
	if v.len() == 1 {
		return div_mod_u_n_1( u, v[0])
	}

	// TODO: various algorithms dependent on the size of inputs. XXX
	panic!("TODO")

}

// From: The Art of Computer Programming - Volume 2 by Knuth. Algorithm D.
fn div_mod_base_case_positives( lhs : &Integer, rhs : &Integer) -> (Integer, Integer) {
	// Normalise.
	let s = Integer::from( rhs.leading_zeros());
	let mut ln = lhs.shl_borrow( &s).content;
	let rn = rhs.shr_borrow( &s).content;

	let mut quot : Vec<Block> = Vec::with_capacity( ln.len());
	quot.resize( ln.len(), 0);

	let r_end : LongBlock = rn[rn.len() - 1] as LongBlock;
	let r_end2 : LongBlock = rn[rn.len() - 2] as LongBlock;
	for j in (0..ln.len() - rn.len() + 1).rev() {
		// Note: Check if u_j == v_end??

		let lj : LongBlock = ln[j] as LongBlock;
		let lj1 : LongBlock = ln[j-1] as LongBlock;
		let lj2 : LongBlock = ln[j-2] as LongBlock;
		let mut qhat : LongBlock = (mul_b( lj) + lj1) / r_end;
		while r_end2 * qhat > mul_b(mul_b( lj) + lj1 - qhat * r_end) + lj2 {
			qhat = qhat - 1;
		}

		let mut k : SignedLongBlock = 0;
		let mask = Block::max_value() as LongBlock;
		for i in 0..rn.len() {
			let p : LongBlock = qhat * (rn[i] as LongBlock);
			let t : SignedLongBlock = (ln[i+j] as SignedLongBlock) - k - ((p & mask) as SignedLongBlock);
			ln[i+j] = t as Block;
			k = ((p >> (BLOCK_SIZE as LongBlock)) as SignedLongBlock) - (t >> (BLOCK_SIZE as SignedLongBlock));

			panic!("TODO");
		}

		let t : SignedLongBlock = (ln[rn.len()+j] as SignedLongBlock) - k;
		ln[rn.len()+j] = t as Block;

		quot[j] = qhat as Block; // TODO: this is probably wrong??
		if t < 0 {
			quot[j] = quot[j] - 1;
			let mut k : LongBlock = 0;
			for i in 0..rn.len() {
				let t : LongBlock = (ln[i + j] as LongBlock) + (rn[i] as LongBlock) + k;
				ln[i + j] = t as Block;
				k = t >> (BLOCK_SIZE as LongBlock);
			}
			ln[rn.len() + j] = ln[rn.len() + j] + (k as Block);
		}
	}

	let q = pos_integer( quot);
	let r = pos_integer( rn);

	// TODO: mut shiftr XXX

	(q, r)
}

#[inline(always)]
fn mul_b( x : LongBlock) -> LongBlock {
	x << BLOCK_SIZE
}

// Divide and mod an unsigned integer when the divisor is one block.
// From: Hacker's Delight, Second Edition by Henry Warren
fn div_mod_u_n_1( u : &Vec<Block>, v : Block) -> ( Integer, Integer) {
	let v = v as LongBlock;
	let mut q = Vec::with_capacity( u.len());
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

	(pos_integer( q), pos_integer( k))
}


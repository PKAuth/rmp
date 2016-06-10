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

		let (mut q, mut r) = div_mod_positives( &self, &rhs);
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

// Divide and mod a (positive) integer.
fn div_mod_positives( u : &Integer, v : &Integer) -> ( Integer, Integer) {
	// Check if u is shorter than v.
	if u.size() < v.size() {
		return (Integer::from(0), u.clone())
	}

	// Check for Nx1.
	if v.size() == 1 {
		return div_mod_u_n_1( &u.content, v.content[0])
	}


	// TODO: various algorithms dependent on the size of inputs. XXX
	div_mod_base_case_positives( u, v)
}

// From: The Art of Computer Programming - Volume 2 by Knuth. Algorithm D.
// Referenced: Hacker's Delight, Second Edition by Henry Warren.
fn div_mod_base_case_positives( lhs : &Integer, rhs : &Integer) -> (Integer, Integer) {
	println!("{}/{}", lhs, rhs);

	// Normalise.
	let s = rhs.leading_zeros();
	let mut ln = lhs.shl_block_borrow( s, 0).content;
	let rn = rhs.shl_block_borrow( s, 0).content;

	let mut quot : Vec<Block> = vec![0; ln.len()];

	let r_end : LongBlock = rn[rn.len() - 1] as LongBlock;
	let r_end2 : LongBlock = rn[rn.len() - 2] as LongBlock;
	let bm1 : LongBlock = Block::max_value() as LongBlock;
	let n = rhs.size();
	if lhs.size() == ln.len() {
		ln.push( 0);
	}

	for j in (0..lhs.size() - n + 1).rev() {
		let lj : LongBlock = ln[n+j] as LongBlock;
		let lj1 : LongBlock = ln[n+j-1] as LongBlock;
		let lj2 : LongBlock = ln[n+j-2] as LongBlock;
		let mut qhat : LongBlock = (mul_b( lj) + lj1) / r_end;
		println!("{}/{}", mul_b( lj) + lj1, r_end);
		println!("qhat': {}", qhat);
		if qhat > bm1 {
			qhat = bm1;
		}

		println!( "{}, {}, {}, {}, {}", r_end2, r_end, lj, lj1, lj2);
		while r_end2 * qhat > mul_b(mul_b( lj) + lj1 - qhat * r_end) + lj2 {
			if (mul_b( lj) + lj1 - qhat * r_end > bm1) {
				println!( "ljlj1: {}", mul_b( lj) + lj1);
				println!( "qr: {} * {} = {}", qhat, r_end, qhat * r_end);
				println!("bad: {}", mul_b( lj) + lj1 - qhat * r_end);
			}

			qhat = qhat - 1;

			// Check if it's going to overflow (implies the while condition will fail). 
			if mul_b( lj) + lj1 - qhat * r_end > bm1 {
				break;
			}
		}
		println!("qhat: {}", qhat);

		let mut k : SignedLongBlock = 0;
		let mask = Block::max_value() as LongBlock;
		for i in 0..n {
			let p : LongBlock = qhat * (rn[i] as LongBlock);
			let t : SignedLongBlock = (ln[i+j] as SignedLongBlock) - k - ((p & mask) as SignedLongBlock);
			ln[i+j] = t as Block;
			k = ((p >> (BLOCK_SIZE as LongBlock)) as SignedLongBlock) - (t >> (BLOCK_SIZE as SignedLongBlock));
		}

		let t : SignedLongBlock = (ln[n+j] as SignedLongBlock) - k;
		println!("{} - {} = {}", ln[n + j], k, t);
		ln[n+j] = t as Block;

		quot[j] = qhat as Block; // TODO: this is probably wrong??
		if t < 0 {
			quot[j] = quot[j] - 1;
			let mut k : LongBlock = 0;
			for i in 0..n {
				let s : LongBlock = (ln[i + j] as LongBlock) + (rn[i] as LongBlock) + k;
				ln[i + j] = s as Block;
				k = s >> (BLOCK_SIZE as LongBlock);
			}
			// println!("{:0b} += {}", ln[n + j], k);
			ln[n + j] = (t + (k as SignedLongBlock)) as Block;
		}
	}

	let q = pos_integer( quot);
	let r = pos_integer( ln);

	// TODO: mut shiftr XXX
	let r = r.shr_block_borrow( s, 0);

	(q, r)
}

#[inline(always)]
fn mul_b( x : LongBlock) -> LongBlock {
	x << BLOCK_SIZE
}

// Divide and mod an unsigned integer when the divisor is one block.
// From: Hacker's Delight, Second Edition by Henry Warren.
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
	let mut k = vec!(dbl_to_bl(k));

	// Reverse vector.
	q.reverse();

	(pos_integer( q), pos_integer( k))
}


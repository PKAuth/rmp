// Internal module for division.

use rmp::internal::{remove_leading_zeroes, Block, LongBlock, BLOCK_SIZE, dbl_to_bl};

// Divide and mod an unsigned integer.
pub fn div_mod_u( u : &Vec<Block>, v : &Vec<Block>) -> ( Vec<Block>, Vec<Block>) {
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


use std::ops::Mul;

use super::internal::pos_integer;
use super::{Integer, Block, LongBlock, BLOCK_SIZE};

impl Integer {
	pub fn mul_borrow( &self, rhs : &Integer) -> Integer {
		// let mut r = self.clone();
		// r.mul_mut( rhs);
		// r
		multiply( self, rhs)
	}

	/// Compute self * rhs mod base.
	// Note: Not safe against side channels. 
	pub fn mul_mod( &self, rhs : &Integer, base : &Integer) -> Integer {
		// Note: Use Montgomery reduction for mult, Barret reduction for mod?
		let r = self.modulus( base) * rhs.modulus( base);
		r.modulus( base)
	}

	// Note: Don't think a mutable version is worth it since we can't really do it in place without an allocation. 
	// pub fn mul_mut( &mut self, rhs : &Integer) {
	// 	// TODO: various algorithms dependent on the size of inputs. XXX

	// 	panic!("TODO");

	// 	// Normalize integer.
	// 	normalize_leading_zeroes( self);
	// }

	// /// Square a number.
	// pub fn sqr_mut( &mut self) {
	// 	// TODO: Optimise this due to symmetry.
	// 	// self.mul_mut( self);
	// }
}

impl Mul for Integer {
	type Output = Integer;

	fn mul(self, rhs : Integer) -> Integer {
		// let mut r = self;
		// r.mul_mut( &rhs);
		// r
		multiply( &self, &rhs)
	}
}

fn multiply( lhs : &Integer, rhs : &Integer) -> Integer {
	let negative = lhs.positive ^ rhs.positive;
	
	let ln = lhs.size();
	let rn = rhs.size();

	// TODO: various algorithms dependent on the size of inputs. XXX
	let mut res;
	if ln == rn && ln > 32 {
		res = mul_daratsuba_positives( lhs, rhs);
	}
	else {
		res = mul_base_case_positives( lhs, rhs);
	}

	if negative {
		res.neg_mut();
	}
	res
}

fn mul_daratsuba_positives( lhs : &Integer, rhs : &Integer) -> Integer {
	// println!( "{} ({}) * {} ({})", lhs, lhs.size(), rhs, rhs.size());
	// Base case.
	let lc = lhs.size();
	if lc <= 1 || rhs.size() <= 1 {
		return mul_base_case_positives( lhs, rhs);
	}

	let n = lc / 2;

	// Split arguments in half. 
	// Note: Maybe we shouldn't do this? Just work with the indices?
	let l1 = lhs.shr_block_borrow( 0, n);
	let r1 = rhs.shr_block_borrow( 0, n);

	// Lower halves.
	let mut l2 = lhs.clone();
	let mut r2 = rhs.clone();
	for _ in 0..(lc - n) {
		l2.content.pop();
		r2.content.pop();
	}

	let t1 = mul_daratsuba_positives( &l1, &r1);
	let t3 = mul_daratsuba_positives( &l2, &r2);
	let mut t2 = mul_daratsuba_positives( &l1.add_borrow( &l2), &r1.add_borrow( &r2));
	t2.sub_mut( &t1);
	t2.sub_mut( &t3);

	// Note: Can these just be copies?
	let mut res = t1.shl_block_borrow(0, 2*n);
	res.add_mut( &t2.shl_block_borrow(0, n));
	res.add_mut( &t3);

	res
}

// From: The Art of Computer Programming - Volume 2 by Knuth. Algorithm M.
fn mul_base_case_positives( lhs : &Integer, rhs : &Integer) -> Integer {
	// Init result with 0s.
	// let mut res : Vec<Block> = Vec::with_capacity( lhs.size() + rhs.size() + 1);
	// res.resize( lhs.size() + rhs.size() + 1, 0);
	let mut res : Vec<Block> = vec![0; lhs.size() + rhs.size() + 1];

	for i in 0..lhs.size() {
		let li : LongBlock = lhs.content[i] as LongBlock;

		// if li == 0, ... skip
		if li == 0 {
			continue;
		}

		let mut carry : LongBlock = 0;
		for j in 0..rhs.size() {
			let rj : LongBlock = rhs.content[j] as LongBlock;
			let t = li * rj + (res[i + j] as LongBlock) + carry; // TODO: Why do we add the carry here? XXX
			carry = t >> BLOCK_SIZE;
			res[i + j] = t as Block; // Mask upper bits.
		}
		res[i + rhs.size()] = carry as Block;
	}

	pos_integer( res)
}

// Note: Would it be better to do make this mutable, and just extend the capacity??


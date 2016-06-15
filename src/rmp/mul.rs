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
	
	// TODO: various algorithms dependent on the size of inputs. XXX
	let mut res = mul_base_case_positives( lhs, rhs);
	if negative {
		res.neg_mut();
	}
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


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
		res = mul_karatsuba_positives( lhs, rhs);
	}
	else {
		res = mul_base_case_positives( lhs, rhs);
	}

	if negative {
		res.neg_mut();
	}
	res
}


fn mul_karatsuba_positives( lhs : &Integer, rhs : &Integer) -> Integer {
	let outputSize = lhs.size()*2; 
	let mut h : Vec<Block> = vec![0; outputSize]; 
	panic!("TODO!! "); 	
}

//Dan Roche's Thesis on Space Efficient Karatsuba Multiplication pg 58
fn mul_karatsuba_helper(a : &[Block], b : &[Block], c : &[Block], 
						d : &mut [Block]){
	let k = c.len()/2; 	
	let k_2 = c.len();
	let k_3 = k+k_2; 
	let k_4 = c.len()*2; 
	//Step 4.2.1
	mul_karatsuba_step1(d, k); 
	mul_karatsuba_step2(d, a, b, k); 
	//TODO: mul_karatsuba_helper(&c[0..k], &c[k..k_2], &d[k_3-1..k_4-1], &mut d[k..k_3-1]); 
	// ---- Error is:cannot borrow `*d` as mutable because it is also borrowed
	// as immutable 
}

fn mul_karatsuba_step1(d :&mut [Block], k : usize){
	let mut carry = false; 
	
	for i in 0..k {
		let j = i + k; 
		let (mut x, p) =  d[j].overflowing_add(d[i]);
		if carry {
			let (y, e) = x.overflowing_add(1); 
			carry = e || p;
			x = y;
		}
		else{
			carry = p; 
		}
		d[j] = x; 
	}
	if carry{
		panic!("Hit carry at end of step1"); 
	}
}

fn mul_karatsuba_step2(d : &mut[Block], a : &[Block], b : &[Block], k : usize){
	let mut carry : LongBlock = 0; 
	
	for i in 0..k{
		let ik = i+k;
		let i3k = i + 3*k - 1; 
		let a_i = a[i] as LongBlock;
		let a_ik = a[ik] as LongBlock; 
		let b_i = b[i] as LongBlock; 
		let b_ik = b[ik] as LongBlock; 
		let sum = a_i + a_ik + b_i + b_ik + carry; 
		d[i3k] = sum as Block; 
		carry = sum >> BLOCK_SIZE; 
	}
	if carry !=0{
		panic!("Hit carry at end of step2"); 
	}
}

fn mul_karatsuba_addVectorMut(lhs :&mut [Block] , rhs : &[Block]){
	//for i in 0..lhs.size() {
	//	//check for overflow
	//	//TODO:: HEREEE
	//	if lhs.overflowing_add(rhs){
	//			
	//	}
	//	d[b] += d[b-k]; 
	//}
	panic!("TODO: AddVectorMut"); 
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


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

enum KCode {
	KCodeInt(Block),
	KCode1, 
	KCode2, 
	KCode3, 
	KCodeVec(Vec<Block>),
}

fn mul_karatsuba_positives( lhs : &Integer, rhs : &Integer) -> Integer {
	let mut u : Vec<Block> = Vec::new();
	let mut v : Vec<Block> = Vec::new();
	let mut c : Vec<KCode> = Vec::new();
	let mut w : Vec<Block> = Vec::new();
	let mut qTable : Vec<Block> = Vec::new();
	let mut rTable : Vec<Block> = Vec::new();

	let mut k = 1; 
	//Not sure about the constants here... ref C1 in Knuth Art of Comp Programming
	qTable.push(16); 
	qTable.push(16); 
	rTable.push(4); 
	rTable.push(4); 
	let mut Q = 4; 
	let mut R : Block = 2; 
	//C1[Compute q, r tables]
	while((qTable[k-1] + qTable[k]) < (lhs.size() as Block) * BLOCK_SIZE ){
		if((R+1).pow(2) <= Q){
			Q += R;
			R = R + 1
		}
		else{
			Q+= R; 
		}
		//q_k <- 2^Q 
		qTable.push(1 << Q);
		//r_k <- 2^R 
		rTable.push(1 << R);
		k += 1; 
	}

	//C2[Put u, v, on stack]
	c.push(KCode::KCode1); 
	//TODO: For us, k = 5 so these would need to be padded w/0's to be 9216...
	//c.push(KCode::KCodeVec(lhs.content)); 
	//c.push(KCode::KCodeVec(rhs.content)); 
	
	//C3 [Check recursion level]
	k -= 1;  
	if(k == 0){
		//TODO: Either need to implement * for Vec<Block> or find a way to
		//unwrap further into a longblock  
		//let u_tmp = c.pop().unwrap() as LongBlock;
		//let v_tmp = c.pop().unwrap() as LongBlock;
		
		//w = u_tmp * v_tmp; 
		//Going to step C10 [Return]... 
		k += 1;
		//let stackCode = c.pop().unwrap(); 
		panic!("TODO!!!");  
	}
	else {
		//set r <- r_k, q<- q_k, p <- q_(k-1_ + q_k) 
		//??: Isn't r already set to r_k and same with q as q_k???
		let p = qTable[k-1] + qTable[k]; 
		panic!("TODO: finish me up!"); 
	}

}
//fn mul_daratsuba_positives( lhs : &Vec<Block>, lStart : usize, lSize : usize,
//rhs : &Vec<Block>, rStart : usize, rSize :usize, res : &mut Vec<Block>)  {
//	// println!( "{} ({}) * {} ({})", lhs, lhs.size(), rhs, rhs.size());
//	// Base case.
//	//let lc = lhs.size();
//	if lSize <= 1 || rSize <= 1 {
//		return mul_base_case_positives( lhs, rhs);
//	}
//
//	let n1 = lSize / 2;
//	let n2 = lSize - n1 		
//
//	let l1Start = lStart + n1; 
//	let l2Start = lStart; 
//	let r1Start = rStart + n1; 
//	let r2Start = rStart; 
//	// Split arguments in half. 
//	// Note: Maybe we shouldn't do this? Just work with the indices?
//	//let l1 = lhs.shr_block_borrow( 0, n);
//	//let r1 = rhs.shr_block_borrow( 0, n);
//	
//	// Lower halves.
//	//let mut l2 = lhs.clone();
//	//let mut r2 = rhs.clone();
//	//l2.content.truncate(n); 
//	//r2.content.truncate(n); 
//	//for _ in 0..(lc - n) {
//	//	l2.content.pop();
//	//	r2.content.pop();
//	//}
//
//	let t1 = mul_daratsuba_positives( &lhs, l1Start, n1, &rhs, r1Start, n1, res);
//	let t3 = mul_daratsuba_positives( &lhs, l2Start, n2, &rhs, r2Start, n2, res);
//	let mut t2 = mul_daratsuba_positives( &l1.add_borrow( &l2), &r1.add_borrow( &r2));
//	t2.sub_mut( &t1);
//	t2.sub_mut( &t3);
//
//	// Note: Can these just be copies?
//	let mut res = t1.shl_block_borrow(0, 2*n);
//	res.add_mut( &t2.shl_block_borrow(0, n));
//	res.add_mut( &t3);
//
//	res
//}

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


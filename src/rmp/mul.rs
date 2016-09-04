use std::ops::Mul;

use super::internal::{pos_integer, ceiling, ceiling_log_two};
use super::{Integer, Block, LongBlock, BLOCK_SIZE};
static KARATSUBA_LIMIT : usize = 16;

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
	if ln == rn && ln >= KARATSUBA_LIMIT {
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

	// Memory usage justification:
	// m(1) = 2
	// m(2) = 4
	// m(3) = 6
	// m(2k) = 4k + m(k + 1)          [even]
	// m(2k-1) = 4k - 2 + m(k + 1)    [odd]
	// 
	// Recurrence:
	// m(n) ~ c + 4(n - 3) + 6 lg(n - 3) - 4
	//
	// Overapproximation:
	// m(n) ~ 4n + 6 lg(n)

	let n = lhs.size();
	let outputSize = n * 4 + 6 * ceiling_log_two( n);
	let mut h : Vec<Block> = vec![0; outputSize]; 

	mul_karatsuba_helper(&lhs.content, &rhs.content, &mut h); 

	panic!("TODO: cut off scratch space. XXX");

	pos_integer(h) 
}

fn mul_karatsuba_helper( f : &[Block], g : &[Block], d : &mut [Block]) {
	let n = f.len();

	// Check base case.
	if n < KARATSUBA_LIMIT {
		panic!("TODO: Do we need to zero memory before?");
		mul_base_case( f, g, d); // JP: Should we cut d off? 
		return;
	}

	let k = ceiling( n, 2); 

	let (f0, f1) = f.split_at( k);
	let (g0, g1) = g.split_at( k);

	// First recursive call to compute alpha.
	mul_karatsuba_helper( f0, g0, d);

	// Second recursive call to compute beta.
	mul_karatsuba_helper( f1, g1, &mut d[n..]); // TODO: Do these indices change for odd length inputs? XXX

	panic!("add/subtract some things...");

	let f_ = mul_karatsuba_add_halves( f0, f1);
	let g_ = mul_karatsuba_add_halves( g0, g1);

	mul_karatsuba_helper( &f_, &g_, &mut d[2*n..]); // TODO: Do these indices change for odd length inputs? XXX
	panic!("add/subtract some things...");
}

// Assumes f is longer than g.
// TODO: Move to output?? Use output???
fn mul_karatsuba_add_halves<'a>( f : &'a[Block], g : &'a[Block]) -> Vec<Block> {
	let res = vec![0; f.len() + 1];
	panic!("TODO");

	res
}



//Top level call in which condition 1 and 2 are not met

fn mul_karatsuba_helper_top(a : &[Block], c : &[Block],
						d : &mut [Block]){
	if c.len() < KARATSUBA_LIMIT{
		mul_base_case(a , c, d); 
		return
	}

	let k = c.len()/2; 	
	//Step 1
	mul_karatsuba_step1_top(d, c, k); 

	// Step 2
	mul_karatsuba_step2_1(d, a, k); 
	
	// Step 3
	{
		//TODO: FIX THIS!!! <<- Not sure how to handle assigning more space to
		//do the split_at_mut with variable d here
		let ( dl, dr) = d.split_at_mut( k*3 - 1);
		let ( dll, dlr) = dl.split_at_mut( k ); 
		//panic!("TODO!! Step 3 of top level call to Karatsuba_helper"); 
		//return; 
		//ASK Dan: Which arg do we omit here? 	
		mul_karatsuba_helper_top(dll, &dr[0..k], dlr); 
	}
	//Step 4
	mul_karatsuba_step4(d, k); 

	//Step 5
	mul_karatsuba_helper_top(&a[0..k], &c[0..k], &mut d[0..(2*k -1)]); 

	//Step 6
	mul_karatsuba_step6(d, k); 
	//mul_karatsuba_sub(d, 2*k, 2*k, k, k-1); 

	//Step 7
	mul_karatsuba_step7(d, k); 

	//Step 8
	mul_karatsuba_helper_1(&a[k..2*k], &c[k..2*k], &mut d[2*k..4*k-1]);

	//Step 9
	mul_karatsuba_step9(d, k); 
	//Step 10
	mul_karatsuba_step10(d, k); 
}
fn mul_karatsuba_step1_top(d : &mut [Block], c : &[Block], k: usize ){
	let mut carry = false; 
	
	for i in 0..k {
		let j = i + k; 
		let (mut x, p) =  c[j].overflowing_add(c[i]);
		if carry {
			let (y, e) = x.overflowing_add(1); 
			carry = e || p;
			x = y;
		}
		else{
			carry = p; 
		}
		d[i] = x; 
	}
	let mut i = k; 
	while carry{
		//WARNING: This may cause out-of-bounds issues
		//panic!("Carry at step1_top d[k]={}", d[k] );  
		let (res, c) = d[i].overflowing_add(1);  
		d[i] = res; 
		carry = c; 
		i += 1; 
		//panic!("Hit carry at end of step1"); 
	}

}

//Need to satisfy condition 4.1
fn mul_karatsuba_helper_1(a : &[Block], c : &[Block], 
						d : &mut [Block]){
	if c.len() < KARATSUBA_LIMIT{
		mul_base_case(a , c, d); 
		return
	}

	let k = c.len()/2; 	
	// Step 1
	mul_karatsuba_step1(d, k); 

	// Step 2
	mul_karatsuba_step2_1(d, a, k); 

	// Step 3
	{
		let ( dl, dr) = d.split_at_mut( k*3 - 1);
		mul_karatsuba_helper_1_2(&c[0..k], &c[k..k*2], &dr[0..k], &mut dl[k..]); 
	}
	//Step 4
	mul_karatsuba_step4(d, k); 

	//Step 5
	mul_karatsuba_helper_1(&a[0..k], &c[0..k], &mut d[0..(2*k -1)]); 

	//Step 6
	mul_karatsuba_step6(d, k); 
	//mul_karatsuba_sub(d, 2*k, 2*k, k, k-1); 

	//Step 7
	mul_karatsuba_step7(d, k); 

	//Step 8
	mul_karatsuba_helper_1(&a[k..2*k], &c[k..2*k], &mut d[2*k..4*k-1]);

	//Step 9
	mul_karatsuba_step9(d, k); 
	//Step 10
	mul_karatsuba_step10(d, k); 
}

fn add_blocks(a : &[Block], b: &[Block]) -> Vec<Block>{
	assert!(a.len() == b.len()); 
	//assuming they're the same len
	let mut res = vec![0; a.len()+1]; 
	let mut carry = false; 
	for i in 0..a.len() {
		let (mut x, p) =  a[i].overflowing_add(b[i]);
		if carry {
			let (y, e) = x.overflowing_add(1); 
			carry = e || p;
			x = y;
		}
		else{
			carry = p; 
		}
		res[i] = x;
	}
	if carry {	
		res[a.len()] = 1; 
	}
	res 
}


//Dan Roche's Thesis on Space Efficient Karatsuba Multiplication pg 58
fn mul_karatsuba_helper_1_2(a : &[Block], b : &[Block], c : &[Block], 
						d : &mut [Block]){
	//TODO: ADD BASE CASE 
	//Base case: Calling traditional/simple mul on small inputs
	if c.len() < KARATSUBA_LIMIT{
		let f = add_blocks(a, b);  	
		mul_base_case( &f, c, d ); 
		return
	}

	//TODO: Are these k variables worth? ie vs computing 2*k and 3*k ...
	let k = c.len()/2; 	
	//let k_2 = c.len();
	//let k_3 = k+k_2; 
	// let k_4 = c.len()*2; 

	// Step 1
	mul_karatsuba_step1(d, k); 

	// Step 2
	mul_karatsuba_step2(d, a, b, k); 

	// Step 3
	{
		let ( dl, dr) = d.split_at_mut( k*3 - 1);
		mul_karatsuba_helper_1_2(&c[0..k], &c[k..k*2], &dr[0..k], &mut dl[k..]); 
	}
	//Step 4
	mul_karatsuba_step4(d, k); 

	//Step 5
	mul_karatsuba_helper_1_2(&a[0..k], &b[0..k], &c[0..k], &mut d[0..(2*k -1)]); 

	//Step 6
	mul_karatsuba_step6(d, k); 
	//mul_karatsuba_sub(d, 2*k, 2*k, k, k-1); 

	//Step 7
	mul_karatsuba_step7(d, k); 

	//Step 8
	mul_karatsuba_helper_1_2(&a[k..2*k], &b[k..k*2], &c[k..2*k], &mut d[2*k..4*k-1]);

	//Step 9
	mul_karatsuba_step9(d, k); 
	//Step 10
	mul_karatsuba_step10(d, k); 

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

fn mul_karatsuba_step2_1(d : &mut[Block], a : &[Block], k : usize){
	let mut carry : LongBlock = 0; 
	
	for i in 0..k{
		let ik = i+k;
		let i3k = i + 3*k - 1; 
		println!("before a_i"); 
		let a_i = a[i] as LongBlock;
		println!("before a_ik"); 
		let a_ik = a[ik] as LongBlock; 
		let sum = a_i + a_ik + carry; 
		println!("i3k: {}", i3k); 
		println!("d.len: {}", d.len()); 
		d[i3k] = sum as Block; 
		carry = sum >> BLOCK_SIZE; 
	}
	let mut i = 4*k - 2; 
	while carry !=0{
		//WARNING Overflow may occur!
		let sum = d[i] as LongBlock + carry; 
		d[i] = sum as Block; 
		carry = sum >> BLOCK_SIZE; 
		//panic!("WARNING: Carry found at step2_1 d[4k-2]={}", d[4*k-2]); 
		//d[4*k-1] = carry as Block;  
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

fn mul_karatsuba_step4(d : &mut[Block], k : usize){
	let mut carry : LongBlock = 0; 
	for i in 0 .. k {
		let ik = i + k; 
		//Question for Dan: why 2k..3k-2?
		let i2k = i + 2*k; 
		let i3k = i + 3*k - 1; 
		let d_ik = d[ik] as LongBlock; 
		let d_i2k = 
			if i == (k - 1) {
				0 
			} 
			else {
				d[i2k] as LongBlock
			}; 
		let sum = d_ik + d_i2k + carry;  
		d[i3k] = sum as Block; 
		carry = sum >> BLOCK_SIZE;
	}
	if carry !=0{
		panic!("Hit carry at end of step4"); 
	}
}

fn mul_karatsuba_step6(d : &mut[Block], k : usize){
	mul_karatsuba_sub(d, 2*k, 2*k, k, k-1); 
}
fn mul_karatsuba_step7(d : &mut[Block], k : usize){
	mul_karatsuba_sub(d, k, 3*k-1, 0, k); 
}

fn mul_karatsuba_step9(d : &mut[Block], k : usize){
	mul_karatsuba_sub(d, k, k, 2*k, k); 
}
fn mul_karatsuba_step10(d : &mut[Block], k : usize){
	mul_karatsuba_sub(d, 2*k, 2*k, 3*k, k-1); 
}
fn mul_karatsuba_sub(d : &mut[Block], output_offset : usize, left_offset :
					usize, right_offset : usize, len: usize){
	let mut carry = false; 
	for i in (0 .. len){
		let left_i = i + left_offset; 
		let right_i = i + right_offset; 
		let left = d[left_i]; 
		let right = d[right_i];  
		let (mut res, resCarry) = left.overflowing_sub(right);  
		if (carry){
			let(r, rc) = res.overflowing_sub(1); 
			res = r;
			carry = rc || resCarry; 
		}
		else{
			carry = resCarry; 
		}
		d[i+output_offset] = res; 
	}
	if carry{
		panic!("It carried during subtraction!!"); 
	}

}
fn mul_base_case(lhs : &[Block] , rhs : &[Block], out : &mut[Block]){
	for i in 0 .. lhs.len(){
		let li : LongBlock = lhs[i] as LongBlock;

		// if li == 0, ... skip
		if li == 0 {
			continue;
		}
		let mut carry : LongBlock = 0; 
		for j in 0 .. rhs.len(){
			let rj : LongBlock = rhs[j] as LongBlock;
			let t = li * rj + (out[i+j] as LongBlock) + carry; 
			carry = t >> BLOCK_SIZE; 
			out[i+j] = t as Block; 
		}	
		
		println!("Before out"); 
		out[i+rhs.len()] = carry as Block;
		println!("After out"); 
	}
}

// From: The Art of Computer Programming - Volume 2 by Knuth. Algorithm M.
fn mul_base_case_positives( lhs : &Integer, rhs : &Integer) -> Integer {
	// Init result with 0s.
	// let mut res : Vec<Block> = Vec::with_capacity( lhs.size() + rhs.size() + 1);
	// res.resize( lhs.size() + rhs.size() + 1, 0);
//	let mut res : Vec<Block> = vec![0; lhs.size() + rhs.size() + 1];
//
//	for i in 0..lhs.size() {
//		let li : LongBlock = lhs.content[i] as LongBlock;
//
//		// if li == 0, ... skip
//		if li == 0 {
//			continue;
//		}
//
//		let mut carry : LongBlock = 0;
//		for j in 0..rhs.size() {
//			let rj : LongBlock = rhs.content[j] as LongBlock;
//			let t = li * rj + (res[i + j] as LongBlock) + carry; // TODO: Why do we add the carry here? XXX
//			carry = t >> BLOCK_SIZE;
//			res[i + j] = t as Block; // Mask upper bits.
//		}
//		res[i + rhs.size()] = carry as Block;
//	}
//
//	pos_integer( res)


	let mut res : Vec<Block> = vec![0; lhs.size() + rhs.size()];
	mul_base_case(&lhs.content, &rhs.content, &mut res); 
	pos_integer( res)
}

// Note: Would it be better to do make this mutable, and just extend the capacity??


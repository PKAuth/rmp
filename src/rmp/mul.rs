use std::ops::Mul;

use super::internal::{pos_integer, ceiling, ceiling_log_two, usize_is_odd};
use super::{Integer, Block, LongBlock, BLOCK_SIZE, SignedBlock};
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


// Assumes lhs and rhs are of equal length.
// TODO: Handle case for odd length.. XXX
fn mul_karatsuba_positives( lhs : &Integer, rhs : &Integer) -> Integer {
	let output_size = 2 * lhs.size();
	let mut h : Vec<Block> = vec![0; output_size];

	// TODO: other versions of karat, etc XXX

	mul_karatsuba_helper_12( &lhs.content, &vec![0; lhs.size()], &rhs.content, &mut h);

	pos_integer( h)
}

// JP: No clue if this is what we should be doing... XXX
fn mul_karatsuba_helper_12_neg( f0 : &[Block], f1 : &[Block], g : &[Block], d : &mut[Block]) -> bool {
	let n = g.len();
	let k = ceiling( n, 2); // TODO: What to do here... XXX

	{
		let _ = mul_karatsuba_negate( &mut d[0..k]);
	}

	let sign = mul_karatsuba_helper_12( f1, f0, g, d);

	mul_karatsuba_negate( d);

	!sign
}

fn mul_karatsuba_helper_12( f0 : &[Block], f1 : &[Block], g : &[Block], d : &mut[Block]) -> bool {
	let n = g.len();

	// Check base case.
	if n < KARATSUBA_LIMIT {
		let mut tmp = vec![ 0; n]; // TODO: Can we get rid of tmp's allocation??? XXX
		let negative = mul_karatsuba_subtract( f0, f1, &mut tmp);
		
		// TODO XXX
		// if negative {
		// 	let _ = mul_karatsuba_negate( &mut tmp);
		// }

		mul_base_case( &tmp, g, d);
		return negative;
	}

	let k = ceiling( n, 2); // TODO: What to do here... XXX

	let carry1 = {
		let (s0, d) = d.split_at_mut( k);
		let s1 = &mut d[0..k];
		// d[1] = h1 - h0
		mul_karatsuba_subtract_from( s1, s0)
	};

	/*
	if carry1 < 0 {
		panic!("h1 - h0 is negative (carry1)")
	}
	*/

	// JP: What do we do if negative?? XXX

	let carry2 = {
		// println!("{} {}", k, n);
		let s3 = &mut d[3*k..4*k];
		mul_karatsuba_assa( &f0[0..k], &f1[0..k], &f0[k..2*k], &f1[k..2*k], s3)
		// JP: What do we do if negative?? XXX
	};

	{
		// JP: Swap order of g's so that it's negative? How do we handle carry??? XXX
		let (d, s3) = d.split_at_mut( 3*k);
		let s23 = &mut d[k..];
		let (ga, gb) = if carry2 < 0 {
			( &g[0..k], &g[k..2*k])
		}
		else {
			( &g[k..2*k], &g[0..k])
		};

		if carry1 < 0 {
			mul_karatsuba_helper_12_neg( ga, gb, s3, s23);
		}
		else {
			mul_karatsuba_helper_12( ga, gb, s3, s23);
		}
		// TODO: sign XXX
	}

	{
		// s3 = s1 - s2
		let (s1, d) = d[k..].split_at_mut( k);
		let (s2, s3) = d.split_at_mut(k);
		let carry3 = mul_karatsuba_subtract( s1, s2, s3);
		// TODO: carry... XXX
	}

	{
		// alpha = (f0_0 - f1_0) * g_0
		let s01 = &mut d[0..2*k];
		mul_karatsuba_helper_12( &f0[0..k], &f1[0..k], &g[0..k], s01);
	}

	{
		// s2 += s1
		let (s1, d) = d[k..].split_at_mut( k);
		let s2 = &mut d[0..k];
		let carry4 = mul_karatsuba_add_to( s2, s1);
		// TODO: sign XXX
	}

	let (s0, d) = d.split_at_mut( k);
	let (s1, s23) = d.split_at_mut( k);
	{
		// s1 = s0 + s3
		let s3 = &s23[k..2*k];
		let carry5 = mul_karatsuba_add( s0, s3, s1);
		// TODO: sign XXX
	}

	// beta = (f0_1 - f1_1) * g_1
	mul_karatsuba_helper_12( &f0[k..2*k], &f1[k..2*k], &g[k..2*k], s23);

	let (s2, s3) = s23.split_at_mut( k);

	// s1 += s2
	let carry6 = mul_karatsuba_add_to( s1, s2);

	// s2 += s3
	let carry7 = mul_karatsuba_add_to( s2, s3);

	// TODO: negatives?? XXX
	false
}

// Assumes |lhs| == |rhs| == |d|.
// Returns if there is a carry.
fn mul_karatsuba_subtract(lhs : &[Block], rhs :&[Block], d : &mut [Block]) -> bool {
	let mut c = false;

	for i in 0..lhs.len() {
		let (mut x, a) = lhs[i].overflowing_sub( rhs[i]);

		if c {
			let (y, e) = x.overflowing_sub( 1);
			c = a || e;
			x = y;
		}
		else {
			c = a;
		}

		d[i] = x;
	}

	// if c {-1} else {0}
	c
}

// Assumes lhs is longer than rhs.
fn mul_karatsuba_subtract_from(lhs : &mut [Block], rhs :&[Block]) -> SignedBlock {
	let mut c = false; 
	let mut i : usize = 0; 

	while i < rhs.len() {
		let (mut x, a) = lhs[i].overflowing_sub( rhs[i]); 
		
		if c {
			let (y, e) = x.overflowing_sub( 1); 
			c = a || e; 
			x = y; 
		}
		else {
			c = a; 
		}

		lhs[i] = x; 
		i+= 1; 
	}

	while c && i < lhs.len() {
		let (x, a) = lhs[i].overflowing_sub( 1);
		lhs[i] = x;
		c = a;
		i += 1;
	}
	
	if c { -1 } else {0} 
}

// Assumes |lhs| == |rhs| == |d|.
fn mul_karatsuba_add( lhs : &mut [Block], rhs : &[Block], d : &mut [Block]) -> SignedBlock {
	let mut c = false;
	for i in 0..lhs.len() {
		let (mut x, a) = lhs[i].overflowing_add( rhs[i]);
		if c {
			let (y, e) = x.overflowing_add( 1);
			c = a || e;
			x = y;
		}
		else {
			c = a;
		}

		d[i] = x;
	}

	if c {1} else {0}
}

// Assumes lengths of all are equal.
fn mul_karatsuba_assa( w : &[Block], x : &[Block], y : &[Block], z : &[Block], d : &mut [Block]) -> SignedBlock {
	let mut carry : SignedBlock = 0;

	// println!("{} {} {} {} {}", w.len(), x.len(), y.len(), z.len(), d.len());

	for i in 0..d.len() {
		// 	d[i] = w[i] - x[i] - y[i] + z[i] + carry
		let r = carry as Block;
		if carry >= 0 {
			carry = 0;
		}
		else {
			carry = -1;
		}

		let (r, c) = r.overflowing_add( w[i]);
		if c {
			carry += 1;
		}

		let (r, c) = r.overflowing_sub( x[i]);
		if c {
			carry = carry - 1;
		}

		let (r, c) = r.overflowing_sub( y[i]);
		if c {
			carry = carry - 1;
		}

		let (r, c) = r.overflowing_add( z[i]);
		if c {
			carry += 1;
		}

		d[i] = r
	}
	
	carry
}

// Assumes lhs is longer than rhs.
fn mul_karatsuba_add_to( lhs : &mut [Block], rhs : &[Block]) -> SignedBlock {
	let mut c = false;
	let mut i : usize = 0;

	while i < rhs.len() {
		let (mut x, a) = lhs[i].overflowing_add( rhs[i]);

		if c {
			let (y, e) = x.overflowing_add( 1);
			c = a || e;
			x = y;
		}
		else {
			c = a;
		}

		lhs[i] = x;
		i += 1;
	}

	while c && i < lhs.len() {
		let (x, a) = lhs[i].overflowing_add( 1);

		lhs[i] = x;
		c = a;
		i += 1;
	}
	
	if c {1} else {0}
}

// Assumes length of num == len of output.
// Returns true if output = 0 (ie num == 0, a == b to begin with) 
fn mul_karatsuba_negate(num : &mut [Block]) -> bool {
	let mut c = true; 
	
	for i in 0 .. num.len() {
		let tmp = num[i] ^ (Block::max_value()); 
		if c {
			let (x, e) = tmp.overflowing_add(1); 
			num[i] = x; 
			c = e; 
		}
		else {
			num[i] = tmp;
		}
	}
	c 
}


/*
fn mul_karatsuba_positives( lhs : &Integer, rhs : &Integer) -> Integer {

	// Memory usage justification:
	// m(1) = 2
	// m(2) = 4
	// m(3) = 6
	// m(2k) = 6k + 2 + m(k + 1)          [even]
	// m(2k-1) = 6k - 1 + m(k + 1)        [odd]
	// 
	// Recurrence:
	// m(n) ~ c + 6(n - 3) + 11 lg(n - 3) - 6
	//
	// Overapproximation:
	// m(n) ~ 6n + 11 lg(n)

	let n = lhs.size();
	let outputSize = n * 6 + 11 * ceiling_log_two( n);
	let mut h : Vec<Block> = vec![0; outputSize]; 

	mul_karatsuba_helper(&lhs.content, &rhs.content, &mut h); 

	// Cut off scratch space.
	h.truncate( 2 * n);

	pos_integer(h) 
}

fn mul_karatsuba_helper( f : &[Block], g : &[Block], d : &mut [Block]) {
	let n = f.len();

	// Check base case.
	if n < KARATSUBA_LIMIT {
		// Zero output memory.
		for i in 0..2*n {
			d[i] = 0;
		}

		mul_base_case( f, g, d); // JP: Should we cut d off? 
		return;
	}

	let k = ceiling( n, 2); 

	let (f0, f1) = f.split_at( k);
	let (g0, g1) = g.split_at( k);

	// First recursive call to compute alpha.
	mul_karatsuba_helper( f0, g0, d);

	// Second recursive call to compute beta.
	mul_karatsuba_helper( f1, g1, &mut d[2*k..]);

	// If n is odd, output space is 4*k-2 instead of 4*k.
	let (s0, d) = d.split_at_mut( k);
	let (s1, d) = d.split_at_mut( k);
	let (s2, d) = d.split_at_mut( k);
	let (s3, d) = d.split_at_mut( if usize_is_odd( n) {k - 2} else {k});

	// Add halves of f and g.
	let (f_, d) = d.split_at_mut( k + 1);
	mul_karatsuba_add_halves( f0, f1, f_);
	let (g_, d) = d.split_at_mut( k + 1);
	mul_karatsuba_add_halves( g0, g1, g_);

	// Third recursive call to compute gamma.
	mul_karatsuba_helper( f_, g_, d);

	// Divide up gamma.
	let (g0, d) = d.split_at( k);
	let (g1, g2) = d.split_at( k);
	let g2 = &g2[0..2];

	// Compute alpha1 - beta0 in second slot.
	let carry1 = mul_karatsuba_subtract_from( s1, s2);  

	// Compute -( alpha1 - beta0) in third slot.
	let is_2_zero = mul_karatsuba_negate( s1, s2);  

	// carry2 is negative if beta0 < alpha1.
	let carry2 = if carry1 == 0 && !is_2_zero { -1} else {0};
	
	// Subtract alpha0 from s1.
	let carry1 = carry1 + mul_karatsuba_subtract_from( s1, s0);

	// Add gamma0 to s1.
	let carry1 = carry1 + mul_karatsuba_add_to( s1, g0);

	// Subtract beta1 from s2.
	let carry2 = carry2 + mul_karatsuba_subtract_from( s2, s3);

	// Add gamma1 to s2.
	let carry2 = carry2 + mul_karatsuba_add_to( s2, g1);

	// Add gamma2 to s3.
	let carry3 = mul_karatsuba_add_to( s3, g2);

	// Add carries.
	let carry2 = carry2 + mul_karatsuba_add_carry( s2, carry1);
	let carry3 = carry3 + mul_karatsuba_add_carry( s3, carry2);

	assert!( carry3 == 0); // TODO: This should always be true.. Remove eventually.. XXX
}

// Assumes length of num == len of output.
// Returns true if output = 0 (ie num == 0, a == b to begin with) 
fn mul_karatsuba_negate(num : &[Block], output : &mut[Block]) -> bool {
	let mut c = true; 
	
	for i in 0 .. num.len() {
		let tmp = num[i] ^ (Block::max_value()); 
		if c {
			let (x, e) = tmp.overflowing_add(1); 
			output[i] = x; 
			c = e; 
		}
		else {
			output[i] = tmp;
		}
	}
	c 
}

// Assumes f is longer than g.
fn mul_karatsuba_add_halves( f : &[Block], g : &[Block], d : &mut [Block]) {
	let mut c = false;
	let mut i : usize = 0;

	while i < g.len() {
		let (mut x, a) = f[i].overflowing_add( g[i]);

		if c {
			let (y, e) = x.overflowing_add( 1);
			c = a || e;
			x = y;
		}
		else {
			c = a;
		}

		d[i] = x;

		i += 1;
	}

	while i < f.len() {
		if c {
			let (x, a) = f[i].overflowing_add( 1);

			d[i] = x;
			c = a;
		}
		else {
			d[i] = f[i];
			// c is always false. 
			// c = false;
		}

		i += 1;
	}

	d[i] = if c {1} else {0};
}

fn mul_karatsuba_add_carry( v : &mut [Block], carry : SignedBlock) -> SignedBlock {
	let mut c = carry;

	for i in 0..v.len() {
		// Zero.
		if c == 0 {
			return 0
		}

		// Positive.
		else if c > 0 {
			let (x, a) = v[i].overflowing_add( c as Block);
			v[i] = x;
			c = if a {1} else {0};
		}

		// Negative.
		else {
			let (x, a) = v[i].overflowing_sub( (-1 * c) as Block);
			v[i] = x;
			c = if a {-1} else {0};
		}
	}

	c
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
*/

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
		
		out[i+rhs.len()] = carry as Block;
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


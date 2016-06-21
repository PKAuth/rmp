extern crate rand;

use std::cmp::{min};
use std::iter::Iterator;
use std::ops::{Neg};
use self::rand::OsRng;
// use std::rand::OsRng;

pub type MulMod = Box<Fn(&Integer, &Integer) -> Integer>;
pub type RMod = Box<Fn(&Integer) -> Integer>;

use super::{Integer, Block, BLOCK_SIZE, SignedBlock};
use super::internal::{get_bit, get_bits, get_zero};

impl Integer {
	/// Generate a prime number of size BLOCK_SIZE * b;
	pub fn generate_prime( b : usize, rng : &mut OsRng) -> Integer {
		let mut p = Integer::random_blocks( b, rng);

		if p.is_even() {
			let i1 = Integer::from( 1);
			p.add_mut( &i1);
		}

		let i2 = Integer::from( 2);
		// let mut c = 0;
		while !p.is_probably_prime( rng) { // && c < 100 { // 
			// println!("Testing: {}", p);
			p.add_mut( &i2);
			// c += 1;
		}

		p
	}

	/// Determine whether the integer is probably prime. 
	pub fn is_probably_prime(&self, rng : &mut OsRng) -> bool {
		// Note: Check for negatives and evens; less than 4?

		// Check if integer is multiple of the earliest primes.
		if FIRST_PRIMES.iter().any( |fp| {let p = Integer::from( *fp); p < *self && self.is_multiple_of( &p)}) {
			return false
		}

		// Fermat primality test.
		if !self.fermat_primality_test( 1, rng) {
			// println!("Failed");
			return false
		}

		if let Some(muls) = Integer::montgomery_multiplication( self.clone()) {
			// Miller Rabin primality test.
			// According to Table 4.4 of Handbook of Applied Cryptography, we only need to do 2 rounds.
			self.miller_rabin_primality_test( 4, muls, rng)
		}
		else {
			false
		}
	}

	/// Perform (self ^ power mod base).
	pub fn exp_mod( &self, power : &Integer, r : &RMod, f : &MulMod) -> Integer {
		// TODO: Choose some k based on the power! XXX
		let k = 4;

		self.sliding_exp_mod( power, k, r, f)
	}

	// Algorithm 14.85 from Handbook of Applied Cryptography.
	// k should be > 2, < BLOCK_SIZE
	fn sliding_exp_mod( &self, e : &Integer, k : usize, mul_r : &RMod, mul_mod : &MulMod) -> Integer {
		fn longest_bitstring( e : &Vec<Block>, b : usize, i : Block, k : Block) -> ( Block, usize) {
			let mut c = min( i + 1, k);

			// Get c bits from e[b].
			let mut str = get_bits( e[b], i, c);

			// Check if we need next block.
			if b != 0 && k != c {
				let second_c = k - c;
				str = str << second_c | get_bits( e[b-1], 31, second_c);

				c = k;
			}

			while c > 1 {
				if str & 1 == 1 {
					break;
				}

				str = str >> 1;
				c = c - 1;
			}

			(c, str as usize)
		}

		// Note: Probably can improve this.
		// a ^ (2 ^ e) mod base
		fn exp_2_exp_mod( a : &Integer, e : Block, mul_mod : &MulMod) -> Integer {
			let mut res = a.clone();
			for _ in 0..e {
				res = mul_mod( &res, &res);
			}
			res
		}

		// Note: Could compact this as we're not using the even ones, but that'd require an extra division by 2.
		let i0 = Integer::from( 0);
		let mut g = vec![ i0; 1 << k]; 

		// Precompute g.
		g[1] = self.clone(); // Note: Do we need to clone this?
		g[2] = mul_mod( &self, &g[1]);
		for i in 1..(1 << (k - 1)) { // 1 .. 2^(k-1)-1
			g[2*i + 1] = mul_mod( &g[2*i - 1], &g[2]);
		}

		let mut a = mul_r( &Integer::from( 1));

		// println!("self: {}", self);
		// println!("e: {}", e);

		// Iterate over bits of e.
		let mut i : SignedBlock = (BLOCK_SIZE - e.leading_zeros() - 1) as SignedBlock; // 0 <= i <= 31
		for b in (0..e.size()).rev() {
			while i >= 0 {
				let e_i = get_bit( e.content[b], i as Block);
				// println!("{:0b}", e.content[b]);
				// println!("{}th bit: {}", i, e_i);

				if e_i == 0 {
					a = mul_mod( &a, &a); // Note: Square this eventually.
					i = i - 1;
				}
				else if e_i == 1 {
					let (len, str) = longest_bitstring( &e.content, b, i as Block, k as Block);
					// println!("longest bs ({}): {}", len, str);
					a = mul_mod( &exp_2_exp_mod( &a, len, &mul_mod), &g[str]);
					// println!("g[{}]: {}", str, g[str]);
					// println!("a: {}", a);
					i = i - (len as SignedBlock);
				}
				else {
					panic!("sliding_exp_mod: bit is not 0 or 1.")
				}
			}

			// Reset i for next block.
			i = i + (BLOCK_SIZE as SignedBlock);
		}
		
		a
	}

	/*
	// Compute self * 2^k mod base
	fn exp_pow2_mod( &self, k : Block, base : &Integer) -> Integer {
		// Note: Can this be improved??
		let x : Integer = self.shl_borrow( &Integer::from( k));
		x.modulus( base)
	}
	*/

	/// Perform fermat primality test k times. Will always produce false positives for carmichael numbers. Input must be greater than 4.
	fn fermat_primality_test( &self, k : usize, rng : &mut OsRng) -> bool {
		if k <= 0 {
			return true
		}

		let i1 : Integer = Integer::from( 1);
		let i2 : Integer = Integer::from( 2);
		let i3 : Integer = Integer::from( 3);

		// Generate a in [2,p-2]
		// let a = Integer::random( self.sub_borrow( &i3), rng).add_borrow( &i2);

		// if a.exp_mod( &self.sub_borrow( &i1), self) != i1 {
		// 	// println!("Failed with: {}, {}, {}", a, self, a.exp_mod( &self.sub_borrow( &i1), self));
		// 	false
		// }
		// else {
		// 	self.fermat_primality_test( k - 1, rng)
		// }
		true // TODO: undo this XXX
	}

	// Input must be greater than 3.
	fn miller_rabin_primality_test( &self, k : usize, (mul_r, mul_mod) : (Box<Fn(&Integer) -> Integer>, MulMod), rng : &mut OsRng) -> bool {
		// Check if even.
		if self.is_even() {
			return false
		}

		let i1 : Integer = Integer::from( 1);
		let i2 : Integer = Integer::from( 2);
		let i3 : Integer = Integer::from( 3);

		// Get r and d where self-1 = d*2^r. 
		let nm1 = self.sub_borrow( &i1);
		let r = nm1.trailing_zeros();
		let d = nm1.shr_borrow( &r);

		// Repeat k times.
		'outer: for _ in 0..k {
			// Generate a in [2,p-2]
			let a = mul_r( &Integer::random( self.sub_borrow( &i3), rng).add_borrow( &i2)); // TODO: finish mul_r

			let mut x = a.exp_mod( &d, &mul_r, &mul_mod);
			let c = mul_mod( &x, &i1);
			if c == i1 || c == nm1 { // TODO: update these... XXX
				continue;
			}

			// Repeat r - 1 times.
			let mut j = i1.clone();
			while j < r {
				// x = x.mul_borrow( &x).modulus( self); // TODO: use sqr_mut XXX
				x = mul_mod( &x, &x); // TODO: use sqr_mut XXX
				let c = mul_mod( &x, &i1);
				if c == i1 { // TODO: update these... XXX
					return false
				}
				else if c == nm1 { // TODO: update these... XXX
					continue 'outer
				}

				j.add_mut( &i1);
			}

			return false
		}

		return true
	}

	// Returns a function that computes x*r mod m, and a function that computes x*y*r^-1 mod m using montgomery reduction.
	pub fn montgomery_multiplication( m : Integer) -> Option<(Box<Fn(&Integer) -> Integer>, Box<Fn(&Integer, &Integer) -> Integer>)> {
		let block_size = Integer::from(BLOCK_SIZE);
		let b = Integer::from(1).shl_borrow( &block_size);
		let n = m.size();
		if let Some( mp) = m.multiplicative_inverse( &b).map(|x| {x.neg() + b}) {
			let shift_c = Integer::from( n).mul_borrow( &block_size);
			let mc = m.clone();
			let mul_f = Box::new(move |x : &Integer| {
				x.shl_borrow( &shift_c).modulus( &mc)
				// TODO: use barret reduction?? XXX
			});

			let mp0 = get_zero( &mp, 0);
			// println!("mp: {}", mp);
			let f = Box::new( move |x : &Integer, y : &Integer| {
				let mut a = Integer::from( 0);
				let y0 = get_zero( &y, 0);

				for i in 0..n {
					let a0 = get_zero( &a, 0);
					let xi = get_zero( &x, i);
					let xi_y0 : Block = xi.overflowing_mul( y0).0;
					let u : Block = a0.overflowing_add( xi_y0).0.overflowing_mul( mp0).0; // Can use overflowing ops since everything is mod b anyways.
					let xi_y = Integer::from( xi).mul_borrow( &y);
					a.add_mut( &xi_y);
					let u_m = Integer::from( u).mul_borrow( &m);
					a.add_mut( &u_m);
					
					// TODO: use a mutable shift. XXX
					a = a.shr_borrow( &block_size);

					// println!("{} {} {} {} {} {} {}", i, xi, xi_y0, u, xi_y, u_m, a)
				}

				if a >= m {
					a.sub_borrow( &m)
				}
				else {
					a
				}
			});

			Some(( mul_f, f))
		}
		else {
			None
		}
	}


	// Compute the multiplicative inverse of self modulus m.
	pub fn multiplicative_inverse( &self, m : &Integer) -> Option<Integer> {
		// TODO: What if self >= m? Panic? XXX

		let (_, mut b, gcd) = Integer::extended_gcd( m, self);

		// If gcd is not one, self does not have a multiplicative inverse mod m.
		if !gcd.is_one() {
			return None
		}

		// If b is negative, make it mod m.
		if b.is_negative() {
			b.add_mut( m);
		}
		
		Some( b)
	}

	/// Computes a, b, gcd(x, y) in a*x + b*y = gcd( x, y).
	// Algorithm 14.61 from Handbook of Applied Cryptography.
	pub fn extended_gcd( x : &Integer, y : &Integer) -> (Integer, Integer, Integer) {
		// Divide x and y by 2 while either are even.
		let end_c = min( x.trailing_zeros(), y.trailing_zeros());
		let i0 : Integer = Integer::from( 0);
		let i1 : Integer = Integer::from( 1);
		// let g = i1.shl_borrow( &end_c);
		let x = x.shr_borrow( &end_c); // Note: use shr_block_borrow??
		let y = y.shr_borrow( &end_c);
	
		let mut u = x.clone();
		let mut v = y.clone();
	
		let mut a = i1.clone();
		let mut b = i0.clone();
		let mut c = i0.clone();
		let mut d = i1.clone();

		// println!("{} {} {} {} {} {}", u, v, a, b, c, d);
	
		loop {
			// Note: count training bits instead? Could save some allocations. 
			while u.is_even() {
				u = u.shr_block_borrow( 1, 0);
	
				if a.is_odd() || b.is_odd() {
					a.add_mut( &y);
					b.sub_mut( &x);
				}
	
				a = a.shr_block_borrow( 1, 0); // Note: Make these mutable.
				b = b.shr_block_borrow( 1, 0);
				// println!("{} {} {} {} {} {}", u, v, a, b, c, d);
			}
	
			while v.is_even() {
				v = v.shr_block_borrow( 1, 0);
	
				if c.is_odd() || d.is_odd() {
					c.add_mut( &y);
					d.sub_mut( &x);
				}
	
			 	c = c.shr_block_borrow( 1, 0);
			 	d = d.shr_block_borrow( 1, 0);
				// println!("{} {} {} {} {} {}", u, v, a, b, c, d);
			}
	
			if u >= v {
				u.sub_mut( &v);
				a.sub_mut( &c);
				b.sub_mut( &d);

				if b > i0 {
					a.add_mut( &y);
					b.sub_mut( &x);
				}
			}
			else {
				v.sub_mut( &u);
				c.sub_mut( &a);
				d.sub_mut( &b);

				if d > i0 {
					c.add_mut( &y);
					d.sub_mut( &x);
				}
			}
	
			// println!("{} {} {} {} {} {}", u, v, a, b, c, d);
			if u.is_zero() {
				break;
			}
		}
	
		let gcd = v.shl_borrow( &end_c);
		( c, d, gcd)
	}
// From errata:
// Page 610, Note 14.64: When Algorithm 14.61 terminates, it may not be the case that |D| < m, so it is not guaranteed that z lies in the interval [0,m-1]. The following changes guarantee that z lies in [0,m-1].
// (1) At the end of the first line of step 6 of Algorithm 14.61, add "If B>0 then A <-- A+y and B <-- B-x.
// (2) At the end of the second line of step 6 of Algorithm 14.61, add " If D>0 then C <-- C+y and D <-- D-x.
// (3) When Algorithm 14.61 terminates, set z=D+m if D<0, and z=D otherwise.

// Note: We probably don't need to compute a and c.

}


// fn applyn(n: int) -> fn@(fn@(i: int)) {
//     return |f| {
//         let mut i = 0;
//         while i < n {
//           f(i);
//           i += 1;
//         }
//     };
// }
// 
// fn main() {
//     let apply10 = applyn(10);
//     apply10(|i| { io::println(i.to_str()); });
// }


// From: https://primes.utm.edu/lists/small/1000.txt
// TODO: Can this be improved? Make a [Integer] instead? XXX
//static first_primes : [Integer] = [2_u64].iter().map(|p| Integer::from(*p));
const FIRST_PRIMES : [u32; 170] = [
   2u32,     3,     5,     7,    11,    13,    17,    19,    23,    29,
     31,    37,    41,    43,    47,    53,    59,    61,    67,    71,
     73,    79,    83,    89,    97,   101,   103,   107,   109,   113,
    127,   131,   137,   139,   149,   151,   157,   163,   167,   173,
    179,   181,   191,   193,   197,   199,   211,   223,   227,   229,
    233,   239,   241,   251,   257,   263,   269,   271,   277,   281,
    283,   293,   307,   311,   313,   317,   331,   337,   347,   349,
    353,   359,   367,   373,   379,   383,   389,   397,   401,   409,
    419,   421,   431,   433,   439,   443,   449,   457,   461,   463,
    467,   479,   487,   491,   499,   503,   509,   521,   523,   541,
    547,   557,   563,   569,   571,   577,   587,   593,   599,   601,
    607,   613,   617,   619,   631,   641,   643,   647,   653,   659,
    661,   673,   677,   683,   691,   701,   709,   719,   727,   733,
    739,   743,   751,   757,   761,   769,   773,   787,   797,   809,
    811,   821,   823,   827,   829,   839,   853,   857,   859,   863,
    877,   881,   883,   887,   907,   911,   919,   929,   937,   941,
    947,   953,   967,   971,   977,   983,   991,   997,  1009,  1013,
  ];

//    1019,  1021,  1031,  1033,  1039,  1049,  1051,  1061,  1063,  1069,
//    1087,  1091,  1093,  1097,  1103,  1109,  1117,  1123,  1129,  1151,
//    1153,  1163,  1171,  1181,  1187,  1193,  1201,  1213,  1217,  1223,
//    1229,  1231,  1237,  1249,  1259,  1277,  1279,  1283,  1289,  1291,
//    1297,  1301,  1303,  1307,  1319,  1321,  1327,  1361,  1367,  1373,
//    1381,  1399,  1409,  1423,  1427,  1429,  1433,  1439,  1447,  1451,
//    1453,  1459,  1471,  1481,  1483,  1487,  1489,  1493,  1499,  1511,
//    1523,  1531,  1543,  1549,  1553,  1559,  1567,  1571,  1579,  1583,
//    1597,  1601,  1607,  1609,  1613,  1619,  1621,  1627,  1637,  1657,
//    1663,  1667,  1669,  1693,  1697,  1699,  1709,  1721,  1723,  1733,
//    1741,  1747,  1753,  1759,  1777,  1783,  1787,  1789,  1801,  1811,
//    1823,  1831,  1847,  1861,  1867,  1871,  1873,  1877,  1879,  1889,
//    1901,  1907,  1913,  1931,  1933,  1949,  1951,  1973,  1979,  1987,
//    1993,  1997,  1999,  2003,  2011,  2017,  2027,  2029,  2039,  2053,
//    2063,  2069,  2081,  2083,  2087,  2089,  2099,  2111,  2113,  2129,
//    2131,  2137,  2141,  2143,  2153,  2161,  2179,  2203,  2207,  2213,
//    2221,  2237,  2239,  2243,  2251,  2267,  2269,  2273,  2281,  2287,
//    2293,  2297,  2309,  2311,  2333,  2339,  2341,  2347,  2351,  2357,
//    2371,  2377,  2381,  2383,  2389,  2393,  2399,  2411,  2417,  2423,
//    2437,  2441,  2447,  2459,  2467,  2473,  2477,  2503,  2521,  2531,
//    2539,  2543,  2549,  2551,  2557,  2579,  2591,  2593,  2609,  2617

//   2621,  2633,  2647,  2657,  2659,  2663,  2671,  2677,  2683,  2687,
//   2689,  2693,  2699,  2707,  2711,  2713,  2719,  2729,  2731,  2741,
//   2749,  2753,  2767,  2777,  2789,  2791,  2797,  2801,  2803,  2819,
//   2833,  2837,  2843,  2851,  2857,  2861,  2879,  2887,  2897,  2903,
//   2909,  2917,  2927,  2939,  2953,  2957,  2963,  2969,  2971,  2999,
//   3001,  3011,  3019,  3023,  3037,  3041,  3049,  3061,  3067,  3079,
//   3083,  3089,  3109,  3119,  3121,  3137,  3163,  3167,  3169,  3181,
//   3187,  3191,  3203,  3209,  3217,  3221,  3229,  3251,  3253,  3257,
//   3259,  3271,  3299,  3301,  3307,  3313,  3319,  3323,  3329,  3331,
//   3343,  3347,  3359,  3361,  3371,  3373,  3389,  3391,  3407,  3413,
//   3433,  3449,  3457,  3461,  3463,  3467,  3469,  3491,  3499,  3511,
//   3517,  3527,  3529,  3533,  3539,  3541,  3547,  3557,  3559,  3571,
//   3581,  3583,  3593,  3607,  3613,  3617,  3623,  3631,  3637,  3643,
//   3659,  3671,  3673,  3677,  3691,  3697,  3701,  3709,  3719,  3727,
//   3733,  3739,  3761,  3767,  3769,  3779,  3793,  3797,  3803,  3821,
//   3823,  3833,  3847,  3851,  3853,  3863,  3877,  3881,  3889,  3907,
//   3911,  3917,  3919,  3923,  3929,  3931,  3943,  3947,  3967,  3989,
//   4001,  4003,  4007,  4013,  4019,  4021,  4027,  4049,  4051,  4057,
//   4073,  4079,  4091,  4093,  4099,  4111,  4127,  4129,  4133,  4139,
//   4153,  4157,  4159,  4177,  4201,  4211,  4217,  4219,  4229,  4231,
//   4241,  4243,  4253,  4259,  4261,  4271,  4273,  4283,  4289,  4297,
//   4327,  4337,  4339,  4349,  4357,  4363,  4373,  4391,  4397,  4409,
//   4421,  4423,  4441,  4447,  4451,  4457,  4463,  4481,  4483,  4493,
//   4507,  4513,  4517,  4519,  4523,  4547,  4549,  4561,  4567,  4583,
//   4591,  4597,  4603,  4621,  4637,  4639,  4643,  4649,  4651,  4657,
//   4663,  4673,  4679,  4691,  4703,  4721,  4723,  4729,  4733,  4751,
//   4759,  4783,  4787,  4789,  4793,  4799,  4801,  4813,  4817,  4831,
//   4861,  4871,  4877,  4889,  4903,  4909,  4919,  4931,  4933,  4937,
//   4943,  4951,  4957,  4967,  4969,  4973,  4987,  4993,  4999,  5003,
//   5009,  5011,  5021,  5023,  5039,  5051,  5059,  5077,  5081,  5087,
//   5099,  5101,  5107,  5113,  5119,  5147,  5153,  5167,  5171,  5179,
//   5189,  5197,  5209,  5227,  5231,  5233,  5237,  5261,  5273,  5279,
//   5281,  5297,  5303,  5309,  5323,  5333,  5347,  5351,  5381,  5387,
//   5393,  5399,  5407,  5413,  5417,  5419,  5431,  5437,  5441,  5443,
//   5449,  5471,  5477,  5479,  5483,  5501,  5503,  5507,  5519,  5521,
//   5527,  5531,  5557,  5563,  5569,  5573,  5581,  5591,  5623,  5639,
//   5641,  5647,  5651,  5653,  5657,  5659,  5669,  5683,  5689,  5693,
//   5701,  5711,  5717,  5737,  5741,  5743,  5749,  5779,  5783,  5791,
//   5801,  5807,  5813,  5821,  5827,  5839,  5843,  5849,  5851,  5857,
//   5861,  5867,  5869,  5879,  5881,  5897,  5903,  5923,  5927,  5939,
//   5953,  5981,  5987,  6007,  6011,  6029,  6037,  6043,  6047,  6053,
//   6067,  6073,  6079,  6089,  6091,  6101,  6113,  6121,  6131,  6133,
//   6143,  6151,  6163,  6173,  6197,  6199,  6203,  6211,  6217,  6221,
//   6229,  6247,  6257,  6263,  6269,  6271,  6277,  6287,  6299,  6301,
//   6311,  6317,  6323,  6329,  6337,  6343,  6353,  6359,  6361,  6367,
//   6373,  6379,  6389,  6397,  6421,  6427,  6449,  6451,  6469,  6473,
//   6481,  6491,  6521,  6529,  6547,  6551,  6553,  6563,  6569,  6571,
//   6577,  6581,  6599,  6607,  6619,  6637,  6653,  6659,  6661,  6673,
//   6679,  6689,  6691,  6701,  6703,  6709,  6719,  6733,  6737,  6761,
//   6763,  6779,  6781,  6791,  6793,  6803,  6823,  6827,  6829,  6833,
//   6841,  6857,  6863,  6869,  6871,  6883,  6899,  6907,  6911,  6917,
//   6947,  6949,  6959,  6961,  6967,  6971,  6977,  6983,  6991,  6997,
//   7001,  7013,  7019,  7027,  7039,  7043,  7057,  7069,  7079,  7103,
//   7109,  7121,  7127,  7129,  7151,  7159,  7177,  7187,  7193,  7207,
//   7211,  7213,  7219,  7229,  7237,  7243,  7247,  7253,  7283,  7297,
//   7307,  7309,  7321,  7331,  7333,  7349,  7351,  7369,  7393,  7411,
//   7417,  7433,  7451,  7457,  7459,  7477,  7481,  7487,  7489,  7499,
//   7507,  7517,  7523,  7529,  7537,  7541,  7547,  7549,  7559,  7561,
//   7573,  7577,  7583,  7589,  7591,  7603,  7607,  7621,  7639,  7643,
//   7649,  7669,  7673,  7681,  7687,  7691,  7699,  7703,  7717,  7723,
//   7727,  7741,  7753,  7757,  7759,  7789,  7793,  7817,  7823,  7829,
//   7841,  7853,  7867,  7873,  7877,  7879,  7883,  7901,  7907,  7919

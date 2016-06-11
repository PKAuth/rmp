extern crate rand;

use self::rand::{OsRng, Rng};

use super::{Integer, Block};
use super::internal::pos_integer;

impl Integer {
	/// Generate a uniformly random integer on [0, n).
	pub fn random( n : Integer, rng : &mut OsRng) -> Integer {
		// Note: Check if n <= 0?

		// Generate random blocks.
		let mut b = random_vec_blocks( n.size(), rng);

		// Mask out upper bits from n.
		let end = n.size() - 1;
		let num_zeros = n.content[end].leading_zeros();
		let mask = Block::max_value() >> num_zeros; // Assumes msb of n isn't 0.

		b[end] = b[end] & mask;

		let r = pos_integer(b);

		// Check if r is 
		if r < n {
			r
		}
		else {
			Integer::random( n, rng)
		}
	}

	/// Generate a uniformly random Integer with n blocks.
	pub fn random_blocks( b : usize, rng : &mut OsRng) -> Integer {
		pos_integer( random_vec_blocks( b, rng))
	}

}

// Generate a uniformly random vector with n blocks. 
fn random_vec_blocks( n : usize, rng : &mut OsRng) -> Vec<Block> {
	let mut r = Vec::with_capacity( n);

	for i in 0..n {
		r.push( rng.gen());
	}

	r
}

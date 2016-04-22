// Internal module for division.

// use std::u32::{wrapping_mul, wrapping_div};

use rmp::internal::{remove_leading_zeroes};

// Divide and mod an unsigned integer.
pub fn div_mod_u( u : &Vec<u32>, v : &Vec<u32>) -> ( Vec<u32>, Vec<u32>) {
	// Check for Nx1.
	if v.len() == 1 {
		return div_mod_u_n_1( u, v[0])
	}

	panic!("TODO")
}

fn div_mod_u_n_1( u : &Vec<u32>, v : u32) -> ( Vec<u32>, Vec<u32>) {
	let mut q = Vec::with_capacity( u.capacity());
	let mut k : u32 = 0;
	// let b = 1u32.rotate_right(1);

	// 2x1 division.
	// for j in (0..u.len()).rev() {
	// 	let kb_uj = k.wrapping_mul( b).wrapping_add( u[j]);
	// 	let qj = kb_uj.wrapping_div( v);

	// 	q.push(qj);
	// 	k = kb_uj.wrapping_sub( qj.wrapping_mul( v));
	// }

	// Reverse vector.
	// q.reverse();

	// Remove leading 0s.
	// remove_leading_zeroes( &mut q);
	
	(q, vec![k])
}

// const b : u32 = u32::max_value();

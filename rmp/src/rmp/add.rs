use std::cmp::{max, min};
use std::ops::{Add, Neg};

use super::internal::{pos_integer, neg_integer, normalize_leading_zeroes};
use super::{Integer, Block};
// use super::sign;

impl Integer {
	// Borrowed addition.
	pub fn add_borrow( &self, rhs : &Integer) -> Integer {
		let mut r = self.clone();
		r.add_mut( rhs);
		r
	}

	// Mutable borrowed addition.
	pub fn add_mut( &mut self, rhs : &Integer) {
		match ( self.is_positive(), rhs.is_positive()) {
			(true, true) => {
				add_positives_mut( self, rhs);
			},
			(false, false) => {
				add_positives_mut( self, rhs);
			},
			(true, false) => {
				// Set sign.
				self.neg_mut();
				let swap = (self as &Integer) <= rhs;
				self.positive = swap;

				sub_positives_mut( self, rhs, swap);
			},
			(false, true) => {
				// Set sign.
				self.neg_mut();
				let swap = (self as &Integer) >= rhs;
				self.positive = (self as &Integer) <= rhs;

				sub_positives_mut( self, rhs, swap);
			},
		}
	}
}

impl Add for Integer {
	type Output = Integer;

	fn add( self, rhs : Integer) -> Integer {
		let mut r = self;
		r.add_mut( &rhs);
		r
	}
}

fn sub_positives_mut( lhs : &mut Integer, rhs : &Integer, swap : bool) {
	#[inline(always)]
	fn get( x : &Integer, i : usize) -> Block {
		match x.content.get(i) {
			Some( b) => *b,
			Nothing => 0,
		}
	}

	let mut c = false;
	let end = max( lhs.size(), rhs.size());

	for i in 0..end {
		let lit = get( lhs, i);
		let rit = get( rhs, i);

		// Swap if negative.
		let ( li, ri) = if swap {( lit, rit)} else {( rit, lit)};

		let (mut x, d) = li.overflowing_sub( ri);
		if c {
			let ( y, e) = x.overflowing_sub( 1);
			c = d || e;
			x = y;
		}
		else {
			c = d
		}

		// We can stop if we are past the shortest arg and there is no carry and x is 0.
		// if i >= ??? && !c && x == 0 {
		// 	panic("TODO: Check if swap. Copy over blocks (including i)")
		// 	if swap {
		// 		lhs.extend( &rhs[i..]);
		// 	}

		// 	break;
		// };

		if i < lhs.size() {
			lhs.content[i] = x;
		}
		else {
			lhs.content.push( x);
		}
	}

	// Normalize integer.
	normalize_leading_zeroes( lhs);
}

fn add_positives_mut( lhs : &mut Integer, rhs : &Integer) {
	let mut c = false;
	let mut i = 0;
	let self_s = lhs.size();
	let rhs_s = rhs.size();
	let s = min( self_s, rhs_s);
	while i < s {
		let (mut x, d) = lhs.content[i].overflowing_add( rhs.content[i]);
	
		if c {
			let (y, e) = x.overflowing_add( 1);
			c = d || e;
			x = y;
		}
		else {
			c = d;
		}

		if i < self_s {
			lhs.content[i] = x;
		}
		else {
			lhs.content.push( x);
		}

		i += 1;
	}

	// If rhs length is longer than lhs length, copy over remaining blocks. 
	if rhs_s > self_s {
		lhs.content.extend( &rhs.content[i..]);
	}

	// Finish any carries.
	let self_s = lhs.size();
	while c && i < self_s {
		let (y, e) = lhs.content[i].overflowing_add( 1);
		lhs.content[i] = y;
		c = e;

		i += 1;
	}

	// Check for last carry.
	if c {
		lhs.content.push( 1);
	}

	// Normalize integer.
	normalize_leading_zeroes( lhs);
}


use std::cmp::{max, min};
use std::ops::{Add, Neg};

use super::internal::{pos_integer, neg_integer};
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
				panic!("TODO")
			},
			(false, true) => {
				panic!("TODO")
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
		// match ( self.is_positive(), rhs.is_positive()) {
		// 	(true, true) => {
		// 		let cs = add_positives( self, rhs);
		// 		pos_integer( cs)
		// 	},
		// 	(false, false) => {
		// 		let cs = add_positives( self, rhs);
		// 		neg_integer(cs)
		// 	},
		// 	(true, false) =>
		// 		sub_positives( &self, &rhs.neg()),
		// 	(false, true) =>
		// 		sub_positives( &rhs, &self.neg()),
		// }
	}
}

fn sub_positives( l : &Integer, r : &Integer) -> Integer {
	let (b, s) = if l > r {(l, r)} else {(r, l)};

	let mut cs = Vec::with_capacity( b.capacity()); // TODO: Improve this capacity?? Do we need to zero out memory? XXX
	let mut c = false;
	let mut i = 0;
	while i < s.size() {
		let (mut x, d) = b.content[i].overflowing_sub( s.content[i]);
		
		if c {
			let ( y, e) = x.overflowing_sub( 1);
			c = d || e;
			x = y;
		}
		else {
			c = d;
		}

		cs.push( x);
		i += 1;
	}

	while i < b.size() {
		let (x, e) = if c {b.content[i].overflowing_sub( 1)} else {(b.content[i], false)};
		c = e;

		// We can stop if there is no carry and x is 0.
		if !c && x == 0 {
			break;
		}

		cs.push( x);
		i += 1;
	}

	if l > r {
		pos_integer( cs)
	}
	else {
		neg_integer( cs)
	}
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
}


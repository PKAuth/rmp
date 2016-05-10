use std::cmp::{max, min};
use std::ops::{Add, Neg};

use super::internal::{pos_integer, neg_integer};
use super::{Integer, Block};
// use super::sign;

impl Integer {
	pub fn plus( &self, rhs : &Integer) -> Integer {
		panic!("TODO")
	}
}

impl Add for Integer {
	type Output = Integer;

	fn add( self, rhs : Integer) -> Integer {
		fn add_positives( lhs : Integer, rhs : Integer) -> Vec<Block> {
			let mut cs = Vec::with_capacity( max( lhs.capacity(), rhs.capacity())); // TODO: Improve this capacity?? Do we need to zero out memory? XXX
			let mut i = 0;
			let mut c = false;
			let self_s = lhs.size();
			let rhs_s = rhs.size();
			let s = min( self_s, rhs_s);
			while i < s {
				let (mut x, d) = lhs.content[i].overflowing_add( rhs.content[i]);

				if c {
					let (y, e) = x.overflowing_add( 1);
					c = d || e;
					x = y
				}
				else {
					c = d
				}

				cs.push( x);
				i += 1;
			}

			let (g,s) = if self_s > rhs_s { (lhs, self_s)} else { (rhs, rhs_s)};
			while i < s {
				if c {
					let (y, e) = g.content[i].overflowing_add( 1);
					cs.push(y);
					c = e;
				}
				else {
					// Optimisation: we can break here and copy the rest of the bits? XXX
					cs.push( g.content[i]);
					c = false;
				}

				i += 1;
			}

			if c {
				cs.push( 1);
			}

			cs
		}

		match ( self.is_positive(), rhs.is_positive()) {
			(true, true) => {
				let cs = add_positives( self, rhs);
				pos_integer( cs)
			},
			(false, false) => {
				let cs = add_positives( self, rhs);
				neg_integer(cs)
			},
			(true, false) =>
				sub_positives( &self, &rhs.neg()),
			(false, true) =>
				sub_positives( &rhs, &self.neg()),
		}
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

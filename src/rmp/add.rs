use std::cmp::{max, min};
use std::ops::{Add};

// use super::internal;
use super::{Integer, Block};
// use super::sign;

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
					let (y, e) = lhs.content[i].overflowing_add( 1);
					c = d || e;
					x = y
				}
				else {
					c = d
				}

				cs.push( x);
				i += 1
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

				i += 1
			}

			if c {
				cs.push( 1)
			}

			cs
		}

		match ( self.is_positive(), rhs.is_positive()) {
			(true, true) => {
				let cs = add_positives( self, rhs);
				Integer{ positive : true, content : cs}
			},
			(false, false) => {
				let cs = add_positives( self, rhs);
				Integer{ positive : false, content : cs}
			},
			_ =>
				panic!("TODO"),
		}
	}
}


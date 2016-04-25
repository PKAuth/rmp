use std::cmp::{Ord, Ordering};

use super::{Integer};

impl Ord for Integer {
	fn cmp(&self, rhs: &Integer) -> Ordering {
		fn helper( a : &Integer, b : &Integer) -> Ordering {
			let a_s = a.size();
			let b_s = b.size();
			if a_s > b_s {
				return Ordering::Greater
			}
			else if a_s < b_s {
				return Ordering::Less
			}

			for i in (0..a_s).rev() {
				let a_i = a.content[i];
				let b_i = b.content[i];
				if a_i > b_i {
					return Ordering::Greater
				}
				else if a_i < b_i {
					return Ordering::Less
				}
			}

			Ordering::Equal
		}

		match ( self.positive, rhs.positive) {
			(true, false) =>
				Ordering::Greater,
			(false, true) =>
				Ordering::Less,
			(true, true) =>
				helper( self, rhs),
			(false, false) =>
				helper( self, rhs).reverse(),
		}
	}
}

impl PartialEq for Integer {
	fn eq(&self, rhs : &Integer) -> bool {
		self.cmp( rhs) == Ordering::Equal
	}
}

impl Eq for Integer {
}

impl PartialOrd for Integer {
	fn partial_cmp(&self, rhs: &Integer) -> Option<Ordering> {
		Some( self.cmp( rhs))
	}
}


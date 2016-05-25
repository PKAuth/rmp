use std::fmt;

use super::Integer;

impl fmt::Display for Integer {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// Check for 0.
		if self.is_zero() {
			return write!(f, "0")
		}

		let mut s : String = "".into();

		let mut i : Integer = self.clone(); 
		let mut rem : Integer;
		
		// Check if negative.
		if self.is_negative() {
			i.neg_mut()
		}

		while i > Integer::from(0) {
			let (it, remt) = i.div_mod( &Integer::from( 10));
			i = it;
			rem = remt;
			let digit = if rem.is_zero() {"0".into()} else {rem.content[0].to_string()}; //{rem.content[0].to_string()};
			s = format!( "{}{}", digit, s);
		}

		if self.is_negative() {
			s = format!( "-{}", s);
		}

		write!(f, "{}", s)
	}
}



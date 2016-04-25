use std::fmt;

use super::Integer;

impl fmt::Display for Integer {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
 		// TODO: Actually implement this XXX
		if self.is_zero() {
			write!(f, "0")
		}
		else {
			write!(f, "{}", self.content[0].to_string())
		}
	}
}



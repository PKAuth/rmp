use std::convert::From;
use std::str::FromStr;

use super::{Integer, Block};

impl From<Block> for Integer {
	fn from(x : Block) -> Integer {
		if x == 0 {
			Integer{ positive : true, content : Vec::with_capacity(1)}
		}
		else {
			Integer{ positive : true, content : vec!{x}}
		}
	}
}

impl From<i32> for Integer {
	fn from(x : i32) -> Integer {
		if x == 0 {
			Integer{ positive : true, content : Vec::with_capacity(1)}
		}
		else if x > 0 {
			Integer{ positive : true, content : vec!{x as u32}}
		}
		else if x == i32::min_value(){
			let u = i32::max_value() as u32 + 1;
			Integer{ positive : false, content : vec!{u}}
		}
		else {
			Integer{ positive : false, content : vec!{-x as u32}}
		}
	}
}

// Note: Negative 0 is ok.
impl FromStr for Integer {
	type Err = &'static str;
	
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		fn to_block( c : char) -> Option<Block> {
			if let Some( x) = c.to_digit( 10) {
				if x < 0 || x > 9 {
					None
				}
				else {
					Some( x)
				}
			}
			else {
				None
			}
		}

		if s.len() == 0 {
			return Err("Invalid Integer")
		}

		let mut chars = s.chars();
		let mut res : Integer;

		match chars.next() {
			Some('-') => {
				if let Some( x) = chars.next().and_then( to_block) {
					if x == 0 {
						return Err("Invalid Integer")
					}
					res = Integer::from( x); // TODO: Check if x is 0...
					res.neg_mut();
				}
				else {
					return Err("Invalid Integer")
				}
			},
			Some( i) => {
				if let Some( x) = to_block( i) {
					res = Integer::from( x);
				}
				else { 
					return Err("Invalid Integer")
				}
			},
			None => {
				return Err("Invalid Integer")
			},
		}

		let i10 = Integer::from( 10);
		while let Some( x) = chars.next() {
			if let Some( i) = to_block( x) {
				res = res.mul_borrow( &i10);
				res = res + Integer::from( i);
			}
			else {
				return Err("Invalid Integer")
			}
		}

		Ok( res)
	}
}

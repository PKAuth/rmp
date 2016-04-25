use std::convert::From;

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

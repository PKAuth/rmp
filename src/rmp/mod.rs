use std::vec::{Vec};

// Internal modules.
mod add;
mod display;
mod div;
mod from;
mod internal;
mod mul;
// mod one;
mod ord;
mod prime;
mod random;
mod shift;
mod sign;
mod sub;

// Re-exports.
pub use self::add::*;
pub use self::display::*;
pub use self::div::*;
pub use self::from::*;
pub use self::mul::*;
// pub use self::one::*;
pub use self::ord::*;
pub use self::prime::*;
pub use self::shift::*;
pub use self::sign::*;

// Data type for multi precision integers.
#[derive(Debug, Clone)]
pub struct Integer {
	content : Vec<Block>, // Blocks of the number. If number is 0, number of blocks is 0.
	positive : bool,      // Whether the number is positive. If number is 0, positive is true. 
}

pub type Block = u32; // Block
pub type LongBlock = u64; // Long Block
const BLOCK_SIZE : Block = 32;

// Some basic Integer functions.
impl Integer {

	/// Determine if the integer is zero.
	#[inline(always)]
	pub fn is_zero(&self) -> bool {
		self.size() == 0
	}

	/// Determine if the integer is even.
	#[inline(always)]
	pub fn is_even(&self) -> bool {
		self.is_zero() || self.content[0] & 1 != 1
	}

	/// Determine if the integer is odd.
	#[inline(always)]
	pub fn is_odd(&self) -> bool {
		!self.is_even()
	}
}


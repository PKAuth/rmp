// Module with internal helper functions.

pub type Block = u32; // Block
pub type LongBlock = u64; // Long Block
pub const BLOCK_SIZE : Block = 32;

pub fn remove_leading_zeroes( v : &mut Vec<Block>) {
    while v.len() > 0 && v[v.len() - 1] == 0 {
        v.pop();
    }
}

pub fn dbl_to_bl( x : LongBlock) -> Block {
	x as Block // Looks like this just truncates. Don't need to mask?
}

pub fn div_by_zero() {
	panic!("division by zero")
}



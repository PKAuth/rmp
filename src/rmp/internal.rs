// Module with internal helper functions.

pub fn remove_leading_zeroes( v : &mut Vec<u64>) {
    while v.len() > 0 && v[v.len() - 1] == 0 {
        v.pop();
    }
}

pub fn div_by_zero() {
	panic!("division by zero")
}


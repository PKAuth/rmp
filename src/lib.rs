// #![feature(const_fn)]

// Modules.
pub mod rmp;

// Testing.

#[cfg(test)]
mod tests {
	use rmp::types::Integer;

	#[test]
	fn equal() {
		let mp = Integer::from(63);
		let mp2 = Integer::from(63);

		assert!( mp == mp2)
	}

	#[test]
	fn not_equal() {
		let mp = Integer::from(63);
		let nmp = Integer::from(-63);
		let mp2 = Integer::from(64);

		assert!( mp != mp2);
		assert!( mp != nmp);
	}

	#[test]
	fn plus1() {
		let mp = Integer::from(63);
		let one = Integer::from(1);
		let mp2 = Integer::from(64);

		assert!( mp + one == mp2);

		let mv = Integer::from( u32::max_value());
		let one = Integer::from(1);
		let zero = Integer::from(0);

		assert!( zero != mv + one)
	}

	#[test]
	fn even() {
		assert!( Integer::from(0).is_even());
		assert!( Integer::from(2).is_even());
		assert!( Integer::from(-2).is_even());
	}

	#[test]
	fn odd() {
		assert!( Integer::from(-1).is_odd());
		assert!( Integer::from(1).is_odd());
	}

	#[test]
	fn div_mod() {
		let i33 = Integer::from( 33);
		let i32 = Integer::from( 32);
		let i16 = Integer::from( 16);
		let i2 = Integer::from( 2);
		let i0 = Integer::from( 0);
		let i1 = Integer::from( 1);
		let (q,r) = i32.div_mod(&i2);
		assert!( q == i16);
		assert!( r == i0);
		let (q,r) = i33.div_mod(&i2);
		assert!( q == i16);
		assert!( r == i1);



		println!("********** Hello world ************");
		println!("{}", q);
		println!("{}", r);
		println!("{}", i16);
		println!("{}", i0);
	}
}

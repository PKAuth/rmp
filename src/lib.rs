// Modules.
pub mod rmp;

// Testing.

#[cfg(test)]
mod tests {
	use rmp::types::Integer;

	#[test]
	fn equal() {
		let mp = Integer::from(63_u64);
		let mp2 = Integer::from(63_u64);

		assert!( mp == mp2)
	}

	#[test]
	fn not_equal() {
		let mp = Integer::from(63_u64);
		let mp2 = Integer::from(64_u64);

		assert!( mp != mp2)
	}

	#[test]
	fn plus1() {
		let mp = Integer::from(63_u64);
		let one = Integer::from(1_u64);
		let mp2 = Integer::from(64_u64);

		assert!( mp + one == mp2);

		let mv = Integer::from( u64::max_value());
		let one = Integer::from(1_u64);
		let zero = Integer::from(0_u64);

		assert!( zero != mv + one)
	}
}

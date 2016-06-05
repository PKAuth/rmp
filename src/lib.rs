// #![feature(const_fn)]

// Modules.
pub mod rmp;

// Testing.

#[cfg(test)]
mod tests {
	extern crate rand;
	
	use rmp::Integer;
	use self::rand::OsRng;

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
	fn sub() {
		let i0 = Integer::from(0);
		let i00 = Integer::from(0);
		let i000 = Integer::from(0);
		assert!( i0 - i00 == i000);

		let i1 = Integer::from(1);
		let i01 = Integer::from(1);
		let i0 = Integer::from(0);
		assert!( i1 - i01 == i0);

		let i0 = Integer::from(0);
		let i1 = Integer::from(1);
		let n1 = Integer::from(-1);
		assert!( i0.clone() - i1.clone() == n1.clone());

		assert!( i0.clone() - i0.clone() == i0.clone());
		assert!( i1.clone() - i1.clone() == i0.clone());
		assert!( n1.clone() - n1.clone() == i0.clone());

		let i7 = Integer::from( 7);
		let i84 = Integer::from( 84);
		let n77 = Integer::from( -77);
		assert!( i7 - i84 == n77);

		let i7 = Integer::from( 7);
		let i84 = Integer::from( 84);
		let n77 = Integer::from( -77);
		assert!( n77 + i84 == i7);
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



		// println!("********** Hello world ************");
		// println!("{}", q);
		// println!("{}", r);
		// println!("{}", i16);
		// println!("{}", i0);
		// println!("{}", Integer::from( -1));
		// println!("{}", Integer::from( -10120));
		// println!("{}", Integer::from( -18128));
		// println!("{}", Integer::from( -1123123));
		// assert!(false);
	}

	#[test]
	fn shr() {
		let i6 = Integer::from( 6);
		let i2 = Integer::from( 2);
		let i3 = Integer::from( 3);
		let i1 = Integer::from( 1);
		// println!("{}", i2.shr_borrow( &i1));
		assert!( i2.shr_borrow( &i1) == i1);
		assert!( i6.shr_borrow( &i1) == i3);
	}

	#[test]
	fn shl() {
		let i6 = Integer::from( 6);
		let i2 = Integer::from( 2);
		let i3 = Integer::from( 3);
		let i1 = Integer::from( 1);

		assert!( i1.shl_borrow( &i1) == i2);
		assert!( i3.shl_borrow( &i1) == i6);
	}

	#[test]
	fn mul() {
		let i3 = Integer::from( 3);
		let i1 = Integer::from( 1);
		let i0 = Integer::from( 0);
		let n3 = Integer::from( 3);
		let n1 = Integer::from( 1);

		println!( "{}", i1.clone() * i3.clone());
		println!( "{}", i3.clone());
		assert!( i1.clone() * i3.clone() == i3.clone());
		assert!( i0.clone() * i1.clone() == i0.clone());
		assert!( i1.clone() * n1.clone() == n1.clone());
		assert!( n1.clone() * n3.clone() == i3.clone());
		assert!( n1.clone() * i3.clone() == n3.clone());
	}

	#[test]
	fn div_alg_d() {
		let i0 = Integer::from( 0);
		let i1 = Integer::from( 1);
		let i2 = Integer::from( 2);
		let im = Integer::from( u32::max_value());
		let i32 = Integer::from( 32);
		let i11 = (i1.clone() << i32.clone()) + i1.clone();
		let imm = (im.clone() << i32.clone()) + im.clone();
		let immm = (imm.clone() << i32.clone()) + im.clone();
		let immmm = (immm.clone() << i32.clone()) + im.clone();

		let i3 = Integer::from( 3);
		let i18446744069414584320 = im.clone() << i32.clone();
		let i79228162495817593524129366015 = ((im.clone() << i32.clone()) << i32.clone()) + im.clone();

		// test div_mod_u_n_1
		let (q, r) = immm.div_mod( &i1);
		assert!( q == immm);
		assert!( r == i0);

		let (q, r) = imm.div_mod( &i11);
		// // println!( "q:{}", q);
		// // println!( "r:{}", r);
		assert!( q == im);
		assert!( r == i0);

		let (q, r) = immm.div_mod( &i11);
		// println!( "q:{}", q);
		// println!( "r:{}", r);
		assert!( q == i18446744069414584320); // 18446744069414584320
		assert!( r == im); // 4294967295

		// println!("{}/{}", immmm, i11);

		let (q, r) = immmm.div_mod( &i11);
		// println!( "q:{}", q);
		// println!( "r:{}", r);
		// println!( "i:{}", i79228162495817593524129366015);
		assert!( q == i79228162495817593524129366015); // 79228162495817593524129366015
		assert!( r == i0); // 0

		let (q, r) = i79228162495817593524129366015.div_mod( &i18446744069414584320);
		assert!( q == im.clone() + i1.clone());
		assert!( r == im);

		panic!("TODO: Test add back XXX");
	}

	#[test]
	fn prime() {
		let mut r = OsRng::new().unwrap();
		
		println!("{}", Integer::generate_prime(2, &mut r));

		assert!(false);
	}
}

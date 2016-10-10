// #![feature(const_fn)]

// Modules.
pub mod rmp;

// Testing.

#[cfg(test)]
mod tests {
	extern crate rand;

	use std::str::FromStr;
	
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
		let i0 = Integer::from( 0);
		// println!("{}", i2.shr_borrow( &i1));
		assert!( i2.shr_borrow( &i1) == i1);
		assert!( i6.shr_borrow( &i1) == i3);
		assert!( i6.shr_borrow( &i0) == i6);
	}

	#[test]
	fn shl() {
		let i6 = Integer::from( 6);
		let i2 = Integer::from( 2);
		let i3 = Integer::from( 3);
		let i1 = Integer::from( 1);
		let i0 = Integer::from( 0);

		assert!( i1.shl_borrow( &i1) == i2);
		assert!( i3.shl_borrow( &i1) == i6);
		assert!( i6.shl_borrow( &i0) == i6);
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
		// let i2 = Integer::from( 2);
		let im = Integer::from( u32::max_value());
		let i32 = Integer::from( 32);
		let i11 = (i1.clone() << i32.clone()) + i1.clone();
		let imm = (im.clone() << i32.clone()) + im.clone();
		let immm = (imm.clone() << i32.clone()) + im.clone();
		let immmm = (immm.clone() << i32.clone()) + im.clone();

		// let i3 = Integer::from( 3);
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

		let i9223231299366420480 = Integer::from_str("9223231299366420480").unwrap();
		let i140737488355329 = Integer::from_str("140737488355329").unwrap();

		let i65534 = Integer::from_str("65534").unwrap();
		let i140737488289794 = Integer::from_str("140737488289794").unwrap();

		assert!( i9223231299366420480.div_mod( &i140737488355329) == (i65534, i140737488289794));

		let t = Integer::from_str( "37755296672150276119904024272893412830191382436986206695533510300981264900").unwrap();
		let b = Integer::from_str( "9524048358032163729718187613397213873").unwrap();
		let q = Integer::from_str( "3964206737811145041420635799606649236").unwrap();
		let r = Integer::from_str( "9524048358032163729718187613397213872").unwrap();

		assert!( t.div_mod( &b) == ( q, r));

		let n = Integer::from_str("27651709839375565209125650188875420306105501298794198504577623286716045783225").unwrap();
		let d = Integer::from_str("308902425432351122477400238366534792609").unwrap();
		let q = Integer::from_str("89516001050083118238499019832146173792").unwrap();
		let r = Integer::from_str("35790995422436827861984988492454679897").unwrap();
		// let ( aq, ar) = n.div_mod( &d);
		// println!("{}/{} = ({},{})", n, d, aq, ar);
		assert!( n.div_mod( &d) == ( q, r));
	}

	#[test]
	fn exp_mod() {
		let i3 = Integer::from( 3);
		let i2 = Integer::from( 2);
		let i9 = Integer::from( 9);
		let i10 = Integer::from( 10);

		assert!( i9 == i3.exp_mod( &i2, &i10));

		let i1 = Integer::from( 1);
		let i12620138152996470709 = Integer::from_str("12620138152996470709").unwrap();
		let i1947183550686142566 = Integer::from_str("1947183550686142566").unwrap();
		// println!( "res: {}", i1947183550686142566.exp_mod( &i12620138152996470709.sub_borrow( &i1), &i12620138152996470709));

		assert!( i1 == i1947183550686142566.exp_mod( &i12620138152996470709.sub_borrow( &i1), &i12620138152996470709));

		let i308902425432351122477400238366534792609 = Integer::from_str("308902425432351122477400238366534792609").unwrap();
		let i308548023917423949115556599091438751199 = Integer::from_str("308548023917423949115556599091438751199").unwrap();

		assert!( i1 == i308548023917423949115556599091438751199.exp_mod( &i308902425432351122477400238366534792609.sub_borrow( &i1), &i308902425432351122477400238366534792609));
	}

	#[test]
	fn prime() {
		let mut r = OsRng::new().unwrap();

		let i12620138152996470709 = Integer::from_str("12620138152996470709").unwrap();
		assert!( i12620138152996470709.is_probably_prime( &mut r));

		let i244917586431534476306755499156239542077 = Integer::from_str("244917586431534476306755499156239542077").unwrap();
		assert!( !i244917586431534476306755499156239542077.is_probably_prime( &mut r));

		let i308902425432351122477400238366534792609 = Integer::from_str("308902425432351122477400238366534792609").unwrap();
		assert!( i308902425432351122477400238366534792609.is_probably_prime( &mut r));
		
		let i128263571026758587339481122278816336209 = Integer::from_str("128263571026758587339481122278816336209").unwrap();
		assert!( !i128263571026758587339481122278816336209.is_probably_prime( &mut r));
		
		// println!("{}", Integer::generate_prime(4, &mut r));

		// println!("{}", Integer::generate_prime( 64, &mut r));

		// assert!(false);
	}

	#[test]
	fn extended_gcd() {
		let i693 = Integer::from(693);
		let i609 = Integer::from(609);
		let i21 = Integer::from( 21);
		// let i206 = Integer::from( 206);
		// let n181 = Integer::from( -181);
		let (a, b, g) = Integer::extended_gcd( &i693, &i609);
		// println!("{} {} {}", a, b, g);
		assert!( g == i21);
		assert!( a * i693 + b * i609 == g);
	}

	#[test]
	fn multiplicative_inverse() {
		let i383 = Integer::from(383);
		let i271 = Integer::from(271);
		let i106 = Integer::from(106);

		let res = i271.multiplicative_inverse( &i383);
		assert!( res == Some( i106));

		let i72639 = Integer::from(72639);
		let i4294967296 = Integer::from_str("4294967296").unwrap();
		let i457233471 = Integer::from(457233471);

		let res = i72639.multiplicative_inverse( &i4294967296);
		assert!( res == Some(i457233471));
	}

	#[test]
	fn montgomery_multiplication() {
		let m = Integer::from(72639);
		let x = Integer::from(5792);
		let y = Integer::from(1229);
		let exp = Integer::from(72385);
		let i1 = Integer::from(1);

		let (mul_r, mul) = Integer::montgomery_multiplication( m.clone()).unwrap();
		let res = mul( &mul( &mul_r(&x), &mul_r(&y)), &i1);
		// println!("{} = {} * {} mod {}", res, x, y, m);
		assert!( res == exp);
	}

	#[test]
	fn karatsuba_mul() {
		
		let a = Integer::from_str( "14629656518445490640279744086021056458081238838802316415905309290044988258245946343643075306222985264569596925160919836879758883521476808720491930677705694405026652").unwrap();
		let b = Integer::from_str( "14629656518445490640279744086021056458081238838802316415905309290044988258245946343643075306222985264569596925160919836879758883521476808720491930677705694405026652").unwrap();

		let ab = Integer::from_str( "214026849847694634422751564733436658938342106846835364094717925718877392450304199703169591250340140167626370348898016957964034876488643833296498629013006809640427732347343599653250254649167098393278723023083024967751505803452454171454638810477853320361326443284907764708331174192444632700815206794348630836092728359564830329104").unwrap();

		// println!("ab: {}", ab);
		// println!("a*b: {}", a.mul_borrow( &b));
		assert!( ab == a.mul_borrow( &b));



		let x = Integer::from_str( "21278747534453977990903307617550092127816054754437603812088370991524168359350927293689518375657467705808824855451713535377104579109423256952771084324877222480324295021672698128434084894234461574897461686686130067063960294314095771904117653987569329112511618880766061578848972273722631577590810098262339603133560285824756358444911436546743615315668489394057698353537891953514169785330381563666012926098976689294503529059251495144939078334135685580225589558687058707811147050227116199999275788518979326612448041179825706963905594793988357908654664891711683610238646063075440040313148722731356937211479120252029972513232").unwrap();
		let xx = Integer::from_str("452785096635031247265054158814999479626427499572121810108506462188230796356366996048139195067396069437887459704929482668313085929969512769894178451293627269780693206723846161197404463659452891635373900095631909706620247376106766084810225724312784851646405267568130241386604354815825917367208239233976857668084999388476009871491821391075566836211421913602656264422662572025856627832511172564610076358329110984378526590264392088135609898207664690990144977996124947231785357358862289206351357981299708242835371329477435354428635971091081434190666812316153885112141113831682877494239243826723722313555202456781206662672070562345534795513659140410851021442370368396098913049430380966950176689698988999436661704851791015369125378565980940112444317051517533405944091086954007226858934535462316626508373000753584130261764034985823401738386350448873364411200185965701131638824755177303894392945110954088182761212788105203033843436194459039624366836894242270847315363689593290741485602544571513028873222341786268271905629383782787613702306510194631142501384194204473080375054552160670234631461272593530080592847986429925112026161795069019995716873232647546123988888104661263697995658796906996665113669493540523648645732090476700477442415085824").unwrap();

		let y = Integer::from_str( "21354374014910357923585486910296494611791043684400646935224275686811552546623796192115583465997576328536111685772021183877934327973919741785156352132049879422318628518777380094517994904348640832138039488045091159899513745612369626584332612932259084851044364871316372463907446431677734479471160163814966782142946113714335013451159894681903299490124094147277301710032690895557831865166359431181921333521484853693656835756844571276125760056520817325735992405673528772024794131158735989741209749443521179217588702382737435926282159323709197769988055320817951914839310714304487249844588346715681059081270515275822421081417").unwrap();
		let xy = Integer::from_str("454394333419581873709838843066672997794813652274780751765869708741446483961447665830081241761388721800925617588474247414874707119767553486667681409321663944676388330107958926520968929319029669962361758747718986184506310084085659861757665344233034096396272087488844721012044697911384678262832439726596281862555266745459848287526583564670296062997466798546708380456355719897496739344383020345885825118203442968200717251936699182760031647338068554210208289030710510033969156965194394540099497698345470574817532306505625392790040834440293935691029100061510458666474990919894688088455738625708303747925126792719393480588211075995984511800883428033440555944609343503834460421234402890478634070363867787000951979735176726949871720826675357203235037591228239960382985336233047890903319110372738274848914754292688137827088149646080839162344176238074590965165845491877837179641733439218435311456517383499359441268034602643709661011549171131114563087695345932412522400340508110835782389196315553378471672558499999190548104139856482735967956782018694737684190918367308072149905930831365530754166186199967030422755074542800350348169263395724636832071304700379007737876699685621362198210607126345574679501040232346222630086241671202629046781809744").unwrap();


		println!("xx: {}", xx);
		println!("x*x: {}", x.mul_borrow( &x));

		assert!( xx == x.mul_borrow( &x));
		assert!( xy == x.mul_borrow( &y));
	}


	// extern crate criterion;
	// use criterion::{Bencher, Criterion};
	// #[bench]
	// fn prime_bench() {
	// 	fn routine(b: &mut Bencher) {
  //   	// Setup (construct data, allocate memory, etc)
	// 		let mut r = OsRng::new().unwrap();
	// 		let i12620138152996470709 = Integer::from_str("12620138152996470709").unwrap();

  //   	b.iter(|| {
  //   	    i12620138152996470709.is_probably_prime( &mut r)
  //   	})

  //   	// Teardown (free resources)
	// 	}
	// 	Criterion::default().bench("routine", routine)
	// }
}

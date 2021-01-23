use parity_scale_codec::{Encode, Decode};

#[derive(Debug, PartialEq, Encode, Decode)]
enum EnumType {
	#[codec(index = 15)]
	A,
	B(u32, u64),
	C {
		a: u32,
		b: u64,
	},
}

fn main() {
	let a = EnumType::A;
	let b = EnumType::B(1, 2);
	let c = EnumType::C { a: 1, b: 2 };
	a.using_encoded(|ref slice| {
		assert_eq!(slice, &b"\x0f");
	});
}
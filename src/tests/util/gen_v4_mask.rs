use super::gen_v4_mask;
use std::net::Ipv4Addr;

#[test]
fn given_between_0_and_32_returns_correct() {
	for i in 0..=32 {
		let actual: Ipv4Addr = gen_v4_mask(i);
		let expected: Ipv4Addr = Ipv4Addr::from(std::u32::MAX.checked_shl(32 - i).unwrap_or(0));
		assert!(actual == expected, "{} == {} (/{})", actual, expected, i);
	}
}

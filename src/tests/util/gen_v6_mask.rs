use super::gen_v6_mask;
use std::net::Ipv6Addr;

#[test]
fn given_between_0_and_128_returns_correct() {
	for i in 0..=128 {
		let actual: Ipv6Addr = gen_v6_mask(i);
		let expected: Ipv6Addr = Ipv6Addr::from(std::u128::MAX.checked_shl(128 - i).unwrap_or(0));
		assert!(actual == expected, "{} == {} (/{})", actual, expected, i);
	}
}

use super::Mask;
use std::net::Ipv6Addr;

impl Mask for Ipv6Addr {
	type Output = Self;

	fn mask(&self, other: &Self) -> Self::Output {
		Self::Output::from((u128::from(*self)) & (u128::from(*other)))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn returns_correct_answer() {
		let a: Ipv6Addr = "2001:db8::dead:beef".parse().unwrap();
		let b: Ipv6Addr = "ffff:ffff::".parse().unwrap();

		assert_eq!(a.mask(&b), "2001:db8::".parse::<Ipv6Addr>().unwrap());
	}

	#[test]
	fn is_reflexive() {
		let a: Ipv6Addr = "2001:db8::dead:beef".parse().unwrap();
		let b: Ipv6Addr = "ffff:ffff::".parse().unwrap();

		assert_eq!(a.mask(&b), b.mask(&a));
	}
}

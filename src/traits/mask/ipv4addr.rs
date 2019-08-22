use super::Mask;
use std::net::Ipv4Addr;

impl Mask for Ipv4Addr {
	type Output = Self;

	fn mask(&self, other: &Self) -> Self::Output {
		Self::Output::from((u32::from(*self)) & (u32::from(*other)))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn returns_correct_answer() {
		let a: Ipv4Addr = "192.0.2.1".parse().unwrap();
		let b: Ipv4Addr = "255.255.255.0".parse().unwrap();

		assert_eq!(a.mask(&b), "192.0.2.0".parse::<Ipv4Addr>().unwrap());
	}

	#[test]
	fn is_reflexive() {
		let a: Ipv4Addr = "192.0.2.1".parse().unwrap();
		let b: Ipv4Addr = "255.255.255.0".parse().unwrap();

		assert_eq!(a.mask(&b), b.mask(&a));
	}
}

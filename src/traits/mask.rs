use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub trait Mask {
	type Output;

	fn mask(&self, other: &Self) -> Self::Output;
}

impl Mask for IpAddr {
	type Output = Result<Self, &'static str>;

	fn mask(&self, other: &Self) -> Self::Output {
		match (self, other) {
			// TODO convert to Self::V4, Self::V6 once stabilized in 1.37 (2019-08-15)
			(IpAddr::V4(a), IpAddr::V4(b)) => Ok(IpAddr::V4(a.mask(&b))),
			// TODO convert to Self::V4, Self::V6 once stabilized in 1.37 (2019-08-15)
			(IpAddr::V6(a), IpAddr::V6(b)) => Ok(IpAddr::V6(a.mask(&b))),
			(_, _) => Err("mismatched address types"),
		}
	}
}

impl Mask for Ipv4Addr {
	type Output = Self;

	fn mask(&self, other: &Self) -> Self::Output {
		Self::Output::from((u32::from(*self)) & (u32::from(*other)))
	}
}

impl Mask for Ipv6Addr {
	type Output = Self;

	fn mask(&self, other: &Self) -> Self::Output {
		Self::Output::from((u128::from(*self)) & (u128::from(*other)))
	}
}

#[cfg(test)]
mod tests {
	use super::Mask;

	mod ipaddr {
		use super::*;
		use std::net::IpAddr;

		mod v4_and_v4 {
			use super::*;

			#[test]
			fn bidirectionally_returns_ok() {
				let a: IpAddr = "192.0.2.1".parse().unwrap();
				let b: IpAddr = "255.255.255.0".parse().unwrap();

				assert!(a.mask(&b).is_ok());
				assert!(b.mask(&a).is_ok());
			}

			#[test]
			fn returns_correct_answer() {
				let a: IpAddr = "192.0.2.1".parse().unwrap();
				let b: IpAddr = "255.255.255.0".parse().unwrap();

				assert_eq!(a.mask(&b), Ok("192.0.2.0".parse::<IpAddr>().unwrap()));
			}

			#[test]
			fn is_reflexive() {
				let a: IpAddr = "192.0.2.1".parse().unwrap();
				let b: IpAddr = "255.255.255.0".parse().unwrap();

				assert_eq!(a.mask(&b), b.mask(&a));
			}
		}

		mod v6_and_v6 {
			use super::*;

			#[test]
			fn bidirectionally_returns_ok() {
				let a: IpAddr = "2001:db8::dead:beef".parse().unwrap();
				let b: IpAddr = "ffff:ffff::".parse().unwrap();

				assert!(a.mask(&b).is_ok());
				assert!(b.mask(&a).is_ok());
			}

			#[test]
			fn returns_correct_answer() {
				let a: IpAddr = "2001:db8::dead:beef".parse().unwrap();
				let b: IpAddr = "ffff:ffff::".parse().unwrap();

				assert_eq!(a.mask(&b), Ok("2001:db8::".parse::<IpAddr>().unwrap()));
			}

			#[test]
			fn is_reflexive() {
				let a: IpAddr = "2001:db8::dead:beef".parse().unwrap();
				let b: IpAddr = "ffff:ffff::".parse().unwrap();

				assert_eq!(a.mask(&b), b.mask(&a));
			}
		}

		mod v4_and_v6 {
			use super::*;

			#[test]
			fn bidirectionally_returns_err() {
				let a: IpAddr = "192.0.2.0".parse().unwrap();
				let b: IpAddr = "ffff:ffff::".parse().unwrap();

				assert!(a.mask(&b).is_err());
				assert!(b.mask(&a).is_err());
			}

			#[test]
			fn returns_correct_error() {
				let a: IpAddr = "192.0.2.0".parse().unwrap();
				let b: IpAddr = "ffff:ffff::".parse().unwrap();

				assert_eq!(a.mask(&b), Err("mismatched address types"));
			}

			#[test]
			fn is_reflexive() {
				let a: IpAddr = "192.0.2.0".parse().unwrap();
				let b: IpAddr = "ffff:ffff::".parse().unwrap();

				assert_eq!(a.mask(&b), b.mask(&a));
			}
		}
	}

	mod ipv4addr {
		use super::*;
		use std::net::Ipv4Addr;

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

	mod ipv6addr {
		use super::*;
		use std::net::Ipv6Addr;

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
}

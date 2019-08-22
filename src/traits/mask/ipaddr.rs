use super::Mask;
use std::net::IpAddr;

impl Mask for IpAddr {
	type Output = Result<Self, &'static str>;

	fn mask(&self, other: &Self) -> Self::Output {
		match (self, other) {
			(Self::V4(a), Self::V4(b)) => Ok(Self::V4(a.mask(&b))),
			(Self::V6(a), Self::V6(b)) => Ok(Self::V6(a.mask(&b))),
			(_, _) => Err("mismatched address types"),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	mod v4 {
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

		#[test]
		fn v6_bidirectionally_returns_err() {
			let a: IpAddr = "192.0.2.0".parse().unwrap();
			let b: IpAddr = "ffff:ffff::".parse().unwrap();

			assert!(a.mask(&b).is_err());
			assert!(b.mask(&a).is_err());
		}

		#[test]
		fn v6_returns_correct_error() {
			let a: IpAddr = "192.0.2.0".parse().unwrap();
			let b: IpAddr = "ffff:ffff::".parse().unwrap();

			assert_eq!(a.mask(&b), Err("mismatched address types"));
		}

		#[test]
		fn v6_is_reflexive() {
			let a: IpAddr = "192.0.2.0".parse().unwrap();
			let b: IpAddr = "ffff:ffff::".parse().unwrap();

			assert_eq!(a.mask(&b), b.mask(&a));
		}
	}

	mod v6 {
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

		#[test]
		fn v4_bidirectionally_returns_ok() {
			let a: IpAddr = "2001:db8::dead:beef".parse().unwrap();
			let b: IpAddr = "255.255.255.0".parse().unwrap();

			assert!(a.mask(&b).is_err());
			assert!(b.mask(&a).is_err());
		}

		#[test]
		fn v4_returns_correct_answer() {
			let a: IpAddr = "2001:db8::dead:beef".parse().unwrap();
			let b: IpAddr = "255.255.255.0".parse().unwrap();

			assert_eq!(a.mask(&b), Err("mismatched address types"));
		}

		#[test]
		fn v4_is_reflexive() {
			let a: IpAddr = "2001:db8::dead:beef".parse().unwrap();
			let b: IpAddr = "255.255.255.0".parse().unwrap();

			assert_eq!(a.mask(&b), b.mask(&a));
		}
	}
}

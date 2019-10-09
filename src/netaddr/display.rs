use super::NetAddr;
use core::fmt;

impl fmt::Display for NetAddr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::V4(addr) => write!(f, "{}", addr),
			Self::V6(addr) => write!(f, "{}", addr),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::NetAddr;

	mod v4 {
		use super::NetAddr;

		#[test]
		fn cidr() {
			let addr: NetAddr = "127.0.0.1/0.0.0.0".parse().unwrap();
			assert_eq!(format!("{}", addr), "0.0.0.0/0");

			let addr: NetAddr = "127.0.0.1/255.255.255.0".parse().unwrap();
			assert_eq!(format!("{}", addr), "127.0.0.0/24");

			let addr: NetAddr = "127.0.0.1/255.255.255.255.".parse().unwrap();
			assert_eq!(format!("{}", addr), "127.0.0.1/32");
		}

		#[test]
		fn non_cidr() {
			let addr: NetAddr = "127.0.0.1/251.255.255.0".parse().unwrap();
			assert_eq!(format!("{}", addr), "123.0.0.0/251.255.255.0")
		}
	}

	mod v6 {
		use super::NetAddr;

		#[test]
		fn cidr() {
			let addr: NetAddr = "2001:db8:dead:beef::/::".parse().unwrap();
			assert_eq!(format!("{}", addr), "::/0");

			let addr: NetAddr = "2001:db8:dead:beef::/ffff:ffff:ffff:fff0::"
				.parse()
				.unwrap();
			assert_eq!(format!("{}", addr), "2001:db8:dead:bee0::/60");

			let addr: NetAddr = "2001:db8:dead:beef::/ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff"
				.parse()
				.unwrap();
			assert_eq!(format!("{}", addr), "2001:db8:dead:beef::/128");
		}

		#[test]
		fn non_cidr() {
			let addr: NetAddr = "2001:db8:dead:beef::/ffff:ffff:ffff:fddf::"
				.parse()
				.unwrap();
			assert_eq!(
				format!("{}", addr),
				"2001:db8:dead:bccf::/ffff:ffff:ffff:fddf::"
			)
		}
	}
}

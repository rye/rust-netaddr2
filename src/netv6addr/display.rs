use super::Netv6Addr;
use core::fmt;

impl fmt::Display for Netv6Addr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mask: u128 = self.mask().into();
		let ones = mask.count_ones();
		let cidr_mask: u128 = u128::max_value().checked_shl(128 - ones).unwrap_or(0);

		if mask == cidr_mask {
			write!(f, "{}/{}", self.addr(), ones)
		} else {
			write!(f, "{}/{}", self.addr(), self.mask())
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Netv6Addr;

	#[test]
	fn cidr() {
		// We test in three main cases:

		// (i) The mask has zero bits... (shl must not fail)
		let addr: Netv6Addr = "2001:db8:dead:beef::/::".parse().unwrap();
		assert_eq!(format!("{}", addr), "::/0");

		// (ii) The mask has 0 < n < 128 bits...
		let addr: Netv6Addr = "2001:db8:dead:beef::/ffff:ffff:ffff:fff0::"
			.parse()
			.unwrap();
		assert_eq!(format!("{}", addr), "2001:db8:dead:bee0::/60");

		// (iii) The mask has 128 bits...
		let addr: Netv6Addr = "2001:db8:dead:beef::/ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff"
			.parse()
			.unwrap();
		assert_eq!(format!("{}", addr), "2001:db8:dead:beef::/128");
	}

	#[test]
	fn non_cidr() {
		let addr: Netv6Addr = "2001:db8:dead:beef::/ffff:ffff:ffff:fddf::"
			.parse()
			.unwrap();
		assert_eq!(
			format!("{}", addr),
			"2001:db8:dead:bccf::/ffff:ffff:ffff:fddf::"
		)
	}
}

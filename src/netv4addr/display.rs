use super::Netv4Addr;
use core::fmt;

impl fmt::Display for Netv4Addr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mask: u32 = self.mask().into();
		let ones = mask.count_ones();
		let cidr_mask: u32 = u32::max_value().checked_shl(32 - ones).unwrap_or(0);

		if mask == cidr_mask {
			write!(f, "{}/{}", self.addr(), ones)
		} else {
			write!(f, "{}/{}", self.addr(), self.mask())
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Netv4Addr;

	#[test]
	fn cidr() {
		// We test in three main cases:

		// (i) The mask has zero bits... (shl must not fail)
		let addr: Netv4Addr = "127.0.0.1/0.0.0.0".parse().unwrap();
		assert_eq!(format!("{}", addr), "0.0.0.0/0");

		// (ii) The mask has 0 < n < 32 bits...
		let addr: Netv4Addr = "127.0.0.1/255.255.255.0".parse().unwrap();
		assert_eq!(format!("{}", addr), "127.0.0.0/24");

		// (iii) The mask has 32 bits...
		let addr: Netv4Addr = "127.0.0.1/255.255.255.255.".parse().unwrap();
		assert_eq!(format!("{}", addr), "127.0.0.1/32");
	}

	#[test]
	fn non_cidr() {
		let addr: Netv4Addr = "127.0.0.1/251.255.255.0".parse().unwrap();
		assert_eq!(format!("{}", addr), "123.0.0.0/251.255.255.0")
	}
}

use super::Netv6Addr;
use core::fmt;

impl fmt::Display for Netv6Addr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mask: u128 = (*self.mask()).into();
		let ones = mask.count_ones();
		let cidr_mask: u128 = u128::max_value().checked_shl(128 - ones).unwrap_or(0);

		if mask == cidr_mask {
			write!(f, "{}/{}", self.addr(), ones)
		} else {
			write!(f, "{}/{}", self.addr(), self.mask())
		}
	}
}

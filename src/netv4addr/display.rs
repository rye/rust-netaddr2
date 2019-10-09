use super::Netv4Addr;
use core::fmt;

impl fmt::Display for Netv4Addr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mask: u32 = (*self.mask()).into();
		let ones = mask.count_ones();
		let cidr_mask: u32 = u32::max_value().checked_shl(32 - ones).unwrap_or(0);

		if mask == cidr_mask {
			write!(f, "{}/{}", self.addr(), ones)
		} else {
			write!(f, "{}/{}", self.addr(), self.mask())
		}
	}
}

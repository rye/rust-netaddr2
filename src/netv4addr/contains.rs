use crate::traits::Contains;
use crate::traits::Mask;

use crate::netv4addr::Netv4Addr;

impl Contains for Netv4Addr {
	fn contains<T: Copy>(&self, other: &T) -> bool
	where
		Self: From<T>,
	{
		let other: Self = Self::from(*other);
		other.addr().mask(&self.mask()) == *self.addr()
	}
}

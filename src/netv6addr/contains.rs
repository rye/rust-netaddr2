use crate::netv6addr::Netv6Addr;
use crate::traits::Contains;
use crate::traits::Mask;

impl Contains for Netv6Addr {
	fn contains<T: Copy>(&self, other: &T) -> bool
	where
		Self: From<T>,
	{
		let other: Self = Self::from(*other);
		other.addr().mask(&self.mask()) == *self.addr()
	}
}

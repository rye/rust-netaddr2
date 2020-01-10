use core::convert::TryInto;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub trait Offset<T>: Sized {
	fn offset(&self, offset: T) -> Option<Self>;
}

impl Offset<u128> for Ipv6Addr {
	fn offset(&self, offset: u128) -> Option<Self> {
		u128::from(*self).checked_add(offset).map(Ipv6Addr::from)
	}
}

impl Offset<i32> for Ipv6Addr {
	fn offset(&self, offset: i32) -> Option<Self> {
		use core::i32::{MAX, MIN};

		// Without using weird BigInt types, we need to use absolute
		// values and checked arithmetic.
		let offset_abs: Option<u128> = offset.abs().try_into().ok();

		// This is a simple "if positive, use checked_add, if negative, use checked_sub" conditional
		offset_abs
			.map(|abs: u128| match offset {
				0..=MAX => u128::from(*self).checked_add(abs),
				MIN..=-1 => u128::from(*self).checked_sub(abs),
			})
			.flatten()
			.map(Ipv6Addr::from)
	}
}

impl Offset<u32> for Ipv6Addr {
	fn offset(&self, offset: u32) -> Option<Self> {
		u128::from(*self)
			.checked_add(offset.into())
			.map(Ipv6Addr::from)
	}
}

impl Offset<u32> for Ipv4Addr {
	fn offset(&self, offset: u32) -> Option<Self> {
		u32::from(*self).checked_add(offset).map(Ipv4Addr::from)
	}
}

impl Offset<i32> for Ipv4Addr {
	fn offset(&self, offset: i32) -> Option<Self> {
		let addr: i64 = u32::from(*self).into();
		let offset: Option<u32> = addr
			.checked_add(i64::from(offset))
			.map(TryInto::try_into)
			.map(Result::ok)
			.flatten();
		offset.map(Ipv4Addr::from)
	}
}

impl Offset<u128> for Ipv4Addr {
	fn offset(&self, offset: u128) -> Option<Self> {
		let offset: Option<u32> = offset.try_into().ok();
		offset
			.map(|offset: u32| u32::from(*self).checked_add(offset))
			.flatten()
			.map(Ipv4Addr::from)
	}
}

impl Offset<i32> for IpAddr {
	fn offset(&self, offset: i32) -> Option<Self> {
		match self {
			IpAddr::V4(v4) => v4.offset(offset).map(IpAddr::V4),
			IpAddr::V6(v6) => v6.offset(offset).map(IpAddr::V6),
		}
	}
}

impl Offset<u32> for IpAddr {
	fn offset(&self, offset: u32) -> Option<Self> {
		match self {
			IpAddr::V4(v4) => v4.offset(offset).map(IpAddr::V4),
			IpAddr::V6(v6) => v6.offset(offset).map(IpAddr::V6),
		}
	}
}

impl Offset<u128> for IpAddr {
	fn offset(&self, offset: u128) -> Option<Self> {
		match self {
			IpAddr::V4(v4) => v4.offset(offset).map(IpAddr::V4),
			IpAddr::V6(v6) => v6.offset(offset).map(IpAddr::V6),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	macro_rules! test_offset {
		($test_fn_name:ident, $addr_type:ty, $addr:literal, $amount:literal, $expected:literal) => {
			#[test]
			fn $test_fn_name() {
				assert_eq!(
					$addr.parse::<$addr_type>().unwrap().offset($amount),
					$expected.parse::<$addr_type>().ok()
				)
			}
		};
		($test_fn_name:ident, $addr_type:ty, $addr:literal, $amount:literal, None) => {
			#[test]
			fn $test_fn_name() {
				assert_eq!($addr.parse::<$addr_type>().unwrap().offset($amount), None)
			}
		};
	}

	// Basic sanity checks here that the behavior works as expected

	test_offset!(v4_0u32, Ipv4Addr, "127.0.0.0", 0_u32, "127.0.0.0");
	test_offset!(v4_1u32, Ipv4Addr, "127.0.0.0", 1_u32, "127.0.0.1");
	test_offset!(v4_2u32, Ipv4Addr, "127.0.0.0", 2_u32, "127.0.0.2");

	test_offset!(v6_0u32, Ipv6Addr, "2001:db8::", 0_u32, "2001:db8::0");
	test_offset!(v6_1u32, Ipv6Addr, "2001:db8::", 1_u32, "2001:db8::1");
	test_offset!(v6_2u32, Ipv6Addr, "2001:db8::", 2_u32, "2001:db8::2");

	test_offset!(a_v4_0u32, IpAddr, "127.0.0.0", 0_u32, "127.0.0.0");
	test_offset!(a_v4_1u32, IpAddr, "127.0.0.0", 1_u32, "127.0.0.1");
	test_offset!(a_v4_2u32, IpAddr, "127.0.0.0", 2_u32, "127.0.0.2");

	test_offset!(a_v6_0u32, IpAddr, "2001:db8::", 0_u32, "2001:db8::0");
	test_offset!(a_v6_1u32, IpAddr, "2001:db8::", 1_u32, "2001:db8::1");
	test_offset!(a_v6_2u32, IpAddr, "2001:db8::", 2_u32, "2001:db8::2");

	test_offset!(v4_0i32, Ipv4Addr, "127.0.0.0", 0_i32, "127.0.0.0");
	test_offset!(v4_1i32, Ipv4Addr, "127.0.0.0", 1_i32, "127.0.0.1");
	test_offset!(v4_2i32, Ipv4Addr, "127.0.0.0", 2_i32, "127.0.0.2");

	test_offset!(v6_0i32, Ipv6Addr, "2001:db8::", 0_i32, "2001:db8::0");
	test_offset!(v6_1i32, Ipv6Addr, "2001:db8::", 1_i32, "2001:db8::1");
	test_offset!(v6_2i32, Ipv6Addr, "2001:db8::", 2_i32, "2001:db8::2");

	test_offset!(a_v4_0i32, IpAddr, "127.0.0.0", 0_i32, "127.0.0.0");
	test_offset!(a_v4_1i32, IpAddr, "127.0.0.0", 1_i32, "127.0.0.1");
	test_offset!(a_v4_2i32, IpAddr, "127.0.0.0", 2_i32, "127.0.0.2");

	test_offset!(a_v6_0i32, IpAddr, "2001:db8::", 0_i32, "2001:db8::0");
	test_offset!(a_v6_1i32, IpAddr, "2001:db8::", 1_i32, "2001:db8::1");
	test_offset!(a_v6_2i32, IpAddr, "2001:db8::", 2_i32, "2001:db8::2");

	// Some more odd cases

	test_offset!(
		v4_sub_1i32,
		Ipv4Addr,
		"127.0.0.0",
		-1_i32,
		"126.255.255.255"
	);

	test_offset!(v4_min_minus_1i32, Ipv4Addr, "0.0.0.0", -1_i32, None);
	test_offset!(v4_max_plus_1i32, Ipv4Addr, "255.255.255.255", 1_i32, None);

	test_offset!(
		v6_sub_1i32,
		Ipv6Addr,
		"2001:db8::",
		-1_i32,
		"2001:db7:ffff:ffff:ffff:ffff:ffff:ffff"
	);

	test_offset!(v6_min_minus_1i32, Ipv6Addr, "::", -1_i32, None);
	test_offset!(
		v6_max_plus_1i32,
		Ipv6Addr,
		"ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff",
		1_i32,
		None
	);
}

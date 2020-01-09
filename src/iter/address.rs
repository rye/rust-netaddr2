use crate::Contains;

pub struct AddressIterator<Network, Address> {
	net: Network,
	cur: Option<Address>,
}

mod maybe_next {
	use core::convert::TryInto;

	use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

	pub trait MaybeNext<T>: Sized {
		fn maybe_next(&self, offset: T) -> Option<Self>;
	}

	impl MaybeNext<u128> for Ipv6Addr {
		fn maybe_next(&self, offset: u128) -> Option<Self> {
			u128::from(*self).checked_add(offset).map(Ipv6Addr::from)
		}
	}

	impl MaybeNext<i32> for Ipv6Addr {
		fn maybe_next(&self, offset: i32) -> Option<Self> {
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

	impl MaybeNext<u32> for Ipv6Addr {
		fn maybe_next(&self, offset: u32) -> Option<Self> {
			u128::from(*self)
				.checked_add(offset.into())
				.map(Ipv6Addr::from)
		}
	}

	impl MaybeNext<u32> for Ipv4Addr {
		fn maybe_next(&self, offset: u32) -> Option<Self> {
			u32::from(*self).checked_add(offset).map(Ipv4Addr::from)
		}
	}

	impl MaybeNext<i32> for Ipv4Addr {
		fn maybe_next(&self, offset: i32) -> Option<Self> {
			let addr: i64 = u32::from(*self).into();
			let maybe_next: Option<u32> = addr
				.checked_add(i64::from(offset))
				.map(TryInto::try_into)
				.map(Result::ok)
				.flatten();
			maybe_next.map(Ipv4Addr::from)
		}
	}

	impl MaybeNext<u128> for Ipv4Addr {
		fn maybe_next(&self, offset: u128) -> Option<Self> {
			let offset: Option<u32> = offset.try_into().ok();
			offset
				.map(|offset: u32| u32::from(*self).checked_add(offset))
				.flatten()
				.map(Ipv4Addr::from)
		}
	}

	impl MaybeNext<i32> for IpAddr {
		fn maybe_next(&self, offset: i32) -> Option<Self> {
			match self {
				IpAddr::V4(v4) => v4.maybe_next(offset).map(IpAddr::V4),
				IpAddr::V6(v6) => v6.maybe_next(offset).map(IpAddr::V6),
			}
		}
	}

	impl MaybeNext<u32> for IpAddr {
		fn maybe_next(&self, offset: u32) -> Option<Self> {
			match self {
				IpAddr::V4(v4) => v4.maybe_next(offset).map(IpAddr::V4),
				IpAddr::V6(v6) => v6.maybe_next(offset).map(IpAddr::V6),
			}
		}
	}

	impl MaybeNext<u128> for IpAddr {
		fn maybe_next(&self, offset: u128) -> Option<Self> {
			match self {
				IpAddr::V4(v4) => v4.maybe_next(offset).map(IpAddr::V4),
				IpAddr::V6(v6) => v6.maybe_next(offset).map(IpAddr::V6),
			}
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		macro_rules! test_maybe_next {
			($test_fn_name:ident, $addr_type:ty, $addr:literal, $amount:literal, $expected:literal) => {
				#[test]
				fn $test_fn_name() {
					assert_eq!(
						$addr.parse::<$addr_type>().unwrap().maybe_next($amount),
						$expected.parse::<$addr_type>().ok()
					)
				}
			};
			($test_fn_name:ident, $addr_type:ty, $addr:literal, $amount:literal, None) => {
				#[test]
				fn $test_fn_name() {
					assert_eq!(
						$addr.parse::<$addr_type>().unwrap().maybe_next($amount),
						None
					)
				}
			};
		}

		// Basic sanity checks here that the behavior works as expected

		test_maybe_next!(v4_0u32, Ipv4Addr, "127.0.0.0", 0_u32, "127.0.0.0");
		test_maybe_next!(v4_1u32, Ipv4Addr, "127.0.0.0", 1_u32, "127.0.0.1");
		test_maybe_next!(v4_2u32, Ipv4Addr, "127.0.0.0", 2_u32, "127.0.0.2");

		test_maybe_next!(v6_0u32, Ipv6Addr, "2001:db8::", 0_u32, "2001:db8::0");
		test_maybe_next!(v6_1u32, Ipv6Addr, "2001:db8::", 1_u32, "2001:db8::1");
		test_maybe_next!(v6_2u32, Ipv6Addr, "2001:db8::", 2_u32, "2001:db8::2");

		test_maybe_next!(a_v4_0u32, IpAddr, "127.0.0.0", 0_u32, "127.0.0.0");
		test_maybe_next!(a_v4_1u32, IpAddr, "127.0.0.0", 1_u32, "127.0.0.1");
		test_maybe_next!(a_v4_2u32, IpAddr, "127.0.0.0", 2_u32, "127.0.0.2");

		test_maybe_next!(a_v6_0u32, IpAddr, "2001:db8::", 0_u32, "2001:db8::0");
		test_maybe_next!(a_v6_1u32, IpAddr, "2001:db8::", 1_u32, "2001:db8::1");
		test_maybe_next!(a_v6_2u32, IpAddr, "2001:db8::", 2_u32, "2001:db8::2");

		test_maybe_next!(v4_0i32, Ipv4Addr, "127.0.0.0", 0_i32, "127.0.0.0");
		test_maybe_next!(v4_1i32, Ipv4Addr, "127.0.0.0", 1_i32, "127.0.0.1");
		test_maybe_next!(v4_2i32, Ipv4Addr, "127.0.0.0", 2_i32, "127.0.0.2");

		test_maybe_next!(v6_0i32, Ipv6Addr, "2001:db8::", 0_i32, "2001:db8::0");
		test_maybe_next!(v6_1i32, Ipv6Addr, "2001:db8::", 1_i32, "2001:db8::1");
		test_maybe_next!(v6_2i32, Ipv6Addr, "2001:db8::", 2_i32, "2001:db8::2");

		test_maybe_next!(a_v4_0i32, IpAddr, "127.0.0.0", 0_i32, "127.0.0.0");
		test_maybe_next!(a_v4_1i32, IpAddr, "127.0.0.0", 1_i32, "127.0.0.1");
		test_maybe_next!(a_v4_2i32, IpAddr, "127.0.0.0", 2_i32, "127.0.0.2");

		test_maybe_next!(a_v6_0i32, IpAddr, "2001:db8::", 0_i32, "2001:db8::0");
		test_maybe_next!(a_v6_1i32, IpAddr, "2001:db8::", 1_i32, "2001:db8::1");
		test_maybe_next!(a_v6_2i32, IpAddr, "2001:db8::", 2_i32, "2001:db8::2");

		// Some more odd cases

		test_maybe_next!(
			v4_sub_1i32,
			Ipv4Addr,
			"127.0.0.0",
			-1_i32,
			"126.255.255.255"
		);

		test_maybe_next!(v4_min_minus_1i32, Ipv4Addr, "0.0.0.0", -1_i32, None);
		test_maybe_next!(v4_max_plus_1i32, Ipv4Addr, "255.255.255.255", 1_i32, None);

		test_maybe_next!(
			v6_sub_1i32,
			Ipv6Addr,
			"2001:db8::",
			-1_i32,
			"2001:db7:ffff:ffff:ffff:ffff:ffff:ffff"
		);

		test_maybe_next!(v6_min_minus_1i32, Ipv6Addr, "::", -1_i32, None);
		test_maybe_next!(
			v6_max_plus_1i32,
			Ipv6Addr,
			"ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff",
			1_i32,
			None
		);
	}
}

impl<Network, Address> Iterator for AddressIterator<Network, Address>
where
	Address: Copy + maybe_next::MaybeNext<u32>,
	Network: Copy + Contains,
	Network: From<Address>,
{
	type Item = Address;

	fn next(&mut self) -> Option<Self::Item> {
		let cur: Option<Self::Item> = self.cur;
		let next: Option<Self::Item> = cur.map(|cur| cur.maybe_next(1_u32)).flatten();

		match (cur, next) {
			(Some(cur), Some(next)) => {
				if self.net.contains(&cur) {
					self.cur = Some(next);
					Some(cur)
				} else {
					None
				}
			}
			(Some(cur), None) => {
				self.cur = next;
				Some(cur)
			}
			(None, _) => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::AddressIterator;

	mod netv4addr {
		use super::*;

		use crate::Netv4Addr;
		use std::net::Ipv4Addr;

		impl crate::Netv4Addr {
			pub fn iter(&self) -> AddressIterator<Netv4Addr, Ipv4Addr> {
				AddressIterator {
					net: *self,
					cur: Some(*self.addr()),
				}
			}
		}

		#[test]
		fn loopback_slash_32_produces_one_off() {
			let net: Netv4Addr = "127.0.16.0/32".parse().unwrap();

			let mut iterator: AddressIterator<Netv4Addr, Ipv4Addr> = net.iter();
			assert_eq!(iterator.next(), "127.0.16.0".parse::<Ipv4Addr>().ok());
			assert_eq!(iterator.next(), None);
		}

		#[test]
		fn loopback_slash_29_produces_one_off() {
			let net: Netv4Addr = "127.0.16.0/29".parse().unwrap();

			let mut iterator: AddressIterator<Netv4Addr, Ipv4Addr> = net.iter();
			assert_eq!(iterator.next(), "127.0.16.0".parse::<Ipv4Addr>().ok());
			assert_eq!(iterator.next(), "127.0.16.1".parse::<Ipv4Addr>().ok());
			assert_eq!(iterator.next(), "127.0.16.2".parse::<Ipv4Addr>().ok());
			assert_eq!(iterator.next(), "127.0.16.3".parse::<Ipv4Addr>().ok());
			assert_eq!(iterator.next(), "127.0.16.4".parse::<Ipv4Addr>().ok());
			assert_eq!(iterator.next(), "127.0.16.5".parse::<Ipv4Addr>().ok());
			assert_eq!(iterator.next(), "127.0.16.6".parse::<Ipv4Addr>().ok());
			assert_eq!(iterator.next(), "127.0.16.7".parse::<Ipv4Addr>().ok());
			assert_eq!(iterator.next(), None);
		}

		#[test]
		fn loopback_max_value_properly_stops() {
			let net: Netv4Addr = "255.255.255.255/31".parse().unwrap();

			let mut iterator: AddressIterator<Netv4Addr, Ipv4Addr> = net.iter();
			assert_eq!(iterator.next(), "255.255.255.254".parse::<Ipv4Addr>().ok());
			assert_eq!(iterator.next(), "255.255.255.255".parse::<Ipv4Addr>().ok());
			assert_eq!(iterator.next(), None);
		}
	}

	mod netaddr {
		use super::*;

		use crate::NetAddr;
		use std::net::IpAddr;

		impl crate::NetAddr {
			pub fn iter(&self) -> AddressIterator<NetAddr, IpAddr> {
				AddressIterator {
					net: *self,
					cur: Some(self.addr()),
				}
			}
		}

		mod v4 {
			use super::*;

			#[test]
			fn loopback_slash_32_produces_one_off() {
				let net: NetAddr = "127.0.16.0/32".parse().unwrap();

				let mut iterator: AddressIterator<NetAddr, IpAddr> = net.iter();
				assert_eq!(iterator.next(), "127.0.16.0".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), None);
			}

			#[test]
			fn loopback_slash_29_produces_all_ips_in_network() {
				let net: NetAddr = "127.0.16.0/29".parse().unwrap();

				let mut iterator: AddressIterator<NetAddr, IpAddr> = net.iter();
				assert_eq!(iterator.next(), "127.0.16.0".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "127.0.16.1".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "127.0.16.2".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "127.0.16.3".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "127.0.16.4".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "127.0.16.5".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "127.0.16.6".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "127.0.16.7".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), None);
			}

			#[test]
			fn loopback_max_value_properly_stops() {
				let net: NetAddr = "255.255.255.255/31".parse().unwrap();

				let mut iterator: AddressIterator<NetAddr, IpAddr> = net.iter();
				assert_eq!(iterator.next(), "255.255.255.254".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "255.255.255.255".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), None);
			}
		}

		mod v6 {
			use super::*;

			#[test]
			#[test]
			fn slash_128_produces_one_off() {
				let net: NetAddr = "2001:db8::1/128".parse().unwrap();

				let mut iterator: AddressIterator<NetAddr, IpAddr> = net.iter();
				assert_eq!(iterator.next(), "2001:db8::1".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), None);
			}

			#[test]
			fn slash_125_produces_all_ips_in_network() {
				let net: NetAddr = "2001:db8::1/125".parse().unwrap();

				let mut iterator: AddressIterator<NetAddr, IpAddr> = net.iter();
				assert_eq!(iterator.next(), "2001:db8::0".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "2001:db8::1".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "2001:db8::2".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "2001:db8::3".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "2001:db8::4".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "2001:db8::5".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "2001:db8::6".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "2001:db8::7".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), None);
			}

			#[test]
			fn loopback_max_value_properly_stops() {
				let net: NetAddr = "ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff/127"
					.parse()
					.unwrap();

				let mut iterator: AddressIterator<NetAddr, IpAddr> = net.iter();
				assert_eq!(
					iterator.next(),
					"ffff:ffff:ffff:ffff:ffff:ffff:ffff:fffe"
						.parse::<IpAddr>()
						.ok()
				);
				assert_eq!(
					iterator.next(),
					"ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff"
						.parse::<IpAddr>()
						.ok()
				);
				assert_eq!(iterator.next(), None);
			}
		}
	}
}

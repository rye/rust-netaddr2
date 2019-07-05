use core::str::FromStr;
use std::net::IpAddr;

#[derive(Copy, Clone, Hash, Debug)]
pub struct NetAddr {
	pub netmask: IpAddr,
	pub network: IpAddr,
}

/// Mask the given referenced `addr` with the given `mask`, returning a new
/// `IpAddr`.
///
/// Both `addr` and `mask` must be of the same `enum` variant for the
/// operation to succeed.
///
/// # Panics
///
/// This function will panic if the provided `addr` and `mask` are not of the
/// same enum variant.
///
/// ```should_panic
/// # use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
/// let addr: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
/// let mask: IpAddr = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 255, 0, 0, 0));
/// netaddr2::mask(&addr, &mask);
/// ```
pub fn mask(addr: &IpAddr, mask: &IpAddr) -> IpAddr {
	match (addr, mask) {
		(IpAddr::V4(addr), IpAddr::V4(mask)) => {
			let (addr, mask): (u32, u32) = ((*addr).into(), (*mask).into());
			IpAddr::V4((addr & mask).into())
		}
		(IpAddr::V6(addr), IpAddr::V6(mask)) => {
			let (addr, mask): (u128, u128) = ((*addr).into(), (*mask).into());
			IpAddr::V6((addr & mask).into())
		}
		(_, _) => panic!("mismatched types"),
	}
}

impl NetAddr {
	pub fn contains(&self, other: &IpAddr) -> bool {
		match (self.netmask, self.network, other) {
			(IpAddr::V4(netmask), IpAddr::V4(network), IpAddr::V4(other)) => {
				let other: u32 = (*other).into();
				let mask: u32 = netmask.into();
				let network: u32 = network.into();

				(other & mask) == network
			}
			(IpAddr::V6(netmask), IpAddr::V6(network), IpAddr::V6(other)) => {
				let other: u128 = (*other).into();
				let mask: u128 = netmask.into();
				let network: u128 = network.into();

				(other & mask) == network
			}
			(_, _, _) => panic!("mismatched address types"),
		}
	}
}

impl FromStr for NetAddr {
	type Err = std::net::AddrParseError;

	fn from_str(string: &str) -> Result<Self, std::net::AddrParseError> {
		let split: Vec<&str> = string.split(|c| c == '/' || c == ' ').collect();

		let lhs = split[0];
		let rhs = split[1];

		let address: IpAddr = lhs.parse()?;

		let as_u32 = rhs.parse::<u32>();
		let as_ipaddr = rhs.parse::<IpAddr>();

		match (as_u32, as_ipaddr) {
			(Ok(cidr_prefix_length), _) => match address {
				IpAddr::V4(_addr) => {
					let mask: u32 = 0xff_ff_ff_ff_u32
						^ match 0xff_ff_ff_ff_u32.checked_shr(cidr_prefix_length) {
							Some(k) => k,
							None => 0_u32,
						};

					let netmask = IpAddr::V4(mask.into());

					let network = crate::mask(&address, &netmask);

					Ok(Self { network, netmask })
				}
				IpAddr::V6(_) => {
					let mask: u128 = 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_u128
						^ match 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_u128.checked_shr(cidr_prefix_length) {
							Some(k) => k,
							None => 0_u128,
						};

					let netmask = IpAddr::V6(mask.into());

					let network = crate::mask(&address, &netmask);

					Ok(Self { network, netmask })
				}
			},
			(Err(_), Ok(netmask)) => {
				let network = crate::mask(&address, &netmask);

				Ok(Self { network, netmask })
			}
			(Err(_), Err(e)) => Err(e),
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::NetAddr;

	use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

	#[test]
	fn parse_v4_correct_network() {
		let net: NetAddr = "192.0.2.0/32".parse().unwrap();
		assert_eq!(net.netmask, IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255)));
		assert_eq!(net.network, IpAddr::V4(Ipv4Addr::new(192, 0, 2, 0)));
	}

	#[test]
	fn contains_v4_correct() {
		let net: NetAddr = "127.0.0.1/8".parse().unwrap();
		assert!(net.contains(&IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))));
		assert!(net.contains(&IpAddr::V4(Ipv4Addr::new(127, 127, 255, 1))));
		assert!(!net.contains(&IpAddr::V4(Ipv4Addr::new(64, 0, 0, 0))));
	}

	#[test]
	fn parse_v4_localhost() {
		let net: NetAddr = "127.0.0.1/8".parse().unwrap();
		assert_eq!(net.netmask, IpAddr::V4(Ipv4Addr::new(255, 0, 0, 0)));
		assert_eq!(net.network, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 0)));
	}

	#[test]
	fn parse_v4_cidr_22() {
		let net: NetAddr = "192.168.16.1/22".parse().unwrap();
		assert_eq!(net.netmask, IpAddr::V4(Ipv4Addr::new(255, 255, 252, 0)));
		assert_eq!(net.network, IpAddr::V4(Ipv4Addr::new(192, 168, 16, 0)));
	}

	#[test]
	fn parse_v4_extended_localhost() {
		let net: NetAddr = "127.0.0.1 255.0.0.0".parse().unwrap();
		assert_eq!(net.netmask, IpAddr::V4(Ipv4Addr::new(255, 0, 0, 0)));
		assert_eq!(net.network, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 0)));
	}

	#[test]
	fn parse_v4_slashed_localhost() {
		let net: NetAddr = "127.0.0.1/255.0.0.0".parse().unwrap();
		assert_eq!(net.netmask, IpAddr::V4(Ipv4Addr::new(255, 0, 0, 0)));
		assert_eq!(net.network, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 0)));
	}

	#[test]
	fn parse_v6_cidr_8() {
		let net: NetAddr = "ff02::1/8".parse().unwrap();
		assert_eq!(
			net.netmask,
			IpAddr::V6(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0x0000))
		);
		assert_eq!(
			net.network,
			IpAddr::V6(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0x0000))
		);
	}

	#[test]
	fn parse_v6_cidr_128() {
		let net: NetAddr = "ff02::1/128".parse().unwrap();
		assert_eq!(
			net.netmask,
			IpAddr::V6(Ipv6Addr::new(
				0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff
			))
		);
		assert_eq!(
			net.network,
			IpAddr::V6(Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 0x0001))
		);
	}
}

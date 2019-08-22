use super::{NetAddr, NetAddrError};
use crate::Netv4Addr;
use crate::Netv6Addr;
use core::str::FromStr;

impl FromStr for NetAddr {
	type Err = NetAddrError;

	fn from_str(string: &str) -> Result<Self, NetAddrError> {
		let as_v4: Result<Netv4Addr, NetAddrError> = string.parse::<Netv4Addr>();
		let as_v6: Result<Netv6Addr, NetAddrError> = string.parse::<Netv6Addr>();

		match (as_v4, as_v6) {
			(Ok(v4), _) => Ok(Self::V4(v4)),
			(_, Ok(v6)) => Ok(Self::V6(v6)),
			(Err(_e4), Err(e6)) => Err(e6),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn invalid_is_safe() {
		let _: Result<NetAddr, _> = "zoop".parse::<NetAddr>();
	}

	#[test]
	fn addr_only_returns_full_bitstring() {
		let net: NetAddr = "127.0.0.1/zoop".parse().unwrap();
		assert_eq!(net, "127.0.0.1/32".parse().unwrap());
		let net: NetAddr = "ff02::1/zoop".parse().unwrap();
		assert_eq!(net, "ff02::1/128".parse().unwrap());
	}

	#[test]
	fn non_addr_passes_out_error() {
		let result = "zoop".parse::<NetAddr>();
		assert_eq!(
			result,
			Err(NetAddrError::ParseError(
				"could not split provided input".to_string()
			))
		);
	}

	mod v4 {
		use super::NetAddr;
		use std::net::{IpAddr, Ipv4Addr};

		#[test]
		fn correct_network() {
			let net: NetAddr = "192.0.2.0/32".parse().unwrap();
			assert_eq!(net.mask(), IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255)));
			assert_eq!(net.addr(), IpAddr::V4(Ipv4Addr::new(192, 0, 2, 0)));
		}

		#[test]
		fn localhost_8() {
			let net: NetAddr = "127.0.0.1/8".parse().unwrap();
			assert_eq!(net.mask(), IpAddr::V4(Ipv4Addr::new(255, 0, 0, 0)));
			assert_eq!(net.addr(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 0)));
		}

		#[test]
		fn cidr_22() {
			let net: NetAddr = "192.168.16.1/22".parse().unwrap();
			assert_eq!(net.mask(), IpAddr::V4(Ipv4Addr::new(255, 255, 252, 0)));
			assert_eq!(net.addr(), IpAddr::V4(Ipv4Addr::new(192, 168, 16, 0)));
		}

		#[test]
		fn extended_localhost() {
			let net: NetAddr = "127.0.0.1 255.0.0.0".parse().unwrap();
			assert_eq!(net.mask(), IpAddr::V4(Ipv4Addr::new(255, 0, 0, 0)));
			assert_eq!(net.addr(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 0)));
		}

		#[test]
		fn slashed_localhost() {
			let net: NetAddr = "127.0.0.1/255.0.0.0".parse().unwrap();
			assert_eq!(net.mask(), IpAddr::V4(Ipv4Addr::new(255, 0, 0, 0)));
			assert_eq!(net.addr(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 0)));
		}
	}

	mod v6 {
		use super::NetAddr;
		use std::net::{IpAddr, Ipv6Addr};

		#[test]
		fn v6_cidr_8() {
			let net: NetAddr = "ff02::1/8".parse().unwrap();
			assert_eq!(
				net.mask(),
				IpAddr::V6(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0x0000))
			);
			assert_eq!(
				net.addr(),
				IpAddr::V6(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0x0000))
			);
		}

		#[test]
		fn v6_cidr_128() {
			let net: NetAddr = "ff02::1/128".parse().unwrap();
			assert_eq!(
				net.mask(),
				IpAddr::V6(Ipv6Addr::new(
					0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff
				))
			);
			assert_eq!(
				net.addr(),
				IpAddr::V6(Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 0x0001))
			);
		}

		#[test]
		fn v6_extended() {
			let net: NetAddr = "ff02::1 ffff::0".parse().unwrap();
			assert_eq!(
				net.mask(),
				IpAddr::V6(Ipv6Addr::new(0xffff, 0, 0, 0, 0, 0, 0, 0))
			);
			assert_eq!(
				net.addr(),
				IpAddr::V6(Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 0))
			);
		}

		#[test]
		fn v6_slashed() {
			let net: NetAddr = "ff02::1/128".parse().unwrap();
			assert_eq!(
				net.mask(),
				IpAddr::V6(Ipv6Addr::new(
					0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff
				))
			);
			assert_eq!(
				net.addr(),
				IpAddr::V6(Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 0x0001))
			);
		}
	}
}

use core::str::FromStr;
use std::net::IpAddr;

pub struct NetAddr {
	pub address: IpAddr,
	pub netmask: IpAddr,
}

impl NetAddr {
	pub fn network(&self) -> IpAddr {
		match (self.address, self.netmask) {
			(std::net::IpAddr::V4(address), std::net::IpAddr::V4(netmask)) => {
				let network_masked: Vec<u8> = address
					.octets()
					.iter()
					.zip(netmask.octets().iter())
					.map(|(addr_oct, netm_oct): (&u8, &u8)| -> u8 { addr_oct & netm_oct })
					.collect();

				let mut network_octets: [u8; 4] = [0_u8; 4];
				network_octets.copy_from_slice(&network_masked);

				network_octets.into()
			}
			(std::net::IpAddr::V6(address), std::net::IpAddr::V6(netmask)) => {
				let network_masked: Vec<u8> = address
					.octets()
					.iter()
					.zip(netmask.octets().iter())
					.map(|(addr_oct, netm_oct): (&u8, &u8)| -> u8 { addr_oct & netm_oct })
					.collect();

				let mut network_octets: [u8; 16] = [0_u8; 16];
				network_octets.copy_from_slice(&network_masked);

				network_octets.into()
			}
			(_, _) => panic!("mismatched address/netmask types"),
		}
	}
}

impl FromStr for NetAddr {
	type Err = std::net::AddrParseError;

	fn from_str(string: &str) -> Result<Self, std::net::AddrParseError> {
		let split: Vec<&str> = string.split('/').collect();

		let lhs = split[0];
		let rhs = split[1];

		let lhs: IpAddr = lhs.parse()?;
		let rhs: u32 = rhs.parse().expect("expected cidr rhs of /");

		match lhs {
			IpAddr::V4(_addr) => {
				let mask: u32 = 0xff_ff_ff_ff_u32
					^ match 0xff_ff_ff_ff_u32.checked_shr(rhs) {
						Some(k) => k,
						None => 0_u32,
					};
				let netmask = IpAddr::V4(mask.into());

				Ok(Self {
					address: lhs,
					netmask,
				})
			}
			IpAddr::V6(_) => {
				let mask: u128 = 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_u128
					^ match 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_u128.checked_shr(rhs) {
						Some(k) => k,
						None => 0_u128,
					};

				let netmask = IpAddr::V6(mask.into());

				Ok(Self {
					address: lhs,
					netmask,
				})
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::NetAddr;

	use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

	#[test]
	fn parse_v4_localhost() {
		let net: NetAddr = "127.0.0.1/8".parse().unwrap();
		assert_eq!(net.address, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
		assert_eq!(net.netmask, IpAddr::V4(Ipv4Addr::new(255, 0, 0, 0)));
		assert_eq!(net.network(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 0)));
	}

	#[test]
	fn parse_v4_cidr_22() {
		let net: NetAddr = "192.168.16.1/22".parse().unwrap();
		assert_eq!(net.address, IpAddr::V4(Ipv4Addr::new(192, 168, 16, 1)));
		assert_eq!(net.netmask, IpAddr::V4(Ipv4Addr::new(255, 255, 252, 0)));
		assert_eq!(net.network(), IpAddr::V4(Ipv4Addr::new(192, 168, 16, 0)));
	}

	#[test]
	fn parse_v6_cidr_8() {
		let net: NetAddr = "ff02::1/8".parse().unwrap();
		assert_eq!(
			net.address,
			IpAddr::V6(Ipv6Addr::new(
				0xff02, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001
			))
		);
		assert_eq!(
			net.netmask,
			IpAddr::V6(Ipv6Addr::new(
				0xff00, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000
			))
		);
		assert_eq!(
			net.network(),
			IpAddr::V6(Ipv6Addr::new(
				0xff00, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000
			))
		);
	}

	#[test]
	fn parse_v6_cidr_128() {
		let net: NetAddr = "ff02::1/128".parse().unwrap();
		assert_eq!(
			net.address,
			IpAddr::V6(Ipv6Addr::new(
				0xff02, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001
			))
		);
		assert_eq!(
			net.netmask,
			IpAddr::V6(Ipv6Addr::new(
				0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff
			))
		);
		assert_eq!(
			net.network(),
			IpAddr::V6(Ipv6Addr::new(
				0xff02, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001
			))
		);
	}
}

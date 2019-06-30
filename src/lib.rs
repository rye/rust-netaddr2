use std::net::IpAddr;

pub struct NetAddr {
	pub address: IpAddr,
	pub netmask: IpAddr,
}

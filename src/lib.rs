//! A crate for storing, manipulating, and reasoning about network addresses.
//!
//! `netaddr2` arose from a need to mask and subnet IP space logically similar
//! to the way that networking gear does, for example in simulating Access
//! Control List (ACL) applicability.
//!
//! This crate enables programmers to ask questions like:
//!
//! - Does this IP address belong to this network?
//!   e.g.,
//!   ```rust
//!   # use std::net::IpAddr;
//!   # use netaddr2::{Contains, NetAddr};
//!   let addr: std::net::IpAddr = "192.168.1.7".parse().unwrap();
//!   let net: netaddr2::NetAddr = "192.168.1.0/24".parse().unwrap();
//!   assert!(net.contains(&addr));
//!
//!   let other_addr: std::net::IpAddr = "1.1.1.1".parse().unwrap();
//!   assert!(!net.contains(&other_addr));
//!   ```
//!
//! - Does this network contain this other network?
//!   ```rust
//!   # use netaddr2::{Contains, NetAddr};
//!   let net: netaddr2::NetAddr = "192.168.0.0/16".parse().unwrap();
//!   let subnet: netaddr2::NetAddr = "192.168.17.23/24".parse().unwrap();
//!   assert!(net.contains(&subnet));
//!
//!   let all: netaddr2::NetAddr = "0.0.0.0/0".parse().unwrap();
//!   assert!(!net.contains(&all));
//!   ```
//!
//! You can also build IPv4- and IPv6-specific variants of the [`NetAddr`] struct;
//! these are named [`Netv4Addr`] and [`Netv6Addr`] respectively.
//!
//! The API is strikingly similar to that of the `std::net::Ip.*Addr` structs,
//! and users who have used that set of structs will hopefully find this API
//! quite naturally similar.

#![forbid(unsafe_code)]

#[cfg(test)]
#[macro_use]
mod util {
	/// A macro to parse and unwrap as a type.
	macro_rules! pu {
		($value:literal # $t:ty) => {
			$value.parse::<$t>().unwrap()
		};
	}
}

mod error;
mod netaddr;
mod netv4addr;
mod netv6addr;
mod traits;

pub use error::*;
pub use netaddr::*;
pub use netv4addr::*;
pub use netv6addr::*;
pub use traits::*;

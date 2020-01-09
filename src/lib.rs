//! A crate for parsing, representing, and manipulating network addresses.
//!
//! `netaddr2` arose from a need to mask and subnet IP space logically similar
//! to the way that routers and network interface cards (NICs) do.  This enables
//! the user to ask questions like:
//!
//! - Does this `IpAddr` belong to this `NetAddr`? (That is, does this specific
//!   address happen to lie within the network/netmask given by this `NetAddr`?)
//!
//! - Does this network contain this other network?
//!
//! The API is strikingly similar to that of the `std::net::Ip.*Addr` structs,
//! and users who have used that set of structs will hopefully find this API
//! quite naturally similar.

#[cfg(test)]
#[macro_use]
mod util {
	macro_rules! pu {
		($value:literal # $t:ty) => {
			$value.parse::<$t>().unwrap()
		};
	}
}

mod error;
mod iter;
mod netaddr;
mod netv4addr;
mod netv6addr;
mod traits;

pub use error::*;
pub use iter::*;
pub use netaddr::*;
pub use netv4addr::*;
pub use netv6addr::*;
pub use traits::*;

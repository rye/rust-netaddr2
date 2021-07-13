//! Traits for iteration over `NetAddr` types.

use crate::traits::Contains;

/// An iterator over a network's _contained addresses_.
///
/// Starts from the "network address", the first address contained in the range, and iterates to the
/// end of the network, the last address contained in the specified network.
///
/// # Examples
///
/// ```rust
/// # use netaddr2::{Netv4Addr,AddressIterator};
/// # use std::net::Ipv4Addr;
/// let network: Netv4Addr = "10.0.0.1/30".parse().unwrap();
///
/// let mut iter = network.addresses();
///
/// // A /30 network only contains four addresses.
/// assert_eq!(iter.next(), Some("10.0.0.0".parse().unwrap()));
/// assert_eq!(iter.next(), Some("10.0.0.1".parse().unwrap()));
/// assert_eq!(iter.next(), Some("10.0.0.2".parse().unwrap()));
/// assert_eq!(iter.next(), Some("10.0.0.3".parse().unwrap()));
/// assert_eq!(iter.next(), None);
/// ```
pub struct AddressIterator<Network, Address>
where
	Network: Contains<Address> + From<Address>,
{
	pub(crate) net: Network,
	pub(crate) cur: Option<Address>,
}

mod address;
pub use address::*;

#[cfg(feature = "unstable")]
mod sibling;
#[cfg(feature = "unstable")]
pub use sibling::*;

#[cfg(feature = "unstable")]
pub struct SubnetIterator<Network, Subnet> {
	pub(crate) net: Network,
	pub(crate) cur: Option<Subnet>,
}

#[cfg(feature = "unstable")]
mod subnet;

#[cfg(feature = "unstable")]
pub use subnet::*;

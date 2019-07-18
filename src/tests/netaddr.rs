use crate::*;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[test]
fn is_send() {
	fn assert_send<T: Send>() {}
	assert_send::<NetAddr>();
}

#[test]
fn is_sync() {
	fn assert_sync<T: Sync>() {}
	assert_sync::<NetAddr>();
}

#[cfg(test)]
mod broadcast;
#[cfg(test)]
mod cmp;
#[cfg(test)]
mod contains;
#[cfg(test)]
mod from;
#[cfg(test)]
mod merge;
#[cfg(test)]
mod parse;

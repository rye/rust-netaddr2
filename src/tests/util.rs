use crate::*;
use std::net::{Ipv4Addr, Ipv6Addr};

use rand::Rng;

fn rnd_ipv4addr() -> Ipv4Addr {
	let mut rng = rand::thread_rng();
	[
		rng.gen_range(0x00u8, 0xffu8),
		rng.gen_range(0x00u8, 0xffu8),
		rng.gen_range(0x00u8, 0xffu8),
		rng.gen_range(0x00u8, 0xffu8),
	]
	.into()
}

fn rnd_ipv6addr() -> Ipv6Addr {
	let mut rng = rand::thread_rng();
	[
		rng.gen_range(0x00u8, 0xffu8),
		rng.gen_range(0x00u8, 0xffu8),
		rng.gen_range(0x00u8, 0xffu8),
		rng.gen_range(0x00u8, 0xffu8),
		rng.gen_range(0x00u8, 0xffu8),
		rng.gen_range(0x00u8, 0xffu8),
		rng.gen_range(0x00u8, 0xffu8),
		rng.gen_range(0x00u8, 0xffu8),
		rng.gen_range(0x00u8, 0xffu8),
		rng.gen_range(0x00u8, 0xffu8),
		rng.gen_range(0x00u8, 0xffu8),
		rng.gen_range(0x00u8, 0xffu8),
		rng.gen_range(0x00u8, 0xffu8),
		rng.gen_range(0x00u8, 0xffu8),
		rng.gen_range(0x00u8, 0xffu8),
		rng.gen_range(0x00u8, 0xffu8),
	]
	.into()
}

fn rnd_v4_cidrlen() -> u32 {
	let mut rng = rand::thread_rng();
	rng.gen_range(0, 32)
}

fn rnd_v6_cidrlen() -> u32 {
	let mut rng = rand::thread_rng();
	rng.gen_range(0, 128)
}

fn gen_v4_mask(cidr_len: u32) -> Ipv4Addr {
	(0xff_ff_ff_ff_u32
		.checked_shl(32 - cidr_len)
		.unwrap_or(0x00_00_00_00_u32))
	.into()
}

mod gen_v4_mask;

fn gen_v6_mask(cidr_len: u32) -> Ipv6Addr {
	(0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_u128
		.checked_shl(128 - cidr_len)
		.unwrap_or(0x0000_0000_0000_0000_0000_0000_0000_0000_u128))
	.into()
}

mod gen_v6_mask;

pub(crate) fn rnd_v4_cidr() -> (String, Ipv4Addr, Ipv4Addr) {
	let addr: Ipv4Addr = rnd_ipv4addr();

	let ones: u32 = rnd_v4_cidrlen();

	let mask: Ipv4Addr = gen_v4_mask(ones);

	(format!("{}/{}", addr, ones), addr.mask(&mask), mask)
}

pub(crate) fn rnd_v6_cidr() -> (String, Ipv6Addr, Ipv6Addr) {
	let addr: Ipv6Addr = rnd_ipv6addr();

	let ones: u32 = rnd_v6_cidrlen();

	let mask: Ipv6Addr = gen_v6_mask(ones);

	(format!("{}/{}", addr, ones), addr.mask(&mask), mask)
}

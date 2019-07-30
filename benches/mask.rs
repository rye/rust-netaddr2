#![feature(test)]

extern crate test;

extern crate netaddr2;

use netaddr2::Mask;

use std::net::Ipv4Addr;

#[bench]
fn bench_mask(bencher: &mut test::Bencher) {
	let a: Ipv4Addr = "127.0.0.1".parse().unwrap();
	let b: Ipv4Addr = "255.255.255.0".parse().unwrap();

	bencher.iter(|| test::black_box(a.mask(&b)))
}

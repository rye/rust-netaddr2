#![feature(test)]

extern crate test;

extern crate netaddr2;
use netaddr2::mask;

use std::net::IpAddr;

#[bench]
fn bench_mask(bencher: &mut test::Bencher) {
	let a: IpAddr = "127.0.0.1".parse().unwrap();
	let b: IpAddr = "255.255.255.0".parse().unwrap();

	bencher.iter(|| test::black_box(mask(&a, &b)))
}

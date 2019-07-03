#![feature(test)]

extern crate test;

extern crate netaddr2;
use netaddr2::NetAddr;

#[bench]
fn bench_parse_v4(b: &mut test::Bencher) {
	let net = "127.0.0.1/8";
	b.iter(|| assert!(net.parse::<NetAddr>().is_ok()))
}

#[bench]
fn bench_parse_v6(b: &mut test::Bencher) {
	let net = "2001:db8:dead:beef::/32";
	b.iter(|| assert!(net.parse::<NetAddr>().is_ok()))
}

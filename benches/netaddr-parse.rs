#![feature(test)]

extern crate test;

extern crate netaddr2;
use netaddr2::NetAddr;

#[bench]
fn bench_parse_v4_cidr(b: &mut test::Bencher) {
	let net = "127.0.0.1/8";
	b.iter(|| assert!(net.parse::<NetAddr>().is_ok()))
}

#[bench]
fn bench_parse_v4_extended(b: &mut test::Bencher) {
	let net = "127.0.0.1 255.0.0.0";
	b.iter(|| assert!(net.parse::<NetAddr>().is_ok()))
}

#[bench]
fn bench_parse_v4_extended_slash(b: &mut test::Bencher) {
	let net = "127.0.0.1/255.0.0.0";
	b.iter(|| assert!(net.parse::<NetAddr>().is_ok()))
}

#[bench]
fn bench_parse_v6(b: &mut test::Bencher) {
	let net = "2001:db8:dead:beef::/32";
	b.iter(|| assert!(net.parse::<NetAddr>().is_ok()))
}

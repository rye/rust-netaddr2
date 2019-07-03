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
fn bench_parse_v6_cidr(b: &mut test::Bencher) {
	let net = "2001:db8:dead:beef::/32";
	b.iter(|| assert!(net.parse::<NetAddr>().is_ok()))
}

#[bench]
fn bench_parse_v6_extended(b: &mut test::Bencher) {
	let net = "2001:db8:dead:beef::0 ffff:ffff::0";
	b.iter(|| assert!(net.parse::<NetAddr>().is_ok()))
}

#[bench]
fn bench_parse_v6_extended_slash(b: &mut test::Bencher) {
	let net = "2001:db8:dead:beef::0/ffff:ffff::0";
	b.iter(|| assert!(net.parse::<NetAddr>().is_ok()))
}

#[bench]
fn bench_parse_v6_cidr_full(b: &mut test::Bencher) {
	let net = "2001:0db8:dead:beef:0000:0000:0000:0000/32";
	b.iter(|| assert!(net.parse::<NetAddr>().is_ok()))
}

#[bench]
fn bench_parse_v6_extended_full(b: &mut test::Bencher) {
	let net = "2001:0db8:dead:beef:0000:0000:0000:0000 ffff:ffff:0000:0000:0000:0000:0000:0000";
	b.iter(|| assert!(net.parse::<NetAddr>().is_ok()))
}

#[bench]
fn bench_parse_v6_extended_slash_full(b: &mut test::Bencher) {
	let net = "2001:0db8:dead:beef:0000:0000:0000:0000/ffff:ffff:0000:0000:0000:0000:0000:0000";
	b.iter(|| assert!(net.parse::<NetAddr>().is_ok()))
}

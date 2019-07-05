#![feature(test)]

extern crate test;

extern crate netaddr2;
use netaddr2::NetAddr;

#[bench]
fn bench_partialeq_equal(b: &mut test::Bencher) {
	let net_a: NetAddr = "127.0.0.1/8".parse().unwrap();
	let net_b: NetAddr = "127.0.0.1/8".parse().unwrap();

	b.iter(|| assert!(net_a.eq(&net_b)))
}

#[bench]
fn bench_partialeq_unequal_networks(b: &mut test::Bencher) {
	let net_a: NetAddr = "127.0.0.1/8".parse().unwrap();
	let net_b: NetAddr = "128.0.0.1/8".parse().unwrap();

	b.iter(|| assert!(!net_a.eq(&net_b)))
}

#[bench]
fn bench_partialeq_unequal_netmasks(b: &mut test::Bencher) {
	let net_a: NetAddr = "127.0.0.1/8".parse().unwrap();
	let net_b: NetAddr = "127.0.0.1/7".parse().unwrap();

	b.iter(|| assert!(!net_a.eq(&net_b)))
}

#[bench]
fn bench_partialeq_unequal(b: &mut test::Bencher) {
	let net_a: NetAddr = "127.0.0.1/8".parse().unwrap();
	let net_b: NetAddr = "0.0.0.0/4".parse().unwrap();

	b.iter(|| assert!(!net_a.eq(&net_b)))
}

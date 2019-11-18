#![feature(test)]

extern crate test;

extern crate netaddr2;
use netaddr2::{Contains, NetAddr};

use std::net::IpAddr;

#[bench]
fn bench_contains_ip_contained(b: &mut test::Bencher) {
	let net: NetAddr = "127.0.0.1/8".parse().unwrap();
	let contained_ip: IpAddr = "127.0.0.1".parse().unwrap();
	b.iter(|| assert!(net.contains(&contained_ip)))
}

#[bench]
fn bench_contains_net_contained(b: &mut test::Bencher) {
	let net: NetAddr = "127.0.0.1/8".parse().unwrap();
	let contained_net: NetAddr = "127.127.0.0/16".parse().unwrap();
	b.iter(|| assert!(net.contains(&contained_net)))
}

#[bench]
fn bench_contains_ip_uncontained(b: &mut test::Bencher) {
	let net: NetAddr = "127.0.0.1/8".parse().unwrap();
	let uncontained_ip: IpAddr = "5.20.72.4".parse().unwrap();
	b.iter(|| assert!(!net.contains(&uncontained_ip)))
}

#[bench]
fn bench_contains_net_uncontained(b: &mut test::Bencher) {
	let net: NetAddr = "127.0.0.1/8".parse().unwrap();
	let uncontained_net: NetAddr = "5.20.72.4/24".parse().unwrap();
	b.iter(|| assert!(!net.contains(&uncontained_net)))
}

# rust-netaddr2 (`netaddr2`) &bull; [![Build Status](https://travis-ci.org/rye/rust-netaddr2.svg?branch=master)](https://travis-ci.org/rye/rust-netaddr2) [![codecov](https://codecov.io/gh/rye/rust-netaddr2/branch/master/graph/badge.svg)](https://codecov.io/gh/rye/rust-netaddr2) [![version](https://img.shields.io/crates/v/netaddr2)](https://crates.io/crates/netaddr2) [![downloads](https://img.shields.io/crates/d/netaddr2)](https://crates.io/crates/netaddr2) [![docs.rs](https://docs.rs/netaddr2/badge.svg)](https://docs.rs/netaddr2)

This crate is meant as a replacement for an existing reimplementation of various "netaddr" libraries that other languages have.
There does exist another `netaddr` crate, however the author of this crate did not respond when asked about maintainership status.

## What it does

`NetAddr` arose out of a need to mask and subnet IP space in a manner identical to that which routers and network interfaces do.
Its utility may be most fully realized in the development of tooling for such purposes.

## Usage

There are a few ways to use this library.
Perhaps most ergonomical of these is to use the `FromStr` trait:

```rust
let net: NetAddr = "ff02::1/128".parse().unwrap();
let net: Netv4Addr = "203.0.113.19/29".parse().unwrap();
```

You can do some operations with these parsed structures, like checking address containment:

```rust
let net: NetAddr = "10.10.10.0/24".parse().unwrap();
let addr: IpAddr = "10.10.10.1".parse().unwrap();
assert!(net.contains(&addr));
```

(More options will be added eventually.)

## Vision

This crate aims to be as _simple_ and _straightforward_ as possible.
We accomplish this by mirroring the structure of the `std::net::Ip.*Addr` data structures.
Most of the operations on `NetAddr` structs are implemented through the use of _traits_ which are implemented both on the main structures and on the enum that bridges them.
These are also implemented, where appropriate, for standard library structures.

This crate has no dependencies, and will not accept any unless required for `no_std` support.
The only part of this crate that uses `std` is the part that bridges with `std::net::IpAddr`, so a potential contribution would be to generalize `std::net::IpAddr` in a `no_std` environment.

## Maintenance Status

This codebase is still not feature-complete.
Check out the issue tracker if you want to contribute, and don't hesistate to ask for something in an Issue.
That said, the business logic is tested and should work.
We will release version 1.0 when the GitHub milestone has been fully resolved.

## License

> Copyright &copy; 2019 Kristofer J. Rye

This software is released under either of:

- The Apache License, Version 2.0, (http://www.apache.org/licenses/LICENSE-2.0) or
- The MIT License, (http://opensource.org/licenses/MIT)

at your discretion.
Please see the license file ([LICENSE.md](LICENSE.md)) for more information.


## Acknowledgements

We would like to thank the developers of the `netaddr` Ruby gem for inspiring the development and ergonomics of this project.

# rust-netaddr2 (`netaddr2`) &bull; [![Build Status](https://travis-ci.org/rye/rust-netaddr2.svg?branch=master)](https://travis-ci.org/rye/rust-netaddr2) [![codecov](https://codecov.io/gh/rye/rust-netaddr2/branch/master/graph/badge.svg)](https://codecov.io/gh/rye/rust-netaddr2) [![version](https://img.shields.io/crates/v/netaddr2)](https://crates.io/crates/netaddr2) [![downloads](https://img.shields.io/crates/d/netaddr2)](https://crates.io/crates/netaddr2)

This crate is meant as a replacement for an existing reimplementation of various "netaddr" libraries that other languages have.
There does exist another `netaddr` crate, however the author of this crate did not respond when asked about maintainership status.

This crate aims to be as _simple_ and _straightforward_ as possible.
We accomplish this by mirroring the internal `std::net::Ip.*Addr` structure.
So, the `netaddr2::NetAddr` enum has a `V4` and `V6` variant which each respectively contain `netaddr2::Netv4Addr` and `netaddr2::Netv6Addr` unit-tuples.
Most of the operations are implemented through the use of _traits_ which are implemented both on the main structures and on the enum that bridges them.
We have no dependencies (except `std`) by default and will only accept additional dependencies on an opt-in basis.
Support for `no_std` will come at a later date if `std::net::IpAddr` can be ported.
(PRs are definitely welcome.)

## Usage

There are a few ways to use this library.
Perhaps most ergonomical of these is to use the `FromStr` trait:

```rust
let net: NetAddr = "ff02::1/128".parse().expect("couldn't parse an IPv6 address");
```

(More options will be added eventually.)

## Maintenance Status

This codebase is still not feature-complete.
Check out the issue tracker if you want to contribute, and don't hesistate to ask for something in an Issue.
That said, the business logic is tested and should work.
We will release version 1.0 when the GitHub milestone has been fully resolved.

## Testing

This project is tested both on concrete RFC5737/RFC3849 IPv4 and IPv6 documentation prefixes of:

- `192.0.2.0/24 (TEST-NET-1)`
- `198.51.100.0/24 (TEST-NET-2)`
- `203.0.113.0/24 (TEST-NET-3)`
- `2001:DB8::/32`

as well as randomly-generated IP addresses in the test suite to demonstrate full correctness.

## License

> Copyright &copy; 2019 Kristofer J. Rye

This software is released under either of:

- The Apache License, Version 2.0, (http://www.apache.org/licenses/LICENSE-2.0) or
- The MIT License, (http://opensource.org/licenses/MIT)

at your discretion.
Please see the license file ([LICENSE.md](LICENSE.md)) for more information.


## Acknowledgements

We would like to thank the developers of the `netaddr` Ruby gem for inspiring the development and ergonomics of this project.

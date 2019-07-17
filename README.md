# rust-netaddr (`netaddr2`) &mdot; [![Build Status](https://travis-ci.org/rye/rust-netaddr2.svg?branch=master)](https://travis-ci.org/rye/rust-netaddr2)

This crate is meant as a replacement for an existing reimplementation of various "netaddr" libraries that other languages have.
However, this crate aims to be as _simple_ and _straightforward_ as possible.
We accomplish the desired results by only introducing one new data structure on top of the existing `std::net::IpAddr` frameworks, which were added to the Rust language in version 1.7.0.
We have no dependencies by default and will only accept additional dependencies on an opt-in basis.

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

Until such a time as this project is first published on the crates.io repository, the following copyright notice applies, and this work is not licensed.

>Copyright (c) 2019 Kristofer J. Rye
All rights reserved.

It is likely that, upon release, this software will be licensed under the `GPLv3` license with private licensing options available upon request.

## Acknowledgements

We would like to thank Dustin Spinhirne, the developer of the `netaddr` Ruby gem for inspiring the development of this project.

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- Types of changes: Added, Changed, Deprecated, Removed, Fixed, Security -->

## [Unreleased]
### Added
- A `CHANGELOG.md` file.

## [0.1.0] - 2019-07-23
### Added
- `NetAddr` enum with two variants (`V4`, `V6`), each containing the respective `Ipv\dAddr` class.
- `#[derive]`-ed impls for `Copy`, `Clone`, `Debug`, `PartialEq`, `Eq`, `Ord`, and `Hash` on `NetAddr`.
- `NetAddr#netmask` method to return the netmask address in `IpAddr` form.
- `NetAddr#network` method to return the network address in `IpAddr` form.
- `NetAddrError` enum to hold structured information about `NetAddr`-generated errors.
- `#[derive]`-ed impls for `Clone`, `Debug`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, and `Hash` on `NetAddrError`.
- An impl for `From<std::net::AddrParseError` on `NetAddrError`.
- A public `mask<T, U>` function for performing bitwise arithmetic on arbitrary types.
- A `F32: u32` constant on `NetAddr`.
- A `F128: u128` constant on `NetAddr`.
- A `F32V4: Ipv4Addr` constant on `NetAddr`.
- A `F32V6: Ipv6Addr` constant on `NetAddr`.
- `NetAddr#contains<T>` generic method for checking containment of anything that can be converted to a `NetAddr`.
- `NetAddr#broadcast` method for getting the broadcast address of IPv4 networks only.
- `NetAddr#merge` method for combining networks together, if possible.
- An impl for `From<std::net::IpAddr>` on `NetAddr` which uses netmasks of all ones.
- An impl for `From<std::net::Ipv4Addr>` on `NetAddr`.
- An impl for `From<std::net::Ipv6Addr>` on `NetAddr`.
- An impl for `FromStr` on `NetAddr` for parsing.
- An impl for `PartialOrd` on `NetAddr` for comparing two networks first by network address and then by netmask.
- Tests for all of these Added features.

[Unreleased]: https://github.com/rye/rust-netaddr2/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/rye/rust-netaddr2/releases/tag/v0.1.0

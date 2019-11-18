# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- Types of changes: Added, Changed, Deprecated, Removed, Fixed, Security -->

## Unreleased

## [0.6.1] - 2019-11-18
### Added
- Added the `serde` feature to the `docs.rs` metadata key

## [0.6.0] - 2019-11-18
### Added
- Implementations for `serde::Serialize` and `serde::Deserialize` on core types, gated behind the `serde` feature

## [0.5.0] - 2019-10-20
### Added
- Implementation for `core::fmt::Display` on the `NetAddr` enum and the `Netv4Addr` and `Netv6Addr` structs.

## [0.4.1] - 2019-09-29
### Added
- Tests for the `Merge` trait.
- Tests for the methods on the `Netv4Addr` and `Netv6Addr` structs.
- Documentation for some items.

## [0.4.0] - 2019-08-25
### Changed
- Made the `mask` and `addr` methods `const` and `pub` under the `Netv4Addr` and `Netv6Addr` types.

## [0.3.0] - 2019-08-22
### Added
- A lot of tests to the source code in a structured way.

### Changed
- **Breaking**: Restructured the API to use lots of modules.
- **Breaking**: Use the recently-stabilized `Self` type alias in the code. (requires Rust 1.37)
- Began using GitHub Actions for CI instead of CircleCI.

### Removed
- The `script/clippy` script.

## [0.2.0] - 2019-08-12
### Added
- A Travis CI configuration for testing.

### Changed
- Existing API to use traits.

## [0.1.2] - 2019-07-23
### Fixed
- SPDX identifier for `Apache-2.0` in the manifest, allowing releases to be published.

## [0.1.1] - 2019-07-23
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

[0.4.0]: https://github.com/rye/rust-netaddr2/releases/tag/v0.4.0
[0.3.0]: https://github.com/rye/rust-netaddr2/releases/tag/v0.3.0
[0.2.0]: https://github.com/rye/rust-netaddr2/releases/tag/v0.2.0
[0.1.2]: https://github.com/rye/rust-netaddr2/releases/tag/v0.1.2
[0.1.1]: https://github.com/rye/rust-netaddr2/releases/tag/v0.1.1
[0.1.0]: https://github.com/rye/rust-netaddr2/releases/tag/v0.1.0

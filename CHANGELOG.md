# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- Types of changes: Added, Changed, Deprecated, Removed, Fixed, Security -->

## Unreleased

### Added

- `Netv4Addr`: Added a `len()` method for computing the number of addresses contained in a network.
- New `Netv4Addr#is_empty() -> bool` method for determining whether or not the network is empty.
- New `AddressIterator` iterator type for iterating over a network's addresses.

## [0.10.0] - 2021-07-06

### Changed

- **Breaking**: `Netv4Addr::addr`, `Netv4Addr::mask`, `Netv6Addr::addr`, and `Netv6Addr::mask` all now return `Ipv4Addr` or `Ipv6Addr` respectively instead of `&Ipv4Addr` or `&Ipv6Addr`.

- **Breaking**: Replaced the `derive`'d `Ord` impl with our own explicit implementation.
  If you were using our old ordering, bare in mind that the behavior has changed.

  Previously, we just used the derived `Ord` comparison on the underlying `Ip<...>Addr` structs in field-wise ordering.
  Now, a `Net<...>Addr` struct is considered greater than another if its `addr` is equal but its mask is greater, or otherwise if its `addr` is greater.
  For example, `1.0.0.0/8` < `2.0.0.0/8`, `1.0.0.0/7` < `1.0.0.0/8`, etc.

- Internal fixes for the tests
- Adjusted CI configuration

## [0.9.0] - 2020-02-26

### Changed

- **Breaking**: Adjusted the signature of the `Contains` trait to take a type parameter.

  Most users should not be affected in any way by this change, as the `.contains()` method still has the same syntax.
  However, the types for which containment can be checked are now explicitly enumerated.

## [0.8.0] - 2020-02-26

### Added

- `is_cidr` method on `NetAddr`, `Netv4Addr`, and `Netv6Addr` to return whether the represented netaddr is CIDR or not.

### Changed

- Updated the `serde::de` behavior to be slightly more informative and standard.

## [0.7.1] - 2019-01-12

### Added

- Documented a few undocumented types.

## [0.7.0] - 2019-12-24

### Added

- A new `Result<T, Error>` type, which is a sugar around the nascent `Error` type.

### Changed

- Renamed the `NetAddrError` type to just `Error`.

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

[0.10.0]: https://github.com/rye/rust-netaddr2/releases/tag/v0.10.0
[0.9.0]: https://github.com/rye/rust-netaddr2/releases/tag/v0.9.0
[0.8.0]: https://github.com/rye/rust-netaddr2/releases/tag/v0.8.0
[0.7.1]: https://github.com/rye/rust-netaddr2/releases/tag/v0.7.1
[0.7.0]: https://github.com/rye/rust-netaddr2/releases/tag/v0.7.0
[0.6.1]: https://github.com/rye/rust-netaddr2/releases/tag/v0.6.1
[0.6.0]: https://github.com/rye/rust-netaddr2/releases/tag/v0.6.0
[0.5.0]: https://github.com/rye/rust-netaddr2/releases/tag/v0.5.0
[0.4.1]: https://github.com/rye/rust-netaddr2/releases/tag/v0.4.1
[0.4.0]: https://github.com/rye/rust-netaddr2/releases/tag/v0.4.0
[0.3.0]: https://github.com/rye/rust-netaddr2/releases/tag/v0.3.0
[0.2.0]: https://github.com/rye/rust-netaddr2/releases/tag/v0.2.0
[0.1.2]: https://github.com/rye/rust-netaddr2/releases/tag/v0.1.2
[0.1.1]: https://github.com/rye/rust-netaddr2/releases/tag/v0.1.1
[0.1.0]: https://github.com/rye/rust-netaddr2/releases/tag/v0.1.0

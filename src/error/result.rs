use super::Error;

/// A type definition with `netaddr2::Error` as its type.
pub type Result<T> = core::result::Result<T, Error>;

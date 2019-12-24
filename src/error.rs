/// The error type for operations relating to the `NetAddr` type.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
	ParseError(String),
}

mod display;
mod from;
mod result;

pub use result::*;

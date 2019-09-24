/// The error type for operations relating to the `NetAddr` type.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NetAddrError {
	ParseError(String),
}

mod display;
mod from;

/// Masking one object with another
pub trait Mask {
	/// The result of masking
	type Output;

	/// Perform the mask
	fn mask(&self, other: &Self) -> Self::Output;
}

mod ipaddr;
mod ipv4addr;
mod ipv6addr;

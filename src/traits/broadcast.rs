/// Compute the "broadcast" address for supported networks.
pub trait Broadcast {
	/// The type of the broadcast address
	type Output;

	/// Compute the broadcast address
	fn broadcast(&self) -> Self::Output;
}

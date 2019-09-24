/// A trait encapsulating the operation of computing the broadcast
/// address for applicable networks.
pub trait Broadcast {
	type Output;

	/// Compute the broadcast address.
	fn broadcast(&self) -> Self::Output;
}

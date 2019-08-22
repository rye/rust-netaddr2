pub trait Mask {
	type Output;

	fn mask(&self, other: &Self) -> Self::Output;
}

mod ipaddr;
mod ipv4addr;
mod ipv6addr;

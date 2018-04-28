
mod base_graph;

pub use self::{
	base_graph::*,
};

use graphene::core::trait_aliases::Id;
use quickcheck::{Arbitrary, Gen};
use mock_graphs::{
	MockId
};

///
/// Trait alias for arbitrary identifiers.
///
pub trait ArbId: Arbitrary + Id{}
impl<T> ArbId for T where T: Arbitrary + Id{}

impl Arbitrary for MockId
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self
	{
		Self{value: u32::arbitrary(g)}
	}
	
	fn shrink(&self) -> Box<Iterator<Item = Self>>
	{
		Box::new(self.value.shrink().map(|v| Self{value: v}))
	}
}
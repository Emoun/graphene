
mod mock_graph;

pub use self::{
	mock_graph::*,
};

use graphene::core::trait_aliases::Id;
use quickcheck::{Arbitrary, Gen};
use mock_graphs::{
	MockVertex, MockT
};

///
/// Trait alias for arbitrary identifiers.
///
pub trait ArbVertex: Arbitrary + Id{}
impl<T> ArbVertex for T where T: Arbitrary + Id{}

impl Arbitrary for MockVertex
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

pub trait ArbT: Arbitrary{}
impl<T> ArbT for T where T: Arbitrary + Id{}

impl Arbitrary for MockT
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
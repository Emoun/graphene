//! Mock graph implementations to be used by tests.
//!
//!
//!

#[macro_use]
pub mod utilities;
pub mod arbitrary;
mod mock_graph;

pub use self::mock_graph::*;
use graphene::core::Directedness;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct MockVertex
{
	pub value: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MockT
{
	pub value: u32,
}

impl Default for MockT
{
	fn default() -> Self
	{
		MockT {
			value: u32::default(),
		}
	}
}

pub type MockEdgeWeight = MockT;
pub type MockVertexWeight = MockT;

/// A mock of Directedness for when we want to insure that some implementation
/// is directedness-agnostic.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct MockDirectedness(pub bool);
impl Directedness for MockDirectedness
{
	fn directed() -> bool
	{
		panic!("Mock directedness should not be queried.");
	}
}

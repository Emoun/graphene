//! Mock graph implementations to be used by tests.

#[macro_use]
pub mod utilities;
pub mod arbitrary;
mod mock_graph;

pub use self::mock_graph::*;
use graphene::core::{Directedness, Graph};
use quickcheck::Arbitrary;
use std::fmt::Debug;

/// A trait alias for all types used in testing graphs.
pub trait MockType: Debug + Clone + PartialEq + Send + Arbitrary {}
impl MockType for () {}

/// A mock type for graph vertices.
///
/// Does not use `MockT` because the requirements on graph vertices are stricter
/// than the rest of the associated types. (e.g., requires `Eq`)
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct MockVertex
{
	pub value: usize,
}
impl MockType for MockVertex {}

/// A mock type for various uses
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
impl MockType for MockT {}

pub type MockEdgeWeight = MockT;
pub type MockVertexWeight = MockT;

/// A mock of Directedness for when we want to ensure that some implementation
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

pub trait TestGraph: Clone + Graph<Vertex = MockVertex, VertexWeight = MockVertexWeight>
where
	Self::EdgeWeight: MockType,
{
}
impl<T> TestGraph for T
where
	T: Clone + Graph<Vertex = MockVertex, VertexWeight = MockVertexWeight>,
	<T as Graph>::EdgeWeight: MockType,
{
}

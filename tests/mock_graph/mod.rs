//!
//! Mock graph implementations to be used by tests.
//!
//!
//!

#[macro_use]
pub mod utilities;
pub mod arbitraries;
mod mock_graph;

pub use self::{
	mock_graph::*,
};
use graphene::core::Directedness;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct MockVertex
{
	pub value: u32
}

#[derive(Debug, Clone, PartialEq)]
pub struct MockT
{
	pub value: u32
}

impl Default for MockT
{
	fn default() -> Self {
		MockT{value: u32::default()}
	}
}

pub type MockEdgeWeight = MockT;
pub type MockVertexWeight = MockT;

#[derive(Clone, Debug)]
pub struct MockDirectedness(pub bool);
impl Directedness for MockDirectedness {
	fn directed() -> bool {
		panic!("Mock directedness should not be queried.");
	}
}
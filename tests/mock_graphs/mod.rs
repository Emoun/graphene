//!
//! Mock graph implementations to be used by tests.
//!
//!
//!

#[macro_use]
pub mod utilities;
mod arbitraries;
mod mock_graph;


pub use self::{
	mock_graph::*,
	arbitraries::*
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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
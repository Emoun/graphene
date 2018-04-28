//!
//! Mock graph implementations to be used by tests.
//!
//!
//!

#[macro_use]
pub mod utilities;
mod arbitraries;
mod mock_base_graph;


pub use self::{
	mock_base_graph::*,
	arbitraries::*
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct MockId
{
	pub value: u32
}

///
/// Mock vertex value.
///
pub type MockVertex = MockId;

///
/// Mock edge Id.
///
pub type MockEdgeId = MockId;

//!
//! Tests `DFS`
//!

use crate::mock_graph::arbitrary::{ArbTwoVerticesIn, ArbConnectedGraph};
use graphene::core::{Directed, ImplGraph};
use graphene::algo::DFS;

///
/// Tests can always find a vertex in a connected, directed graph
///
#[quickcheck]
fn connected(ArbTwoVerticesIn(mock, v1, v2): ArbTwoVerticesIn<ArbConnectedGraph<Directed>>)
			 -> bool
{
	DFS::new(mock.graph(), v1).find(|&v| v == v2).is_some()
}

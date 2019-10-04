//!
//! Tests the `core::Connected` trait and its constrainer `core::ConnectedGraph`.
//!

use graphene::core::{Directed, Constrainer, Undirected};
use crate::mock_graph::arbitrary::ArbConnectedGraph;
use graphene::core::constraint::ConnectedGraph;

///
/// Tests that Connected Graph correctly identifies connected graphs.
///
#[quickcheck]
fn accept_unique(g: ArbConnectedGraph<Undirected>) -> bool
{
	ConnectedGraph::constrain_single(g.0.unconstrain()).is_ok()
}

///
/// Tests that Connected Graph correctly identifies connected graphs.
///
#[quickcheck]
fn accept_unique_directed(g: ArbConnectedGraph<Directed>) -> bool
{
	ConnectedGraph::constrain_single(g.0.unconstrain()).is_ok()
}
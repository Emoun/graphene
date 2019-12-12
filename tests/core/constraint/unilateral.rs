//!
//! Tests the `core::Unilateral` trait and its constrainer `core::UnilateralGraph`.
//!

use graphene::core::{
	Constrainer,
	constraint::UnilaterallyConnectedGraph
};
use crate::mock_graph::arbitrary::{ArbUnilatralGraph, ArbNonUnilatralGraph};

///
/// Tests that UnilateralGraph correctly identifies unilateral graphs.
///
#[quickcheck]
fn accept_connected(g: ArbUnilatralGraph) -> bool
{
	UnilaterallyConnectedGraph::constrain_single(g.0.unconstrain()).is_ok()
}

///
/// Tests that UnilateralGraph correctly rejects non-unilateral graphs.
///
#[quickcheck]
fn reject_unconnected(g: ArbNonUnilatralGraph) -> bool
{
	UnilaterallyConnectedGraph::constrain_single(g.0).is_err()
}
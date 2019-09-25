//!
//! Tests the `core::Unique` trait and its constrainer `core::UniqueGraph`.
//!

use crate::mock_graph::arbitrary::{ArbUniqueGraph, ArbNonUniqueGraph};
use graphene::core::{Directed, Constrainer, Undirected, Directedness, Graph, Edge};
use graphene::core::constraint::UniqueGraph;
use crate::mock_graph::{MockGraph, MockEdgeWeight};
use quickcheck::{StdThreadGen, Arbitrary};
use rand::Rng;

///
/// Tests that UniqueGraph correctly identifies unique directed graphs.
///
#[quickcheck]
fn constrain_directed(g: ArbUniqueGraph<Directed>) -> bool
{
	UniqueGraph::constrain_single(g.0.unconstrain()).is_ok()
}

///
/// Tests that UniqueGraph correctly identifies unique undirected graphs.
///
#[quickcheck]
fn constrain_undirected(g: ArbUniqueGraph<Undirected>) -> bool
{
	UniqueGraph::constrain_single(g.0.unconstrain()).is_ok()
}

#[quickcheck]
fn reject_non_unique_directed(g: ArbNonUniqueGraph<Directed>) -> bool
{
	UniqueGraph::constrain_single(g.0).is_err()
}

#[quickcheck]
fn reject_non_unique_undirected(g: ArbNonUniqueGraph<Undirected>) -> bool
{
	UniqueGraph::constrain_single(g.0).is_err()
}


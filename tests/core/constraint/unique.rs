//!
//! Tests the `core::Unique` trait and its constrainer `core::UniqueGraph`.
//!

use crate::mock_graph::arbitrary::{ArbUniqueGraph, ArbNonUniqueGraph};
use graphene::core::{Directed, Constrainer, Undirected};
use graphene::core::constraint::UniqueGraph;

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

///
/// Tests that UniqueGraph correctly rejects non-unique directed graphs.
///
#[quickcheck]
fn reject_non_unique_directed(g: ArbNonUniqueGraph<Directed>) -> bool
{
	UniqueGraph::constrain_single(g.0).is_err()
}

///
/// Tests that UniqueGraph correctly rejects non-unique undirected graphs.
///
#[quickcheck]
fn reject_non_unique_undirected(g: ArbNonUniqueGraph<Undirected>) -> bool
{
	UniqueGraph::constrain_single(g.0).is_err()
}


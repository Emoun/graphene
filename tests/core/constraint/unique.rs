//!
//! Tests the `core::Unique` trait and its constrainer `core::UniqueGraph`.
//!

use crate::mock_graph::arbitrary::ArbUniqueGraph;
use graphene::core::{Directed, Constrainer};
use graphene::core::constraint::UniqueGraph;

#[quickcheck]
fn constrain_test(g: ArbUniqueGraph<Directed>) -> bool
{
	UniqueGraph::constrain_single(g.0.unconstrain()).is_ok()
}
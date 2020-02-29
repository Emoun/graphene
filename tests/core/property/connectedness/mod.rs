use crate::mock_graph::{MockDirectedness, MockGraph};
use graphene::core::{
	property::{Connected, ConnectedGraph, Unilateral, UnilateralGraph, Weak},
	Directed,
};
use static_assertions::assert_impl_all;

mod connected;
mod unilateral;
mod weak;

// Test that all Connected graphs are also unilateral and weak.
assert_impl_all!(ConnectedGraph<MockGraph<MockDirectedness>>: Connected, Unilateral, Weak);

// Test that all Unilateral graphs are also weak.
assert_impl_all!(UnilateralGraph<MockGraph<Directed>>: Unilateral, Weak);

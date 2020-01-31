use graphene::core::{
	Directed,
	constraint::{
		ConnectedGraph, UnilateralGraph, Connected, Unilateral, Weak
	}
};
use crate::mock_graph::{MockGraph, MockDirectedness};
use static_assertions::assert_impl_all;

mod connected;
mod unilateral;
mod weak;

// Test that all Connected graphs are also unilateral and weak.
assert_impl_all!(ConnectedGraph<MockGraph<MockDirectedness>>: Connected, Unilateral, Weak);

// Test that all Unilateral graphs are also weak.
assert_impl_all!(UnilateralGraph<MockGraph<Directed>>: Unilateral, Weak);

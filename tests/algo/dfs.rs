//! Tests `Dfs`
//!

use crate::mock_graph::{
	arbitrary::{ArbConnectedGraph, ArbVertexIn},
	MockVertex,
};
use duplicate::duplicate;
use graphene::{
	algo::Dfs,
	core::{Directed, Undirected},
};
use std::cell::Cell;

#[duplicate(
	module			directedness;
	[ directed ]	[ Directed ];
	[ undirected ]	[ Undirected ]
)]
mod module
{
	use super::*;

	/// Tests that the 'on_exit' closure is called in stack order compared to
	/// the produced vertices.
	#[quickcheck]
	fn on_exit_stack_call_order(mock: ArbVertexIn<ArbConnectedGraph<directedness>>) -> bool
	{
		let stack: Cell<Vec<MockVertex>> = Cell::new(Vec::new());
		let mut success = true;

		fn on_exit<G>(
			_: &G,
			v: MockVertex,
			(stack, success): &mut (&Cell<Vec<MockVertex>>, &mut bool),
		)
		{
			// On exit, check that the same vertex is on top of the stack
			let mut s = stack.take();
			if let Some(&v2) = s.last()
			{
				if v == v2
				{
					s.pop();
				}
				else
				{
					**success = false;
				}
			}
			else
			{
				**success = false;
			}
			stack.replace(s);
		}

		Dfs::new(&mock, on_exit, (&stack, &mut success)).for_each(|v| {
			// When a vertex is produced by the Dfs, put it on the stack.
			let mut s = stack.take();
			s.push(v);
			stack.replace(s);
		});
		success
	}
}

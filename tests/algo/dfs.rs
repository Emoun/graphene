//! Tests `Dfs`

use crate::mock_graph::{arbitrary::Arb, MockGraph, MockVertex};
use duplicate::duplicate_item;
use graphene::{
	algo::Dfs,
	core::{
		property::{ConnectedGraph, HasVertex, VertexInGraph},
		Directed, Undirected,
	},
};
use std::cell::Cell;

#[duplicate_item(
	directedness; [ Directed ]; [ Undirected ]
)]
mod __
{
	use super::*;

	/// Tests that the 'on_exit' closure is called in stack order compared to
	/// the produced vertices.
	#[quickcheck]
	fn on_exit_stack_call_order(
		Arb(mock): Arb<VertexInGraph<ConnectedGraph<MockGraph<directedness>>>>,
	) -> bool
	{
		// Ensure the starting vertex is on the stack, so that it is the last
		// to run 'on_exit'
		let stack: Cell<Vec<MockVertex>> = Cell::new(vec![mock.get_vertex()]);
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

		Dfs::new(
			&mock,
			Dfs::do_nothing_on_visit,
			on_exit,
			Dfs::do_nothing_on_explore,
			(&stack, &mut success),
		)
		.for_each(|v| {
			// When a vertex is produced by the Dfs, put it on the stack.
			let mut s = stack.take();
			s.push(v);
			stack.replace(s);
		});
		success
	}
}

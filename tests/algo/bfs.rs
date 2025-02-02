use crate::mock_graph::{arbitrary::Arb, MockGraph};
use duplicate::duplicate_item;
use graphene::{
	algo::Bfs,
	core::{
		property::{ConnectedGraph, VertexIn, VertexInGraph},
		Directed, Undirected,
	},
};
use std::collections::HashSet;

#[duplicate_item(
	directedness; [ Directed ]; [ Undirected ]
)]
mod __
{
	use super::*;

	/// Tests that each produced vertex has an equal or higher depth than the
	/// previous one.
	#[quickcheck]
	fn increasing_depth(
		Arb(graph): Arb<VertexInGraph<ConnectedGraph<MockGraph<directedness>>>>,
	) -> bool
	{
		let mut depth = 0;
		let mut bfs = Bfs::new(&graph);

		while let Some(v) = bfs.next()
		{
			if bfs.depth(v) < depth
			{
				return false;
			}
			depth = bfs.depth(v);
		}
		true
	}

	/// Tests that a vertex's predecessor in the search has already been seen.
	#[quickcheck]
	fn predecessor_already_seen(
		Arb(graph): Arb<VertexInGraph<ConnectedGraph<MockGraph<directedness>>>>,
	) -> bool
	{
		let mut seen = HashSet::new();
		seen.insert(graph.vertex_at::<0>().value);

		let mut bfs = Bfs::new(&graph);
		while let Some(v) = bfs.next()
		{
			if let Some(p) = bfs.predecessor(v)
			{
				if !seen.contains(&p.value)
				{
					return false;
				}
			}
			seen.insert(v.value);
		}
		true
	}

	/// Tests that each produced vertex's depth is 1 higher that its
	/// predecessor.
	#[quickcheck]
	fn predecessor_shallower(
		Arb(graph): Arb<VertexInGraph<ConnectedGraph<MockGraph<directedness>>>>,
	) -> bool
	{
		let mut bfs = Bfs::new(&graph);

		while let Some(v) = bfs.next()
		{
			if let Some(p) = bfs.predecessor(v)
			{
				if !(bfs.depth(v) == (bfs.depth(p) + 1))
				{
					return false;
				}
			}
		}
		true
	}

	/// Tests that any vertex with a depth > 0 has a predecessor.
	#[quickcheck]
	fn has_predecessor(
		Arb(graph): Arb<VertexInGraph<ConnectedGraph<MockGraph<directedness>>>>,
	) -> bool
	{
		let mut bfs = Bfs::new(&graph);

		while let Some(v) = bfs.next()
		{
			if bfs.depth(v) > 0
			{
				if bfs.predecessor(v).is_none()
				{
					return false;
				}
			}
		}
		true
	}

	/// Tests that following the predecessors will reach the root.
	#[quickcheck]
	fn predecessor_path_reaches_root(
		Arb(graph): Arb<VertexInGraph<ConnectedGraph<MockGraph<directedness>>>>,
	) -> bool
	{
		let root = graph.vertex_at::<0>();
		let mut bfs = Bfs::new(&graph);

		while let Some(v) = bfs.next()
		{
			let mut current = v;
			while let Some(p) = bfs.predecessor(current)
			{
				current = p;
			}
			if current != root
			{
				return false;
			}
		}
		true
	}
}

/// Tests that following the predecessors will reach the root.
#[quickcheck]
fn predecessor_path_reaches_root(
	Arb(graph): Arb<VertexInGraph<ConnectedGraph<MockGraph<Directed>>>>,
) -> bool
{
	let root = graph.vertex_at::<0>();
	let mut bfs = Bfs::new(&graph);

	while let Some(v) = bfs.next()
	{
		let mut current = v;
		while let Some(p) = bfs.predecessor(current)
		{
			current = p;
		}
		if current != root
		{
			return false;
		}
	}
	true
}

use crate::mock_graph::{arbitrary::Arb, MockGraph};
use duplicate::duplicate_item;
use graphene::{
	algo::Bfs,
	core::{
		property::{ConnectedGraph, Rooted, VertexCount, VertexIn, VertexInGraph},
		Directed, Graph, Undirected,
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

	/// Tests that the predecessor tree is correct
	#[quickcheck]
	fn predecessor_tree(
		Arb(graph): Arb<VertexInGraph<ConnectedGraph<MockGraph<directedness>>>>,
		pre_iter: usize,
	) -> bool
	{
		let root = graph.vertex_at::<0>();
		let mut bfs = Bfs::new(&graph);

		// Iterate some amount before test start
		(0..(pre_iter % graph.vertex_count())).for_each(|_| {
			bfs.next();
		});

		let tree = bfs.predecessor_tree();

		// Tests the search root has is present without predecessor

		if tree.root() == root
		{
			if tree.edges_sourced_in(tree.root()).count() > 0
			{
				// Search root has predecessor
				return false;
			}
		}
		else
		{
			// Search root not in tree
			return false;
		}

		// Tests all expected predecessor edges are present
		for (v, pred) in graph
			.all_vertices()
			.filter_map(|v| bfs.predecessor(v).map(|p| (v, p)))
		{
			// Tests all predecessor edges are present and alone
			if vec![pred]
				!= tree
					.edges_sourced_in(v)
					.map(|(si, _)| si)
					.collect::<Vec<_>>()
			{
				return false;
			}
		}

		// Tests all present edges are expected
		for (so, si, _) in tree.all_edges()
		{
			if bfs.predecessor(so).unwrap() != si
			{
				return false;
			}
		}

		true
	}
}

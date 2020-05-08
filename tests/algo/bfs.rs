use crate::mock_graph::arbitrary::{ArbConnectedGraph, ArbVertexIn};
use duplicate::duplicate;
use graphene::{
	algo::Bfs,
	core::{property::HasVertex, Directed, Undirected},
};
use std::collections::HashSet;

#[duplicate(
	module			directedness;
	[ directed ]	[ Directed ];
	[ undirected ]	[ Undirected ]
)]
mod module
{
	use super::*;

	/// Tests that each produced vertex has an equal or higher depth than the
	/// previous one.
	#[quickcheck]
	fn increasing_depth(graph: ArbVertexIn<ArbConnectedGraph<directedness>>) -> bool
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
	fn predecessor_already_seen(graph: ArbVertexIn<ArbConnectedGraph<directedness>>) -> bool
	{
		let mut seen = HashSet::new();
		seen.insert(graph.get_vertex().value);

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
	fn predecessor_shallower(graph: ArbVertexIn<ArbConnectedGraph<directedness>>) -> bool
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
	fn has_predecessor(graph: ArbVertexIn<ArbConnectedGraph<directedness>>) -> bool
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
	fn predecessor_path_reaches_root(graph: ArbVertexIn<ArbConnectedGraph<directedness>>) -> bool
	{
		let root = graph.get_vertex();
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
fn predecessor_path_reaches_root(graph: ArbVertexIn<ArbConnectedGraph<Directed>>) -> bool
{
	let root = graph.get_vertex();
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

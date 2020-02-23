use crate::mock_graph::arbitrary::{ArbConnectedGraph, ArbVertexIn};
use graphene::{
	algo::Bfs,
	core::{Directed, ImplGraph},
};
use std::collections::HashSet;

duplicate_for_directedness! {
	$directedness

	///
	/// Tests that each produced vertex has an equal or higher depth than the previous
	/// one.
	///
	#[quickcheck]
	fn increasing_depth(
		ArbVertexIn(mock, v): ArbVertexIn<ArbConnectedGraph<directedness>>
	) -> bool
	{
		let mut depth = 0;
		let mut bfs = Bfs::new(mock.graph(), v);

		while let Some(v) = bfs.next() {
			if bfs.depth(v) < depth {
				return false;
			}
			depth = bfs.depth(v);
		}
		true
	}

	///
	/// Tests that a vertex's predecessor in the search has already been seen.
	///
	#[quickcheck]
	fn predecessor_already_seen(
		ArbVertexIn(mock, v): ArbVertexIn<ArbConnectedGraph<directedness>>
	) -> bool
	{
		let mut seen = HashSet::new();
		let mut bfs = Bfs::new(mock.graph(), v);

		while let Some(v) = bfs.next() {
			if let Some(p) = bfs.predecessor(v) {
				if !seen.contains(&p.value) {
					return false
				}
			}
			seen.insert(v.value);
		}
		true
	}

	///
	/// Tests that each produced vertex's depth is 1 higher that its predecessor.
	///
	#[quickcheck]
	fn predecessor_shallower(
		ArbVertexIn(mock, v): ArbVertexIn<ArbConnectedGraph<directedness>>
	) -> bool
	{
		let mut bfs = Bfs::new(mock.graph(), v);

		while let Some(v) = bfs.next() {
			if let Some(p) = bfs.predecessor(v) {
				if !(bfs.depth(v) == (bfs.depth(p)+1)) {
					return false;
				}
			}
		}
		true
	}

	///
	/// Tests that any vertex with a depth > 0 has a predecessor.
	///
	#[quickcheck]
	fn has_predecessor(
		ArbVertexIn(mock, v): ArbVertexIn<ArbConnectedGraph<directedness>>
	) -> bool
	{
		let mut bfs = Bfs::new(mock.graph(), v);

		while let Some(v) = bfs.next() {
			if bfs.depth(v) > 0{
				if bfs.predecessor(v).is_none() {
					return false;
				}
			}
		}
		true
	}

	///
	/// Tests that following the predecessors will reach the root.
	///
	#[quickcheck]
	fn predecessor_path_reaches_root(
		ArbVertexIn(mock, root): ArbVertexIn<ArbConnectedGraph<directedness>>
	) -> bool
	{
		let mut bfs = Bfs::new(mock.graph(), root);

		while let Some(v) = bfs.next() {
			let mut current = v;
			while let Some(p) = bfs.predecessor(current) {
				current = p;
			}
			if current != root {
				return false
			}
		}
		true
	}
}

/// Tests that following the predecessors will reach the root.
#[quickcheck]
fn predecessor_path_reaches_root(
	ArbVertexIn(mock, root): ArbVertexIn<ArbConnectedGraph<Directed>>,
) -> bool
{
	let mut bfs = Bfs::new(mock.graph(), root);

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

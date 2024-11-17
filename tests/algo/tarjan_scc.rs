use crate::mock_graph::{arbitrary::Arb, MockGraph};
use graphene::core::property::VertexInGraph;
/// Tests `TarjanScc`: Tarjan's algorithm for finding strongly connected
/// components.
use graphene::{
	algo::TarjanScc,
	core::{
		property::{ConnectedGraph, Subgraph},
		Directed, Graph, Guard,
	},
};

/// Tests that no produced SCC is empty
#[quickcheck]
fn produces_non_empty_components(Arb(graph): Arb<VertexInGraph<MockGraph<Directed>>>) -> bool
{
	for scc in TarjanScc::new(&graph)
	{
		if scc.all_vertices().count() == 0
		{
			return false;
		}
	}
	true
}

/// Tests that any SCC returned is actually strongly connected.
#[quickcheck]
fn produces_connected_components(Arb(graph): Arb<VertexInGraph<MockGraph<Directed>>>) -> bool
{
	for scc in TarjanScc::new(&graph)
	{
		if !ConnectedGraph::can_guard(&scc)
		{
			return false;
		}
	}
	true
}

/// Tests that for any SCC pair produced, they are not strongly connected.
#[quickcheck]
fn produces_disconnected_components(Arb(graph): Arb<VertexInGraph<MockGraph<Directed>>>) -> bool
{
	let sccs = TarjanScc::new(&graph).collect::<Vec<_>>();
	let mut scc_iter = sccs.iter();

	while let Some(scc) = scc_iter.next()
	{
		for scc2 in scc_iter.clone()
		{
			if scc.reaches(scc2).is_some()
			{
				if scc2.reaches(scc).is_some()
				{
					return false;
				}
			}
		}
	}
	true
}

/// Tests that all vertices are put inside some produced SCC.
#[quickcheck]
fn produces_all_vertices(Arb(graph): Arb<VertexInGraph<MockGraph<Directed>>>) -> bool
{
	// We simply count the vertices since we have another test
	// for checking that no vertex is reused
	let mut vertex_count = 0;
	for scc in TarjanScc::new(&graph)
	{
		vertex_count += scc.all_vertices().count();
	}
	vertex_count == graph.all_vertices().count()
}

/// Tests that all vertices in the components are from the original graph.
#[quickcheck]
fn produces_only_valid_vertices(Arb(graph): Arb<VertexInGraph<MockGraph<Directed>>>) -> bool
{
	for scc in TarjanScc::new(&graph)
	{
		for v in scc.all_vertices()
		{
			if !graph.contains_vertex(v)
			{
				return false;
			}
		}
	}
	true
}

/// Tests that no two produced SCCs share any vertices
#[quickcheck]
fn produces_vertex_disjoint_components(Arb(graph): Arb<VertexInGraph<MockGraph<Directed>>>)
	-> bool
{
	let sccs = TarjanScc::new(&graph).collect::<Vec<_>>();
	let mut scc_iter = sccs.iter();

	while let Some(scc) = scc_iter.next()
	{
		for scc2 in scc_iter.clone()
		{
			for v in scc.all_vertices()
			{
				if scc2.contains_vertex(v)
				{
					return false;
				}
			}
		}
	}
	true
}

/// Tests that the SCCs are produced in some reverse topological order.
/// This is a guarantee of Tarjan's algorithm, which means if we don't do that,
/// we are not implementing it correctly.
#[quickcheck]
fn produces_reverse_topological_ordering(
	Arb(graph): Arb<VertexInGraph<MockGraph<Directed>>>,
) -> bool
{
	// To test the ordering, we simply check that an earlier-produced component
	// can't reach any later one.
	let sccs = TarjanScc::new(&graph).collect::<Vec<_>>();
	let mut scc_iter = sccs.iter();

	while let Some(scc) = scc_iter.next()
	{
		for scc2 in scc_iter.clone()
		{
			if scc.reaches(scc2).is_some()
			{
				return false;
			}
		}
	}
	true
}

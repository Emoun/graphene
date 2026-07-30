use crate::mock_graph::{arbitrary::Arb, MockGraph};
use graphene::{
	algo::{search::Topo, UnretainedIterator},
	core::{
		property::{AcyclicGraph, ConnectedGraph, VertexIn, VertexInGraph},
		Directed, Graph, Guard,
	},
};
use quickcheck::TestResult;
use std::collections::HashSet;

/// Tests that the nodes are output in topological sort order
#[quickcheck]
fn sort_order(Arb(graph): Arb<VertexInGraph<ConnectedGraph<MockGraph<Directed>>>>) -> TestResult
{
	let v = graph.vertex_at::<0>();

	if let Ok(g) = AcyclicGraph::guard(&graph)
	{
		let mut seen = HashSet::new();
		seen.insert(v);

		let mut top = Topo::new(&g);
		while let Some(t) = top.next(&g)
		{
			if g.edges_sinked_in(t).any(|(pred, _)| !seen.contains(&pred))
			{
				return TestResult::failed();
			}
		}

		TestResult::passed()
	}
	else
	{
		TestResult::discard()
	}
}

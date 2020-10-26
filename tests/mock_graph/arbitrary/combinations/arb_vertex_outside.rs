use crate::mock_graph::{
	arbitrary::{GuidedArbGraph, Limit},
	MockVertex, TestGraph,
};
use graphene::{
	core::{Ensure, Graph},
	impl_ensurer,
};
use quickcheck::{Arbitrary, Gen};
use std::collections::HashSet;

/// An arbitrary graph and a vertex that is guaranteed to not be in it.
#[derive(Clone, Debug)]
pub struct VertexOutside<G>(pub G, pub MockVertex)
where
	G: GuidedArbGraph,
	G::Graph: TestGraph;

impl<G> Ensure for VertexOutside<G>
where
	G: GuidedArbGraph,
	G::Graph: TestGraph,
{
	fn ensure_unvalidated(_c: Self::Ensured, _: ()) -> Self
	{
		unimplemented!()
	}

	fn validate(_c: &Self::Ensured, _: &()) -> bool
	{
		unimplemented!()
	}
}

impl_ensurer! {
	use<G> VertexOutside<G>: Ensure
	as (self.0): G
	where
	G: GuidedArbGraph,
	G::Graph: TestGraph,
}

impl<Gr> GuidedArbGraph for VertexOutside<Gr>
where
	Gr: GuidedArbGraph,
	Gr::Graph: TestGraph,
{
	fn arbitrary_fixed<G: Gen>(g: &mut G, v_count: usize, e_count: usize) -> Self
	{
		let graph = Gr::arbitrary_fixed(g, v_count, e_count);

		// Find a vertex that isn't in the graph
		let mut v = MockVertex::arbitrary(g);
		while graph.graph().contains_vertex(v)
		{
			v = MockVertex::arbitrary(g);
		}

		Self(graph, v)
	}

	fn shrink_guided(&self, limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		let mut result = Vec::new();
		// 	First shrink the graph, keeping only the shrunk ones where the vertex
		// stays invalid
		result.extend(
			self.0
				.shrink_guided(limits)
				.filter(|g| !g.graph().contains_vertex(self.1))
				.map(|g| Self(g, self.1)),
		);

		// We then shrink the vertex, keeping only the shrunk values
		// that are invalid in the graph
		result.extend(
			self.1
				.shrink()
				.filter(|&v| self.0.graph().contains_vertex(v))
				.map(|v| Self(self.0.clone(), v)),
		);

		Box::new(result.into_iter())
	}
}

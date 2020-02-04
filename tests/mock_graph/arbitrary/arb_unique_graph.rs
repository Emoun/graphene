use crate::mock_graph::{
	arbitrary::{GuidedArbGraph, Limit},
	MockEdgeWeight, MockGraph,
};
use graphene::core::{
	constraint::{DirectedGraph, UniqueGraph},
	AddEdge, Constrainer, Directedness, Edge, Graph, ImplGraph, ImplGraphMut, RemoveEdge,
};
use quickcheck::{Arbitrary, Gen};
use rand::Rng;
use std::{collections::HashSet, ops::RangeBounds};

/// An arbitrary graph that is unique
#[derive(Clone, Debug)]
pub struct ArbUniqueGraph<D: Directedness>(pub UniqueGraph<MockGraph<D>>);

impl<D: Directedness> ImplGraph for ArbUniqueGraph<D>
{
	type Graph = UniqueGraph<MockGraph<D>>;

	fn graph(&self) -> &Self::Graph
	{
		&self.0
	}
}
impl<D: Directedness> ImplGraphMut for ArbUniqueGraph<D>
{
	fn graph_mut(&mut self) -> &mut Self::Graph
	{
		&mut self.0
	}
}

impl<D: Directedness> GuidedArbGraph for ArbUniqueGraph<D>
{
	/// Generates a Unique graph with a vertex count within the given range.
	///
	/// The range for edges is only upheld if the lower bound is 1 and the lower
	/// bound of vertices is 1, in which case the graph is guaranteed to have at
	/// least 1 edge.
	fn arbitrary_guided<G: Gen>(
		g: &mut G,
		v_range: impl RangeBounds<usize>,
		e_range: impl RangeBounds<usize>,
	) -> Self
	{
		let (v_min, v_max, e_min, _) = Self::validate_ranges(g, v_range, e_range);
		let mut graph = MockGraph::arbitrary_guided(g, v_min..v_max, 0..=0);
		let verts: Vec<_> = graph.all_vertices().collect();
		let vertex_count = verts.len();

		// If the amount of vertices is 0, no edges can be created.
		if vertex_count > 0
		{
			// For each vertex pair (in each direction), maybe create an edge
			// The maximum number of edges is ridiculously big, so we allow approximately
			// the same the same number as vertices
			let edge_saturation = g.gen_range(
				0.0,
				(vertex_count) as f64 / (vertex_count * vertex_count) as f64,
			);
			let mut maybe_add_edge = |source, sink| {
				if g.gen_bool(edge_saturation)
				{
					graph
						.add_edge_weighted((source, sink, MockEdgeWeight::arbitrary(g)))
						.unwrap();
				}
			};
			if D::directed()
			{
				for &source in verts.iter()
				{
					for &sink in verts.iter()
					{
						maybe_add_edge(source, sink)
					}
				}
			}
			else
			{
				let mut iter = verts.iter();
				let mut iter_rest = iter.clone();
				while let Some(&source) = iter.next()
				{
					for &sink in iter_rest
					{
						maybe_add_edge(source, sink)
					}
					iter_rest = iter.clone()
				}
			}
			if e_min == 1 && graph.all_edges().count() < 1
			{
				graph
					.add_edge_weighted((
						verts[g.gen_range(0, verts.len())],
						verts[g.gen_range(0, verts.len())],
						MockEdgeWeight::arbitrary(g),
					))
					.unwrap()
			}
		}
		Self(UniqueGraph::unchecked(graph))
	}

	fn shrink_guided(&self, _limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		Box::new(
			self.0
				.clone()
				.unconstrain()
				.shrink()
				.map(|g| Self(UniqueGraph::unchecked(g))),
		)
	}
}

impl<D: Directedness> Arbitrary for ArbUniqueGraph<D>
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self
	{
		Self::arbitrary_guided(g, .., ..)
	}

	fn shrink(&self) -> Box<dyn Iterator<Item = Self>>
	{
		self.shrink_guided(HashSet::new())
	}
}

/// An arbitrary graph that is __not__ unique
#[derive(Clone, Debug)]
pub struct ArbNonUniqueGraph<D: Directedness>(pub MockGraph<D>, usize);

impl<D: Directedness> Arbitrary for ArbNonUniqueGraph<D>
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self
	{
		// Ensure there are at least 1 edge (so that we can duplicate)
		let mut graph = ArbUniqueGraph::arbitrary_guided(g, .., 1..).0.unconstrain();

		// Duplicate a arbitrary number of additional edges (at least 1)
		let original_edges: Vec<_> = graph.all_edges().map(|e| (e.source(), e.sink())).collect();
		let duplicate_count = g.gen_range(1, original_edges.len() + 1);
		for _ in 0..duplicate_count
		{
			let dup_edge = original_edges[g.gen_range(0, original_edges.len())];
			graph
				.add_edge_weighted((
					dup_edge.source(),
					dup_edge.sink(),
					MockEdgeWeight::arbitrary(g),
				))
				.unwrap();
		}
		Self(graph, duplicate_count)
	}

	fn shrink(&self) -> Box<dyn Iterator<Item = Self>>
	{
		let mut result = Vec::new();

		// Allow MockGraph to shrink everything except removing edges.
		let mut limits = HashSet::new();
		limits.insert(Limit::EdgeRemove);
		result.extend(self.0.shrink_guided(limits).map(|g| Self(g, self.1)));

		// Shrink by removing an edge.
		// Can only remove an edge if there are more than 2 (must have at least 2 edges
		// duplicating each other.
		if self.0.all_edges().count() > 2
		{
			for e in self.0.all_edges()
			{
				// Add to the result a copy of the graph
				// without the edge
				let mut shrunk_graph = self.0.clone();
				let mut shrunk_dup_count = self.1;
				if let Ok(g) = <DirectedGraph<&MockGraph<D>>>::constrain(&self.0)
				{
					if g.edges_sourced_in(e.source()).count() > 1
					{
						// Trying to remove a duplicate edge
						if shrunk_dup_count > 1
						{
							shrunk_dup_count -= 1;
							shrunk_graph.remove_edge(e).unwrap();
							result.push(Self(shrunk_graph, shrunk_dup_count));
						}
						else
						{
							// Cant remove this edge, since it would result in a
							// unique graph
						}
					}
					else
					{
						// A non-duplicate edge can be removed
						shrunk_graph.remove_edge(e).unwrap();
						result.push(Self(shrunk_graph, shrunk_dup_count));
					}
				}
				else
				{
					if self.0.edges_between(e.source(), e.sink()).count() > 1
					{
						// Trying to remove a duplicate edge
						if shrunk_dup_count > 1
						{
							shrunk_dup_count -= 1;
							shrunk_graph.remove_edge(e).unwrap();
							result.push(Self(shrunk_graph, shrunk_dup_count));
						}
						else
						{
							// Cant remove this edge, since it would result in a
							// unique graph
						}
					}
					else
					{
						// A non-duplicate edge can be removed
						shrunk_graph.remove_edge(e).unwrap();
						result.push(Self(shrunk_graph, shrunk_dup_count));
					}
				}
			}
		}

		Box::new(result.into_iter())
	}
}

impl<D: Directedness> ImplGraph for ArbNonUniqueGraph<D>
{
	type Graph = MockGraph<D>;

	fn graph(&self) -> &Self::Graph
	{
		&self.0
	}
}
impl<D: Directedness> ImplGraphMut for ArbNonUniqueGraph<D>
{
	fn graph_mut(&mut self) -> &mut Self::Graph
	{
		&mut self.0
	}
}

use crate::mock_graph::{
	arbitrary::{ArbTwoVerticesIn, ArbVertexIn, GuidedArbGraph, Limit, Unique},
	MockEdgeWeight, MockGraph, MockVertexWeight,
};
use graphene::{
	algo::Dfs,
	common::Ensured,
	core::{
		property::{AcyclicGraph, AddEdge, HasVertex, NewVertex},
		Directed, Directedness, EnsureUnloaded, Graph, ReleaseUnloaded, Undirected,
	},
	impl_ensurer,
};
use quickcheck::{Arbitrary, Gen};
use rand::Rng;
use static_assertions::_core::ops::RangeBounds;
use std::collections::{hash_map::RandomState, HashSet};

/// An arbitrary graph that is acyclic
#[derive(Clone, Debug)]
pub struct ArbAcyclicGraph<D: Directedness>(pub AcyclicGraph<MockGraph<D>>);

impl GuidedArbGraph for ArbAcyclicGraph<Undirected>
{
	fn arbitrary_guided<G: Gen>(
		g: &mut G,
		v_range: impl RangeBounds<usize>,
		e_range: impl RangeBounds<usize>,
	) -> Self
	{
		let (v_min, v_max, e_min, e_max) = Self::validate_ranges(g, v_range, e_range);
		let (v_min, v_max, e_min, e_max) = if e_min > 0
		{
			Self::validate_ranges(g, std::cmp::max(v_min, e_min + 1)..v_max, e_min..e_max)
		}
		else
		{
			(v_min, v_max, e_min, e_max)
		};

		if v_min != (v_max - 1)
		{
			let v_count = g.gen_range(v_min, v_max);
			return Self::arbitrary_guided(g, v_count..=v_count, e_min..e_max);
		}

		Self::ensure_unvalidated(AcyclicGraph::ensure_unvalidated(
			if v_min == 0
			{
				MockGraph::empty()
			}
			else if e_min > 0 && (v_min == e_min + 1)
			{
				let mut graph = MockGraph::empty();
				let mut prev = graph
					.new_vertex_weighted(MockVertexWeight::arbitrary(g))
					.unwrap();

				for _ in 0..=e_min
				{
					let new = graph
						.new_vertex_weighted(MockVertexWeight::arbitrary(g))
						.unwrap();
					graph
						.add_edge_weighted(prev, new, MockEdgeWeight::arbitrary(g))
						.unwrap();
					prev = new;
				}
				graph
			}
			else
			{
				let graph =
					Self::arbitrary_guided(g, (v_min - 1)..v_min, e_min..e_max).release_all();

				// Add a vertex
				let mut graph = graph
					.ensured()
					.new_vertex_weighted(MockVertexWeight::arbitrary(g))
					.unwrap();

				let mut dfs = Dfs::new_simple(&graph);
				let _ = dfs.next(); // Visit our vertex

				let mut candidates = Vec::new();

				for v in graph.all_vertices().filter(|&v| v == graph.get_vertex())
				{
					if !candidates.contains(&v) && !dfs.visited(v)
					{
						candidates.push(v);
						dfs.continue_from(v);
					}
					while dfs.next().is_some()
					{} // Find all reachable
				}

				// Make edges to all candidates
				for v in candidates.into_iter()
				{
					if g.gen_bool(0.5)
					{
						graph
							.add_edge_weighted(graph.get_vertex(), v, MockEdgeWeight::arbitrary(g))
							.unwrap();
					}
				}

				graph.release_all()
			},
		))
	}

	fn shrink_guided(&self, limits: HashSet<Limit, RandomState>) -> Box<dyn Iterator<Item = Self>>
	{
		Box::new(
			self.0
				.clone()
				.release_all()
				.shrink_guided(limits)
				.map(|g| Self::ensure_unvalidated(AcyclicGraph::ensure(g).unwrap())),
		)
	}
}

impl GuidedArbGraph for ArbAcyclicGraph<Directed>
{
	fn arbitrary_guided<G: Gen>(
		g: &mut G,
		v_range: impl RangeBounds<usize>,
		e_range: impl RangeBounds<usize>,
	) -> Self
	{
		let (v_min, v_max, e_min, e_max) = Self::validate_ranges(g, v_range, e_range);
		let (v_min, v_max, e_min, e_max) = if e_min > 0
		{
			Self::validate_ranges(g, std::cmp::max(v_min, e_min + 1)..v_max, e_min..e_max)
		}
		else
		{
			(v_min, v_max, e_min, e_max)
		};

		if v_min != (v_max - 1)
		{
			let v_count = g.gen_range(v_min, v_max);
			return Self::arbitrary_guided(g, v_count..(v_count + 1), e_min..e_max);
		}

		// Create graph with v_min vertices and no edges
		let mut graph = MockGraph::arbitrary_guided(g, v_min..v_max, 0..1);
		let mut order = Vec::new();

		// Create a random order for the vertices
		for v in graph.all_vertices()
		{
			let pos = g.gen_range(order.len(), order.len() + 1);

			order.insert(pos, v);
		}

		if v_min >= 2
		{
			// Add edges randomly, however, ensuring vertices only point
			// to vertices with a higher order
			for _ in 0..g.gen_range(e_min, e_max)
			{
				let (v1, v2) = ArbTwoVerticesIn::<_, Unique>::get_two_vertices(g, &graph);
				if order.iter().position(|&v| v == v1) < order.iter().position(|&v| v == v2)
				{
					graph
						.add_edge_weighted(v1, v2, MockEdgeWeight::arbitrary(g))
						.unwrap();
				}
				else
				{
					graph
						.add_edge_weighted(v2, v1, MockEdgeWeight::arbitrary(g))
						.unwrap();
				}
			}
		}

		Self::ensure_unvalidated(AcyclicGraph::ensure_unvalidated(graph))
	}

	fn shrink_guided(&self, limits: HashSet<Limit, RandomState>) -> Box<dyn Iterator<Item = Self>>
	{
		Box::new(
			self.0
				.clone()
				.release_all()
				.shrink_guided(limits)
				.map(|g| Self::ensure_unvalidated(AcyclicGraph::ensure_unvalidated(g))),
		)
	}
}

impl Arbitrary for ArbAcyclicGraph<Undirected>
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

impl Arbitrary for ArbAcyclicGraph<Directed>
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

impl_ensurer! {
	use<D> ArbAcyclicGraph<D>:
	// Can never impl the following because MockGraph doesn't
	Reflexive
	as (self.0) : AcyclicGraph<MockGraph<D>>
	where D: Directedness
}

/// An arbitrary graph that is cyclic
#[derive(Clone, Debug)]
pub struct ArbCyclicGraph<D: Directedness>(pub MockGraph<D>);

impl<D: Directedness> GuidedArbGraph for ArbCyclicGraph<D>
{
	fn arbitrary_guided<G: Gen>(
		g: &mut G,
		v_range: impl RangeBounds<usize>,
		e_range: impl RangeBounds<usize>,
	) -> Self
	{
		let mut graph = ArbVertexIn::<MockGraph<_>>::arbitrary_guided(g, v_range, e_range);

		let mut reachable: Vec<_> = Dfs::new_simple(&graph).collect();
		reachable.push(graph.get_vertex()); // not added by DFS

		// Add random edge back to the beginning
		graph
			.add_edge_weighted(
				reachable[g.gen_range(0, reachable.len())],
				graph.get_vertex(),
				MockEdgeWeight::arbitrary(g),
			)
			.unwrap();

		ArbCyclicGraph::ensure_unvalidated(graph.release_all())
	}

	fn shrink_guided(&self, limits: HashSet<Limit, RandomState>) -> Box<dyn Iterator<Item = Self>>
	{
		Box::new(
			self.0
				.clone()
				.release_all()
				.shrink_guided(limits)
				.filter(|g| !AcyclicGraph::validate(&g))
				.map(|g| Self::ensure_unvalidated(g)),
		)
	}
}

impl<D: Directedness> Arbitrary for ArbCyclicGraph<D>
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self
	{
		Self::arbitrary_guided(g, 1.., ..)
	}

	fn shrink(&self) -> Box<dyn Iterator<Item = Self>>
	{
		self.shrink_guided(HashSet::new())
	}
}

impl_ensurer! {
	use<D> ArbCyclicGraph<D>:
	// Can never impl the following because MockGraph doesn't
	Reflexive
	as (self.0) : MockGraph<D>
	where D: Directedness
}

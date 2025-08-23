use crate::mock_graph::{
	arbitrary::{
		choose_size_static_edges, CyclicGraph, GuidedArbGraph, Limit,
		Limit::{EdgeKeep, EdgeMin, EdgeRemove, VertexKeep, VertexMin, VertexRemove},
		NonUniqueGraph, UnconnectedGraph,
	},
	MockGraph, MockType,
};
use graphene::{
	core::{
		property::{
			AcyclicGraph, AddEdge, ConnectedGraph, EdgeCount, NewVertex, RemoveVertex, TreeGraph,
			UniqueGraph, VertexCount,
		},
		Directedness, Ensure, Graph, Guard,
	},
	impl_ensurer,
};
use quickcheck::Gen;
use rand::Rng;
use std::collections::HashSet;

impl<D: Directedness, Ew: MockType> GuidedArbGraph for TreeGraph<MockGraph<D, Ew>>
{
	fn choose_size<G: Gen>(
		g: &mut G,
		v_min: usize,
		v_max: usize,
		e_min: usize,
		e_max: usize,
	) -> (usize, usize)
	{
		choose_size_static_edges(g, v_min, v_max, e_min, e_max, |v| {
			(v.saturating_sub(1), v.saturating_sub(1))
		})
	}

	fn arbitrary_fixed<G: Gen>(g: &mut G, v_count: usize, e_count: usize) -> Self
	{
		assert_eq!(v_count.saturating_sub(1), e_count);

		let mut graph = MockGraph::<D, Ew>::arbitrary_fixed(g, 1, 0);

		while graph.vertex_count() < v_count
		{
			let add_on = g.gen_range(0, graph.vertex_count());
			let add_on_v = graph.all_vertices().nth(add_on).unwrap();
			let new_v = graph.new_vertex().unwrap();

			if g.gen_bool(0.5)
			{
				graph
					.add_edge_weighted(add_on_v, new_v, Ew::arbitrary(g))
					.unwrap()
			}
			else
			{
				graph
					.add_edge_weighted(new_v, add_on_v, Ew::arbitrary(g))
					.unwrap()
			}
		}

		Ensure::ensure_unchecked(graph, ())
	}

	fn shrink_guided(&self, limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		let cloned = self.clone();
		Box::new(
			self.all_vertices()
				.collect::<Vec<_>>()
				.into_iter()
				.filter_map(move |v| {
					if !limits.contains(&VertexRemove)
						&& !limits.contains(&EdgeRemove)
						&& !limits.contains(&VertexKeep(v))
						&& !limits.contains(&VertexMin(cloned.vertex_count()))
						&& !limits.contains(&EdgeMin(cloned.edge_count()))
						&& limits
							.iter()
							.find(|l| {
								match l
								{
									EdgeKeep(_, v2) | EdgeKeep(v2, _) if *v2 == v => true,
									_ => false,
								}
							})
							.is_none() && cloned.edges_incident_on(v).count() == 1
					{
						let mut clone = cloned.clone();
						clone.remove_vertex(v).unwrap();
						Some(clone)
					}
					else
					{
						None
					}
				}),
		)
	}
}

/// An arbitrary graph that is __not__ a tree
#[derive(Clone, Debug)]
pub struct NonTreeGraph<D: Directedness, Ew: MockType>(pub MockGraph<D, Ew>);

impl_ensurer! {
	use<D,Ew> NonTreeGraph<D,Ew>:
	// Can never impl the following because MockGraph doesn't
	Reflexive
	as (self.0) : MockGraph<D, Ew>
	where D: Directedness, Ew:MockType
}

impl<D: Directedness, Ew: MockType> GuidedArbGraph for NonTreeGraph<D, Ew>
{
	fn choose_size<G: Gen>(
		g: &mut G,
		v_min: usize,
		v_max: usize,
		e_min: usize,
		e_max: usize,
	) -> (usize, usize)
	{
		assert!(v_max > 1 && e_max > 1);
		MockGraph::<D>::choose_size(
			g,
			std::cmp::max(v_min, 2),
			v_max,
			std::cmp::max(e_min, 2),
			e_max,
		)
	}

	fn arbitrary_fixed<G: Gen>(g: &mut G, v_count: usize, e_count: usize) -> Self
	{
		assert!(v_count >= 1);
		assert!(e_count >= 2);

		Self(match g.gen_range(0, 100)
		{
			0..33 => NonUniqueGraph::arbitrary_fixed(g, v_count, e_count).0,
			33..66 => UnconnectedGraph::arbitrary_fixed(g, v_count, e_count).0,
			66..99 => CyclicGraph::arbitrary_fixed(g, v_count, e_count).0,
			_ =>
			{
				// Empty Graph
				MockGraph::arbitrary_fixed(g, 0, 0)
			},
		})
	}

	fn shrink_guided(&self, limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		Box::new(
			self.0
				.shrink_guided(limits)
				.filter(|g| {
					!UniqueGraph::can_guard(g)
						|| !ConnectedGraph::can_guard(g)
						|| !AcyclicGraph::can_guard(g)
				})
				.map(|g| Self(g)),
		)
	}
}

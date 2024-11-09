use crate::mock_graph::{
	arbitrary::{GuidedArbGraph, Limit},
	MockGraph, MockType,
};
use graphene::{
	core::{
		property::{AddEdge, EdgeCount, UniqueGraph},
		Directedness, Graph, ReleaseUnloaded,
	},
	impl_ensurer,
};
use quickcheck::Gen;
use rand::Rng;
use std::collections::HashSet;

impl<D: Directedness, Ew: MockType> GuidedArbGraph for UniqueGraph<MockGraph<D, Ew>>
{
	fn choose_size<G: Gen>(
		g: &mut G,
		v_min: usize,
		v_max: usize,
		e_min: usize,
		e_max: usize,
	) -> (usize, usize)
	{
		// Used to calculate the maximum possible number of edges
		// in a unique graph with a number of vertices
		let max_allowed =
			|v: usize| v + ((v * (v.saturating_sub(1))) / (2 - (D::directed() as usize)));

		assert!(
			e_min <= max_allowed(v_max),
			"Minimum number of edges higher than theoretically possible: e_min: {}, Max possible: \
			 {}",
			e_min,
			max_allowed(v_max)
		);

		let v_count = g.gen_range(v_min, v_max);

		let t_max = max_allowed(v_count) + 1;
		let e_count = g.gen_range(e_min, std::cmp::min(e_max, t_max));

		(v_count, e_count)
	}

	fn arbitrary_fixed<G: Gen>(g: &mut G, v_count: usize, e_count: usize) -> Self
	{
		let mut graph = MockGraph::<D, Ew>::arbitrary_fixed(g, v_count, 0);
		let verts: Vec<_> = graph.all_vertices().collect();
		let mut edges_added = 0;

		while edges_added < e_count
		{
			assert!(v_count > 0);
			let v1 = verts[g.gen_range(0, v_count)];
			let v2 = verts[g.gen_range(0, v_count)];

			if graph.edges_between(v1, v2).count() == 0
			{
				graph.add_edge_weighted(v1, v2, Ew::arbitrary(g)).unwrap();
				edges_added += 1;
			}
		}

		Self::unchecked(graph)
	}

	fn shrink_guided(&self, limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		Box::new(
			self.clone()
				.release_all()
				.shrink_guided(limits)
				.map(|g| Self::unchecked(g)),
		)
	}
}

/// An arbitrary graph that is __not__ unique
#[derive(Clone, Debug)]
pub struct NonUniqueGraph<D: Directedness, Ew: MockType>(pub MockGraph<D, Ew>);

impl_ensurer! {
	use<D,Ew> NonUniqueGraph<D,Ew>:
	// Can never impl the following because MockGraph doesn't
	Reflexive
	as (self.0) : MockGraph<D, Ew>
	where D: Directedness, Ew:MockType
}

impl<D: Directedness, Ew: MockType> GuidedArbGraph for NonUniqueGraph<D, Ew>
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
			std::cmp::max(v_min, 1),
			v_max,
			std::cmp::max(e_min, 2),
			e_max,
		)
	}

	fn arbitrary_fixed<G: Gen>(g: &mut G, v_count: usize, e_count: usize) -> Self
	{
		assert!(v_count >= 1);
		assert!(e_count >= 2);

		// Create a graph with 1 less edge
		let mut graph = MockGraph::<D, Ew>::arbitrary_fixed(g, v_count, e_count - 1);

		// Duplicate an arbitrary edge
		let (source, sink, _) = graph
			.all_edges()
			.nth(g.gen_range(0, graph.edge_count()))
			.unwrap();
		graph
			.add_edge_weighted(source, sink, Ew::arbitrary(g))
			.unwrap();

		Self(graph)
	}

	fn shrink_guided(&self, limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		Box::new(
			self.0
				.shrink_guided(limits)
				.filter(|g| {
					// Ensure we only keep graphs with duplicate edges
					for (v1, v2, _) in g.all_edges()
					{
						if g.edges_between(v1, v2).count() > 1
						{
							return true;
						}
					}
					false
				})
				.map(|g| Self(g)),
		)
	}
}

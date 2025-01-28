use crate::mock_graph::{
	arbitrary::{GuidedArbGraph, Limit},
	MockType, TestGraph,
};
use graphene::core::{property::VertexInGraph, Ensure, Graph, ReleasePayload};
use quickcheck::Gen;
use rand::Rng;
use std::collections::HashSet;

impl<Gr: GuidedArbGraph, const V: usize, const U: bool> GuidedArbGraph for VertexInGraph<Gr, V, U>
where
	Gr::Graph: TestGraph,
	<Gr::Graph as Graph>::EdgeWeight: MockType,
{
	fn choose_size<G: Gen>(
		g: &mut G,
		v_min: usize,
		v_max: usize,
		e_min: usize,
		e_max: usize,
	) -> (usize, usize)
	{
		assert!(!U || v_max > V);
		Gr::choose_size(g, std::cmp::max(v_min, V), v_max, e_min, e_max)
	}

	fn arbitrary_fixed<G: Gen>(g: &mut G, v_count: usize, e_count: usize) -> Self
	{
		assert!(!U || v_count >= V);
		let graph = Gr::arbitrary_fixed(g, v_count, e_count);

		// Collect V vertices through elimination
		let mut all_vs: Vec<_> = graph.graph().all_vertices().collect();
		let mut chosen_vs = Vec::new();
		while chosen_vs.len() != V
		{
			let idx = g.gen_range(0, all_vs.len());
			chosen_vs.push(all_vs[idx]);
			if U
			{
				all_vs.remove(idx);
			}
		}

		let final_vs: [_; V] = chosen_vs.try_into().unwrap();

		Self::ensure_unchecked(graph, final_vs)
	}

	fn shrink_guided(&self, mut limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		let (g, vs) = self.clone().release();
		vs.iter().for_each(|v| {
			limits.insert(Limit::VertexKeep(*v));
		});
		Box::new(
			// Shrink without changing the designated vertices
			g
				.shrink_guided(limits)
				.map(move |g| Self::ensure_unchecked(g, vs))
				// If non-unique, make a vertex equal it predecessor
				.chain({
					// Collect vertices
					let (g, vs) = self.clone().release();
					(1..V).filter(|_| !U).map(move|i| {
						let mut vs_clone = vs.clone();
						if vs[i] != vs[i-1] {
							vs_clone[i] = vs[i-1]
						}
						Self::ensure_unchecked(g.clone(), vs_clone)
					})
				}),
		)
	}
}

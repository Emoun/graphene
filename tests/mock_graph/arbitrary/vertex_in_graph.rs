use crate::mock_graph::{
	arbitrary::{GuidedArbGraph, Limit},
	MockType, TestGraph,
};
use graphene::core::{
	property::{HasVertex, VertexInGraph},
	Ensure, Graph, Release,
};
use quickcheck::Gen;
use rand::Rng;
use std::collections::HashSet;

impl<Gr: GuidedArbGraph, const V: usize> GuidedArbGraph for VertexInGraph<Gr, V>
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
		assert!(v_max > V);
		Gr::choose_size(g, std::cmp::max(v_min, V), v_max, e_min, e_max)
	}

	fn arbitrary_fixed<G: Gen>(g: &mut G, v_count: usize, e_count: usize) -> Self
	{
		assert!(v_count >= V);
		let graph = Gr::arbitrary_fixed(g, v_count, e_count);
		
		// Collect V vertices through elimination
		let mut all_vs : Vec<_> = graph .graph().all_vertices().collect();
		while all_vs.len() > V {
			let remove_idx = g.gen_range(0, all_vs.len());
			all_vs.remove(remove_idx);
		}
		
		let final_vs : [_;V] =  all_vs.try_into().unwrap();

		Self::ensure_unchecked(graph, final_vs)
	}

	fn shrink_guided(&self, mut limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		let vs: [_;V]= (0..V).map(|i| self.get_vertex_idx(i)).collect::<Vec<_>>().try_into().unwrap();
		vs.iter().for_each(|v| {limits.insert(Limit::VertexKeep(*v));});
		Box::new(
			self.clone()
				.release()
				.shrink_guided(limits)
				.map(move |g| Self::ensure_unchecked(g, vs)),
		)
	}
}

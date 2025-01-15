use crate::{
	algo::{path_exists, Dfs},
	core::{
		property::{AddEdge, NoLoops, VertexInGraph},
		Directedness, Ensure, Graph, GraphDerefMut,
	},
};
use std::borrow::Borrow;

/// An acyclic graph
pub trait Acyclic: NoLoops {}

#[derive(Clone, Debug)]
pub struct AcyclicGraph<C: Ensure>(C);

impl<C: Ensure> Ensure for AcyclicGraph<C>
{
	fn ensure_unchecked(c: Self::Ensured, _: ()) -> Self
	{
		Self(c)
	}

	fn can_ensure(c: &Self::Ensured, _: &()) -> bool
	{
		fn on_visit<G: Graph>(dfs: &mut Dfs<G, (Vec<G::Vertex>, &mut bool)>, v: G::Vertex)
		{
			dfs.payload.0.push(v);
		}
		fn on_exit<G: Graph>(_: &G, _: G::Vertex, (stack, _): &mut (Vec<G::Vertex>, &mut bool))
		{
			stack.pop();
		}
		fn on_explore<G: Graph>(
			dfs: &mut Dfs<G, (Vec<G::Vertex>, &mut bool)>,
			source: G::Vertex,
			sink: G::Vertex,
			_: &G::EdgeWeight,
		)
		{
			if G::Directedness::directed()
			{
				*dfs.payload.1 |= dfs.payload.0.contains(&sink);
			}
			else
			{
				// Check whether the second to last element is the same as the sink
				// (the last element is the same as source, since on_visit is called
				// before exploration)
				if dfs.payload.0.len() >= 2
					&& dfs
						.payload
						.0
						.get(dfs.payload.0.len() - 2)
						.map_or(false, |&v| v == sink)
				{
					// This is an edge to the direct predecessor
					// Therefore, only counts as a cycle if there
					// are multiple edge between these two
					*dfs.payload.1 |= dfs.graph.edges_between(source, sink).nth(1).is_some();
				}
				else
				{
					*dfs.payload.1 |= dfs.visited(sink);
				}
			}
		}
		let mut result = false;
		let mut done = Vec::new();

		// Ensure we have explored all the graph
		for v in c.graph().all_vertices()
		{
			if !done.contains(&v)
			{
				done.push(v); // not returned by the dfs
				let g = VertexInGraph::ensure_unchecked(c.graph(), [v]);
				let dfs = Dfs::new(&g, on_visit, on_exit, on_explore, (Vec::new(), &mut result));

				dfs.for_each(|v| {
					if !done.contains(&v)
					{
						done.push(v)
					}
				});
			}
		}
		!result
	}
}

impl<C: Ensure + GraphDerefMut> AddEdge for AcyclicGraph<C>
where
	C::Graph: AddEdge,
{
	fn add_edge_weighted(
		&mut self,
		source: impl Borrow<Self::Vertex>,
		sink: impl Borrow<Self::Vertex>,
		weight: Self::EdgeWeight,
	) -> Result<(), ()>
	{
		if !path_exists(self, sink.borrow(), source.borrow())
		{
			self.0.graph_mut().add_edge_weighted(source, sink, weight)
		}
		else
		{
			Err(())
		}
	}
}

impl<C: Ensure> NoLoops for AcyclicGraph<C> {}
impl<C: Ensure> Acyclic for AcyclicGraph<C> {}

impl_ensurer! {
	use<C> AcyclicGraph<C>: Ensure, Acyclic, NoLoops, AddEdge
	as (self.0) : C
	where C: Ensure
}

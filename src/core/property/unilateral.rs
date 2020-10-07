use crate::{
	algo::TarjanScc,
	core::{
		property::{
			proxy_remove_edge_where_weight, proxy_remove_vertex, HasVertexGraph, RemoveEdge,
			RemoveVertex, Subgraph, Weak,
		},
		Directed, Ensure, Graph, GraphDerefMut,
	},
};
use std::borrow::Borrow;

/// A marker trait for graphs that are unilaterally connected.
///
/// A graph is unilaterally connected if, for each pair of vertices, there
/// exists at least 1 path from one of them to the other. This is contrasted
/// with strongly connected graphs, where there must exist a path from either to
/// the other (i.e. a path in each direction).
///
/// The distinction between unilaterally and strongly connected only exists for
/// directed graphs, for undirected ones, they are equal. For this reason, the
/// companion ensurer graph `UnilateralGraph` only allows directed graphs.
/// For undirected graph, simply use `ConnectedGraph`.
///
/// For type safety reasons, the trait itself does not restrict directedness.
pub trait Unilateral: Weak
{
}

#[derive(Clone, Debug)]
pub struct UnilateralGraph<C: Ensure>(C)
where
	C::Graph: Graph<Directedness = Directed>;

impl<C: Ensure> Ensure for UnilateralGraph<C>
where
	C::Graph: Graph<Directedness = Directed>,
{
	fn ensure_unvalidated(c: Self::Ensured, _: ()) -> Self
	{
		Self(c)
	}

	fn validate(c: &Self::Ensured, _: &()) -> bool
	{
		if let Ok(graph) = HasVertexGraph::ensure(c.graph(), ())
		{
			// Algorithm: First use Tarjan's Strongly Connected Component (SCC) algorithm to
			// find SCCs and then check whether every component has an edge to the next one
			// in the list. Note: Tarjan's  algorithm produces SCCs in reverse topological
			// order, so we don't need to sort, just check the first has an edge to it from
			// the next.

			let mut tarjan = TarjanScc::new(&graph);

			let mut scc_current = tarjan.next();

			while let Some(scc1) = &scc_current
			{
				let scc_next = tarjan.next();
				if let Some(scc2) = &scc_next
				{
					if scc2.reaches(scc1).is_none()
					{
						return false;
					}
				}
				scc_current = scc_next;
			}
		}
		true
	}
}

impl<C: Ensure + GraphDerefMut> RemoveVertex for UnilateralGraph<C>
where
	C::Graph: RemoveVertex<Directedness = Directed>,
{
	fn remove_vertex(&mut self, v: impl Borrow<Self::Vertex>) -> Result<Self::VertexWeight, ()>
	{
		proxy_remove_vertex::<UnilateralGraph<_>, _>(self.0.graph_mut(), v.borrow())
	}
}

impl<C: Ensure + GraphDerefMut> RemoveEdge for UnilateralGraph<C>
where
	C::Graph: RemoveEdge<Directedness = Directed>,
{
	fn remove_edge_where_weight<F>(
		&mut self,
		source: impl Borrow<Self::Vertex>,
		sink: impl Borrow<Self::Vertex>,
		f: F,
	) -> Result<Self::EdgeWeight, ()>
	where
		F: Fn(&Self::EdgeWeight) -> bool,
	{
		proxy_remove_edge_where_weight::<UnilateralGraph<_>, _, _>(
			self.0.graph_mut(),
			source.borrow(),
			sink.borrow(),
			f,
		)
	}
}

impl<C: Ensure> Weak for UnilateralGraph<C> where C::Graph: Graph<Directedness = Directed> {}
impl<C: Ensure> Unilateral for UnilateralGraph<C> where C::Graph: Graph<Directedness = Directed> {}

impl_ensurer! {
	use<C> UnilateralGraph<C>: Ensure, Unilateral, Weak, RemoveVertex, RemoveEdge,
	// A new vertex would be unconnected to the rest of the graph
	NewVertex
	as (self.0) : C
	where C::Graph: Graph<Directedness=Directed>
}

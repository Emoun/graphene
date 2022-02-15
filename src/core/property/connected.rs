use crate::{
	algo::Dfs,
	core::{
		property::{
			proxy_remove_edge_where_weight, proxy_remove_vertex, DirectedGraph, HasVertexGraph,
			RemoveEdge, RemoveVertex, Unilateral, Weak,
		},
		proxy::ReverseGraph,
		Ensure, Graph, GraphDerefMut,
	},
};
use std::borrow::Borrow;

/// A marker trait for graphs that are connected.
///
/// A graph is connected if there is a path from any vertex to any other vertex.
/// Graphs with one or zero vertices count as connected.
pub trait Connected: Unilateral {}

#[derive(Clone, Debug)]
pub struct ConnectedGraph<C: Ensure>(C);

impl<C: Ensure> ConnectedGraph<C>
{
	/// Creates a new connected graph. The given graph *must* be connected.
	/// This method does not check for this!!
	pub fn new(c: C) -> Self
	{
		Self(c)
	}
}

impl<C: Ensure> Ensure for ConnectedGraph<C>
{
	fn ensure_unvalidated(c: Self::Ensured, _: ()) -> Self
	{
		Self(c)
	}

	fn validate(c: &Self::Ensured, _: &()) -> bool
	{
		let g = c.graph();
		let v_count = g.all_vertices().count();

		if let Ok(g) = HasVertexGraph::ensure(g, ())
		{
			let dfs_count = Dfs::new_simple(&g).count();
			if (dfs_count + 1) == v_count
			{
				// If its undirected, no more needs to be done
				if let Ok(g) = DirectedGraph::ensure(g, ())
				{
					let reverse = ReverseGraph::new(g);
					if (Dfs::new_simple(&reverse).count() + 1) != v_count
					{
						return false;
					}
				}
				return true;
			}
			return false;
		}
		true
	}
}

impl<C: Ensure + GraphDerefMut> RemoveVertex for ConnectedGraph<C>
where
	C::Graph: RemoveVertex,
{
	fn remove_vertex(&mut self, v: impl Borrow<Self::Vertex>) -> Result<Self::VertexWeight, ()>
	{
		proxy_remove_vertex::<ConnectedGraph<_>, _>(self.0.graph_mut(), v.borrow())
	}
}

impl<C: Ensure + GraphDerefMut> RemoveEdge for ConnectedGraph<C>
where
	C::Graph: RemoveEdge,
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
		proxy_remove_edge_where_weight::<ConnectedGraph<_>, _, _>(
			self.0.graph_mut(),
			source.borrow(),
			sink.borrow(),
			f,
		)
	}
}

impl<C: Ensure> Weak for ConnectedGraph<C> {}
impl<C: Ensure> Unilateral for ConnectedGraph<C> {}
impl<C: Ensure> Connected for ConnectedGraph<C> {}

impl_ensurer! {
	use<C> ConnectedGraph<C>: Ensure, Connected, Unilateral, Weak, RemoveVertex, RemoveEdge,
	// A new vertex wouldn't be connected to the rest of the graph
	NewVertex
	as (self.0) : C
}

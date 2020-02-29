use crate::{
	algo::DFS,
	core::{
		property::{
			proxy_remove_edge_where, proxy_remove_vertex, DirectedGraph, RemoveEdge, RemoveVertex,
			Unilateral, Weak,
		},
		Graph, GraphDerefMut, Insure, ReverseGraph,
	},
};
use delegate::delegate;

/// A marker trait for graphs that are connected.
///
/// A graph is connected if there is a path from any vertex to any other vertex.
/// Graphs with one or zero vertices count as connected.
pub trait Connected: Unilateral
{
}

#[derive(Clone, Debug)]
pub struct ConnectedGraph<C: Insure>(C);

impl<C: Insure> ConnectedGraph<C>
{
	/// Creates a new connected graph. The given graph *must* be connected.
	/// This method does not check for this!!
	pub fn new(c: C) -> Self
	{
		Self(c)
	}
}

impl<C: Insure> Insure for ConnectedGraph<C>
{
	fn insure_unvalidated(c: Self::Insured) -> Self
	{
		Self(c)
	}

	fn validate(c: &Self::Insured) -> bool
	{
		let g = c.graph();
		let v_count = g.all_vertices().count();

		if v_count > 0
		{
			let v = g.all_vertices().next().unwrap();
			let dfs_count = DFS::new_simple(g, v).count();
			if dfs_count == v_count
			{
				// If its undirected, no more needs to be done
				if let Ok(g) = <DirectedGraph<&C::Graph>>::insure_all(g)
				{
					let reverse = ReverseGraph::new(g);
					if DFS::new_simple(&reverse, v).count() != v_count
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

impl<C: Insure + GraphDerefMut> RemoveVertex for ConnectedGraph<C>
where
	C::Graph: RemoveVertex,
{
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>
	{
		proxy_remove_vertex::<ConnectedGraph<_>, _>(self.0.graph_mut(), v)
	}
}

impl<C: Insure + GraphDerefMut> RemoveEdge for ConnectedGraph<C>
where
	C::Graph: RemoveEdge,
{
	fn remove_edge_where<F>(
		&mut self,
		f: F,
	) -> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
	where
		F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool,
	{
		proxy_remove_edge_where::<ConnectedGraph<_>, _, _>(self.0.graph_mut(), f)
	}
}

impl<C: Insure> Weak for ConnectedGraph<C> {}
impl<C: Insure> Unilateral for ConnectedGraph<C> {}
impl<C: Insure> Connected for ConnectedGraph<C> {}

impl_insurer! {
	ConnectedGraph<C>: Connected, Unilateral, Weak, RemoveVertex, RemoveEdge,
	// A new vertex wouldn't be connected to the rest of the graph
	NewVertex
}

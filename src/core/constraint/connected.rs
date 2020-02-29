use crate::{
	algo::DFS,
	core::{
		constraint::{
			proxy_remove_edge_where, proxy_remove_vertex, DirectedGraph, RemoveEdge, RemoveVertex,
			Unilateral, Weak,
		},
		Constrainer, Graph, GraphDeref, GraphDerefMut, ReverseGraph,
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
pub struct ConnectedGraph<C: Constrainer>(C);

impl<C: Constrainer> ConnectedGraph<C>
{
	/// Creates a new connected graph. The given graph *must* be connected.
	/// This method does not check for this!!
	pub fn new(c: C) -> Self
	{
		Self(c)
	}
}

impl<C: Constrainer> GraphDeref for ConnectedGraph<C>
{
	type Graph = Self;

	fn graph(&self) -> &Self::Graph
	{
		self
	}
}
impl<C: Constrainer> GraphDerefMut for ConnectedGraph<C>
{
	fn graph_mut(&mut self) -> &mut Self::Graph
	{
		self
	}
}
impl<C: Constrainer> Constrainer for ConnectedGraph<C>
{
	type Base = C::Base;
	type Constrained = C;

	fn constrain_single(c: Self::Constrained) -> Result<Self, ()>
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
				if let Ok(g) = <DirectedGraph<&C::Graph>>::constrain(g)
				{
					let reverse = ReverseGraph::new(g);
					if DFS::new_simple(&reverse, v).count() != v_count
					{
						return Err(());
					}
				}
				return Ok(Self(c));
			}
			return Err(());
		}
		Ok(Self(c))
	}

	fn unconstrain_single(self) -> Self::Constrained
	{
		self.0
	}
}

impl<C: Constrainer + GraphDerefMut> RemoveVertex for ConnectedGraph<C>
where
	C::Graph: RemoveVertex,
{
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>
	{
		proxy_remove_vertex::<ConnectedGraph<_>, _>(self.0.graph_mut(), v)
	}
}

impl<C: Constrainer + GraphDerefMut> RemoveEdge for ConnectedGraph<C>
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

impl<C: Constrainer> Weak for ConnectedGraph<C> {}
impl<C: Constrainer> Unilateral for ConnectedGraph<C> {}
impl<C: Constrainer> Connected for ConnectedGraph<C> {}

impl_constraints! {
	ConnectedGraph<C>: Connected, Unilateral, Weak, RemoveVertex, RemoveEdge,
	// A new vertex wouldn't be connected to the rest of the graph
	NewVertex
}

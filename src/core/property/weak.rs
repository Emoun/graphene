use crate::core::{
	property::{
		proxy_remove_edge_where, proxy_remove_vertex, ConnectedGraph, RemoveEdge, RemoveVertex,
	},
	proxy::UndirectedProxy,
	Directed, Graph, GraphDerefMut, Insure,
};

/// A marker trait for graphs that are weakly connected.
///
/// A graph is weakly connected if, when replacing all edges by undirected
/// versions, there exists a path between every pair of vertices in the graph.
///
/// The distinction between weakly and strongly connected only exists for
/// directed graphs, for undirected ones, they are equal. For this reason, the
/// companion insurer graph `WeakGraph` only allows directed graphs. For
/// undirected graph, simply use `ConnectedGraph`.
///
/// For type safety reasons, the trait itself does not restrict directedness.
pub trait Weak: Graph
{
}

#[derive(Clone, Debug)]
pub struct WeakGraph<C: Insure>(C)
where
	C::Graph: Graph<Directedness = Directed>;

impl<C: Insure> WeakGraph<C>
where
	C::Graph: Graph<Directedness = Directed>,
{
	/// Creates a new weakly connected graph.
	/// The given graph *must* be weakly connected.
	/// This method does not check for this!!
	pub fn new(c: C) -> Self
	{
		Self(c)
	}
}

impl<C: Insure> Insure for WeakGraph<C>
where
	C::Graph: Graph<Directedness = Directed>,
{
	fn insure_unvalidated(c: Self::Insured) -> Self
	{
		Self(c)
	}

	fn validate(c: &Self::Insured) -> bool
	{
		let undirected = UndirectedProxy::new(c.graph());

		ConnectedGraph::validate(&undirected)
	}
}

impl<C: Insure + GraphDerefMut> RemoveVertex for WeakGraph<C>
where
	C::Graph: RemoveVertex<Directedness = Directed>,
{
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>
	{
		proxy_remove_vertex::<WeakGraph<_>, _>(self.0.graph_mut(), v)
	}
}

impl<C: Insure + GraphDerefMut> RemoveEdge for WeakGraph<C>
where
	C::Graph: RemoveEdge<Directedness = Directed>,
{
	fn remove_edge_where<F>(
		&mut self,
		f: F,
	) -> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
	where
		F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool,
	{
		proxy_remove_edge_where::<WeakGraph<_>, _, _>(self.0.graph_mut(), f)
	}
}

impl<C: Insure> Weak for WeakGraph<C> where C::Graph: Graph<Directedness = Directed> {}

impl_insurer! {
	WeakGraph<C>: Insure, Weak, RemoveVertex, RemoveEdge,
	// A new vertex wouldn't be connected to the rest of the graph
	NewVertex
	for <C> as (self.0)
	where C::Graph: Graph<Directedness=Directed>
}

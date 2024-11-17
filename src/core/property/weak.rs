use crate::core::{
	property::{
		proxy_remove_edge_where_weight, proxy_remove_vertex, ConnectedGraph, RemoveEdge,
		RemoveVertex,
	},
	proxy::UndirectedProxy,
	Directed, Ensure, Graph, GraphDerefMut,
};
use std::borrow::Borrow;

/// A marker trait for graphs that are weakly connected.
///
/// A graph is weakly connected if, when replacing all edges by undirected
/// versions, there exists a path between every pair of vertices in the graph.
///
/// The distinction between weakly and strongly connected only exists for
/// directed graphs, for undirected ones, they are equal. For this reason, the
/// companion ensurer graph `WeakGraph` only allows directed graphs. For
/// undirected graph, simply use `ConnectedGraph`.
///
/// For type safety reasons, the trait itself does not restrict directedness.
pub trait Weak: Graph {}

#[derive(Clone, Debug)]
pub struct WeakGraph<C: Ensure>(C)
where
	C::Graph: Graph<Directedness = Directed>;

impl<C: Ensure> WeakGraph<C>
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

impl<C: Ensure> Ensure for WeakGraph<C>
where
	C::Graph: Graph<Directedness = Directed>,
{
	fn ensure_unchecked(c: Self::Ensured, _: ()) -> Self
	{
		Self(c)
	}

	fn can_ensure(c: &Self::Ensured, _: &()) -> bool
	{
		let undirected = UndirectedProxy::new(c.graph());

		ConnectedGraph::can_ensure(&undirected, &())
	}
}

impl<C: Ensure + GraphDerefMut> RemoveVertex for WeakGraph<C>
where
	C::Graph: RemoveVertex<Directedness = Directed>,
{
	fn remove_vertex(&mut self, v: impl Borrow<Self::Vertex>) -> Result<Self::VertexWeight, ()>
	{
		proxy_remove_vertex::<WeakGraph<_>, _>(self.0.graph_mut(), v.borrow())
	}
}

impl<C: Ensure + GraphDerefMut> RemoveEdge for WeakGraph<C>
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
		proxy_remove_edge_where_weight::<WeakGraph<_>, _, _>(
			self.0.graph_mut(),
			source.borrow(),
			sink.borrow(),
			f,
		)
	}
}

impl<C: Ensure> Weak for WeakGraph<C> where C::Graph: Graph<Directedness = Directed> {}

impl_ensurer! {
	use<C> WeakGraph<C>: Ensure, Weak, RemoveVertex, RemoveEdge,
	// A new vertex wouldn't be connected to the rest of the graph
	NewVertex
	as (self.0) : C
	where C::Graph: Graph<Directedness=Directed>
}

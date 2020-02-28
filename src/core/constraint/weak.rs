use crate::core::{
	constraint::{
		proxy_remove_edge_where, proxy_remove_vertex, ConnectedGraph, RemoveEdge, RemoveVertex,
	},
	proxy::UndirectedProxy,
	Constrainer, Directed, Graph, GraphDeref, GraphDerefMut, GraphMut,
};
use delegate::delegate;

/// A marker trait for graphs that are weakly connected.
///
/// A graph is weakly connected if, when replacing all edges by undirected
/// versions, there exists a path between every pair of vertices in the graph.
///
/// The distinction between weakly and strongly connected only exists for
/// directed graphs, for undirected ones, they are equal. For this reason, the
/// companion constrainer graph `WeakGraph` only allows directed graphs. For
/// undirected graph, simply use `ConnectedGraph`.
///
/// For type safety reasons, the trait itself does not restrict directedness.
pub trait Weak: Graph
{
}

#[derive(Clone, Debug)]
pub struct WeakGraph<C: Constrainer>(C)
where
	C::Graph: Graph<Directedness = Directed>;

impl<C: Constrainer> WeakGraph<C>
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

impl<C: Constrainer> GraphDeref for WeakGraph<C>
where
	C::Graph: Graph<Directedness = Directed>,
{
	type Graph = Self;

	fn graph(&self) -> &Self::Graph
	{
		self
	}
}
impl<C: Constrainer> GraphDerefMut for WeakGraph<C>
where
	C::Graph: Graph<Directedness = Directed>,
{
	fn graph_mut(&mut self) -> &mut Self::Graph
	{
		self
	}
}
impl<C: Constrainer> Constrainer for WeakGraph<C>
where
	C::Graph: Graph<Directedness = Directed>,
{
	type Base = C::Base;
	type Constrained = C;

	fn constrain_single(c: Self::Constrained) -> Result<Self, ()>
	{
		let undirected = UndirectedProxy::new(c.graph());

		if ConnectedGraph::constrain_single(undirected).is_ok()
		{
			Ok(WeakGraph::new(c))
		}
		else
		{
			Err(())
		}
	}

	fn unconstrain_single(self) -> Self::Constrained
	{
		self.0
	}
}

impl<C: Constrainer> Graph for WeakGraph<C>
where
	C::Graph: Graph<Directedness = Directed>,
{
	type Directedness = <C::Graph as Graph>::Directedness;
	type EdgeWeight = <C::Graph as Graph>::EdgeWeight;
	type Vertex = <C::Graph as Graph>::Vertex;
	type VertexWeight = <C::Graph as Graph>::VertexWeight;

	delegate! {
		to self.0.graph() {
			fn all_vertices_weighted<'a>(
				&'a self,
			) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, &'a Self::VertexWeight)>>;

			fn all_edges<'a>(
				&'a self,
			) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>;
		}
	}
}

impl<C: Constrainer + GraphDerefMut> GraphMut for WeakGraph<C>
where
	C::Graph: GraphMut<Directedness = Directed>,
{
	delegate! {
		to self.0.graph_mut() {
			fn all_vertices_weighted_mut<'a>(
				&'a mut self,
			) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, &'a mut Self::VertexWeight)>>;

			fn all_edges_mut<'a>(
				&'a mut self,
			) -> Box<
				dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>
			>;
		}
	}
}

impl<C: Constrainer + GraphDerefMut> RemoveVertex for WeakGraph<C>
where
	C::Graph: RemoveVertex<Directedness = Directed>,
{
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>
	{
		proxy_remove_vertex::<WeakGraph<_>, _>(self.0.graph_mut(), v)
	}
}

impl<C: Constrainer + GraphDerefMut> RemoveEdge for WeakGraph<C>
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

impl<C: Constrainer> Weak for WeakGraph<C> where C::Graph: Graph<Directedness = Directed> {}

impl_constraints! {
	WeakGraph<C>: Weak, RemoveVertex, RemoveEdge,
	// A new vertex wouldn't be connected to the rest of the graph
	NewVertex
	where C::Graph: Graph<Directedness=Directed>
}

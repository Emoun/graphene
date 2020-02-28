use crate::{
	algo::TarjanSCC,
	core::{
		constraint::{
			proxy_remove_edge_where, proxy_remove_vertex, RemoveEdge, RemoveVertex, Subgraph, Weak,
		},
		Constrainer, Directed, Graph, GraphDeref, GraphDerefMut, GraphMut,
	},
};
use delegate::delegate;

/// A marker trait for graphs that are unilaterally connected.
///
/// A graph is unilaterally connected if, for each pair of vertices, there
/// exists at least 1 path from one of them to the other. This is contrasted
/// with strongly connected graphs, where there must exist a path from either to
/// the other (i.e. a path in each direction).
///
/// The distinction between unilaterally and strongly connected only exists for
/// directed graphs, for undirected ones, they are equal. For this reason, the
/// companion constrainer graph `UnilateralGraph` only allows directed graphs.
/// For undirected graph, simply use `ConnectedGraph`.
///
/// For type safety reasons, the trait itself does not restrict directedness.
pub trait Unilateral: Weak
{
}

#[derive(Clone, Debug)]
pub struct UnilateralGraph<C: Constrainer>(C)
where
	C::Graph: Graph<Directedness = Directed>;

impl<C: Constrainer> UnilateralGraph<C>
where
	C::Graph: Graph<Directedness = Directed>,
{
	/// Creates a new unilaterally connected graph.
	/// The given graph *must* be unilaterally connected.
	/// This method does not check for this!!
	pub fn new(c: C) -> Self
	{
		Self(c)
	}
}

impl<C: Constrainer> GraphDeref for UnilateralGraph<C>
where
	C::Graph: Graph<Directedness = Directed>,
{
	type Graph = Self;

	fn graph(&self) -> &Self::Graph
	{
		self
	}
}
impl<C: Constrainer> GraphDerefMut for UnilateralGraph<C>
where
	C::Graph: Graph<Directedness = Directed>,
{
	fn graph_mut(&mut self) -> &mut Self::Graph
	{
		self
	}
}
impl<C: Constrainer> Constrainer for UnilateralGraph<C>
where
	C::Graph: Graph<Directedness = Directed>,
{
	type Base = C::Base;
	type Constrained = C;

	fn constrain_single(c: Self::Constrained) -> Result<Self, ()>
	{
		let graph = c.graph();
		let verts = graph.all_vertices().collect::<Vec<_>>();

		if verts.len() != 0
		{
			// Algorithm: First use Tarjan's Strongly Connected Component (SCC) algorithm to
			// find SCCs and then check whether every component has an edge to the next one
			// in the list. Note: Tarjan's  algorithm produces SCCs in reverse topological
			// order, so we don't need to sort, just check the first has an edge to it from
			// the next.

			let mut tarjan = TarjanSCC::new(graph, verts[0]);

			let mut scc_current = tarjan.next();

			while let Some(scc1) = &scc_current
			{
				let scc_next = tarjan.next();
				if let Some(scc2) = &scc_next
				{
					if scc2.reaches(scc1).is_none()
					{
						return Err(());
					}
				}
				scc_current = scc_next;
			}
		}
		Ok(Self::new(c))
	}

	fn unconstrain_single(self) -> Self::Constrained
	{
		self.0
	}
}

impl<C: Constrainer> Graph for UnilateralGraph<C>
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

impl<C: Constrainer + GraphDerefMut> GraphMut for UnilateralGraph<C>
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

impl<C: Constrainer + GraphDerefMut> RemoveVertex for UnilateralGraph<C>
where
	C::Graph: RemoveVertex<Directedness = Directed>,
{
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>
	{
		proxy_remove_vertex::<UnilateralGraph<_>, _>(self.0.graph_mut(), v)
	}
}

impl<C: Constrainer + GraphDerefMut> RemoveEdge for UnilateralGraph<C>
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
		proxy_remove_edge_where::<UnilateralGraph<_>, _, _>(self.0.graph_mut(), f)
	}
}

impl<C: Constrainer> Weak for UnilateralGraph<C> where C::Graph: Graph<Directedness = Directed> {}
impl<C: Constrainer> Unilateral for UnilateralGraph<C> where C::Graph: Graph<Directedness = Directed>
{}

impl_constraints! {
	UnilateralGraph<C>: Unilateral, Weak, RemoveVertex, RemoveEdge,
	// A new vertex would be unconnected to the rest of the graph
	NewVertex
	where C::Graph: Graph<Directedness=Directed>
}

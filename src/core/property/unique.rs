use crate::core::{
	property::AddEdge, Directedness, Edge, EdgeWeighted, Ensure, Graph, GraphDerefMut,
};

/// A marker trait for graphs containing only unique edges.
///
/// An edge is unique if it is the only edge in the graph
/// connecting two vertices.
/// If the graph is directed then between two vertices v1 and v2
/// two edges are allowed: (v1,v2,_) and (v2,v1,_).
/// If the graph is undirected, there may only be one edge of either
/// (v1,v2,_) or (v1,v2,_).
/// Regardless of directedness, only one loop is allowed for each vertex,
/// i.e. only one (v,v,_).
pub trait Unique: Graph
{
	fn edge_between(&self, v1: Self::Vertex, v2: Self::Vertex) -> Option<&Self::EdgeWeight>
	{
		self.edges_between(v1, v2).next().map(|(_, _, w)| w)
	}
}

#[derive(Clone, Debug)]
pub struct UniqueGraph<C: Ensure>(C);

impl<C: Ensure> UniqueGraph<C>
{
	/// Constrains the given graph.
	///
	/// The given graph must be unique. This is not checked by this function.
	pub fn unchecked(c: C) -> Self
	{
		Self(c)
	}
}

impl<C: Ensure> Ensure for UniqueGraph<C>
{
	fn ensure_unvalidated(c: Self::Ensured) -> Self
	{
		Self(c)
	}

	fn validate(c: &Self::Ensured) -> bool
	{
		let edges: Vec<_> = c.graph().all_edges().collect();
		let mut iter = edges.iter();
		while let Some(e) = iter.next()
		{
			for e2 in iter.clone()
			{
				if (e.source() == e2.source() && e.sink() == e2.sink())
					|| (e.source() == e2.sink()
						&& e.sink() == e2.source()
						&& !<C::Graph as Graph>::Directedness::directed())
				{
					return false;
				}
			}
		}
		true
	}
}

impl<C: Ensure + GraphDerefMut> AddEdge for UniqueGraph<C>
where
	C::Graph: AddEdge,
{
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
	where
		E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>,
	{
		if Self::Directedness::directed()
		{
			if self
				.edges_between(e.source(), e.sink())
				.any(|edge| e.source() == edge.source() && e.sink() == edge.sink())
			{
				return Err(());
			}
		}
		else
		{
			if self.edges_between(e.source(), e.sink()).next().is_some()
			{
				return Err(());
			}
		}
		self.0.graph_mut().add_edge_weighted(e)
	}
}

impl<C: Ensure> Unique for UniqueGraph<C> {}

impl_ensurer! {
	UniqueGraph<C>: Ensure, Unique, AddEdge
	for <C> as (self.0)
	where C: Ensure
}

use crate::core::{property::AddEdge, Directedness, Edge, Ensure, Graph, GraphDerefMut};
use std::borrow::Borrow;

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
	fn edge_between(
		&self,
		v1: impl Borrow<Self::Vertex>,
		v2: impl Borrow<Self::Vertex>,
	) -> Option<Self::EdgeWeightRef<'_>>
	{
		self.edges_between(v1, v2).next()
	}

	/// Returns whether the given edge can be added to the graph without
	/// violating uniqueness
	fn can_add_edge<G: Graph>(
		graph: &G,
		source: impl Borrow<G::Vertex>,
		sink: impl Borrow<G::Vertex>,
	) -> bool
	{
		graph.edges_between(source, sink).next().is_none()
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
	fn ensure_unchecked(c: Self::Ensured, _: ()) -> Self
	{
		Self(c)
	}

	fn can_ensure(c: &Self::Ensured, _: &()) -> bool
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
	fn add_edge_weighted(
		&mut self,
		source: impl Borrow<Self::Vertex>,
		sink: impl Borrow<Self::Vertex>,
		weight: Self::EdgeWeight,
	) -> Result<(), ()>
	{
		if !Self::can_add_edge(self, source.borrow(), sink.borrow())
		{
			return Err(());
		}
		self.0.graph_mut().add_edge_weighted(source, sink, weight)
	}
}

impl<C: Ensure> Unique for UniqueGraph<C> {}

impl_ensurer! {
	use<C> UniqueGraph<C>: Ensure, Unique, AddEdge
	as (self.0) : C
}

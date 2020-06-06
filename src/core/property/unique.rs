use crate::core::{property::AddEdge, Directedness, Ensure, Graph, GraphDerefMut};

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
	fn edge_between<'a>(
		&'a self,
		v1: &'a Self::Vertex,
		v2: &'a Self::Vertex,
	) -> Option<&'a Self::EdgeWeight>
	{
		self.edges_between(v1, v2).next()
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
	fn ensure_unvalidated(c: Self::Ensured, _: ()) -> Self
	{
		Self(c)
	}

	fn validate(c: &Self::Ensured, _: &()) -> bool
	{
		let verts: Vec<_> = c.graph().all_vertices().collect();
		let mut iter = verts.iter();
		// Clone iter before every loop, so that loop-edges are also analyzed.
		let mut iter_rest = iter.clone();

		while let Some(v) = iter.next()
		{
			for v2 in iter_rest
			{
				if c.graph().edges_between(v, v2).nth(1).is_some()
					|| (<C::Graph as Graph>::Directedness::directed()
						&& (v != v2) && c.graph().edges_between(v2, v).nth(1).is_some())
				{
					return false;
				}
			}
			iter_rest = iter.clone();
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
		source: &Self::Vertex,
		sink: &Self::Vertex,
		weight: Self::EdgeWeight,
	) -> Result<(), ()>
	{
		if self.edges_between(source, sink).next().is_some()
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
	where C: Ensure
}

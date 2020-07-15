use crate::core::{Edge, Graph};
use std::borrow::Borrow;

pub trait Subgraph: Graph
{
	/// Edges who's sources are in this subgraph but who's sinks aren't.
	fn exit_edges<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex)>>;

	/// Whether this subgraph can reach a vertex in the other subgraph, either
	/// by sharing a vertex with it, or having an exit edge to one of its
	/// vertices.
	fn reaches<G>(&self, other: &G) -> Option<(Self::Vertex, Self::Vertex)>
	where
		G: Subgraph<
			Vertex = Self::Vertex,
			VertexWeight = Self::VertexWeight,
			EdgeWeight = Self::EdgeWeight,
			Directedness = Self::Directedness,
			VertexRef = Self::VertexRef,
		>,
	{
		// Check whether they share any vertex
		for v in other.all_vertices()
		{
			if self.contains_vertex(v.borrow())
			{
				return Some((v.borrow().clone(), v.borrow().clone()));
			}
		}

		// Check whether an exit edge is sinked in the other subgraph
		for e in self.exit_edges()
		{
			if other.all_vertices().any(|v| *v.borrow() == e.sink())
			{
				return Some((e.source(), e.sink()));
			}
		}

		None
	}
}

use crate::core::Graph;
use std::borrow::Borrow;

pub trait Subgraph: Graph
{
	/// Edges who's sources are in this subgraph but who's sinks aren't.
	fn exit_edges<'a>(
		&'a self,
	) -> Box<dyn 'a + Iterator<Item = (Self::VertexRef, Self::VertexRef)>>;

	/// Whether this subgraph can reach a vertex in the other subgraph, either
	/// by sharing a vertex with it, or having an exit edge to one of its
	/// vertices.
	fn reaches<G>(&self, other: &G) -> Option<(Self::VertexRef, Self::VertexRef)>
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
			if self.contains_vertex(v.clone())
			{
				return Some((v.clone(), v.clone()));
			}
		}

		// Check whether an exit edge is sinked in the other subgraph
		for (source, sink) in self.exit_edges()
		{
			if other.all_vertices().any(|v| v.borrow() == sink.borrow())
			{
				return Some((source, sink));
			}
		}

		None
	}
}

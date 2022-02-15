use crate::core::{Edge, Graph};

pub trait Subgraph: Graph
{
	/// Edges who's sources are in this subgraph but who's sinks aren't.
	fn exit_edges(&self) -> Box<dyn '_ + Iterator<Item = (Self::Vertex, Self::Vertex)>>;

	/// Whether this subgraph can reach a vertex in the other subgraph, either
	/// by sharing a vertex with it, or having an axit edge to one of its
	/// vertices.
	fn reaches<G>(&self, other: &G) -> Option<(Self::Vertex, Self::Vertex)>
	where
		G: Subgraph<
			Vertex = Self::Vertex,
			VertexWeight = Self::VertexWeight,
			EdgeWeight = Self::EdgeWeight,
			Directedness = Self::Directedness,
		>,
	{
		// Check whether they share any vertex
		for v in other.all_vertices()
		{
			if self.contains_vertex(v)
			{
				return Some((v, v));
			}
		}

		// Check whether an exit edge is sinked in the other subgraph
		for e in self.exit_edges()
		{
			if other.all_vertices().any(|v| v == e.sink())
			{
				return Some((e.source(), e.sink()));
			}
		}

		None
	}
}

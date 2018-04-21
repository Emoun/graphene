
use ::core::BaseGraph;

pub trait ExactGraph: BaseGraph
	where
		<Self::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
		<Self::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
{
	
	///
	/// Returns the number of vertices in the graph.
	///
	fn vertex_count(&self) -> usize {
		self.all_vertices().into_iter().len()
	}
	
	///
	/// Returns the number of edges in the graph.
	///
	fn edge_count(&self) -> usize {
		self.all_edges().into_iter().len()
	}
	
}
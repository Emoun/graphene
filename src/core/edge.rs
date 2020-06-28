use crate::core::trait_aliases::Id;

/// An edge in a [graph](trait.Graph.html) with vertices of type `V`.
///
/// An edge is simply a pair of vertices in a graph.
/// Every edge has a distinguished `source` and `sink`.
/// For undirected graphs, which vertex is which has no meaning.
/// For directed graphs, an edge points from the `source` to the `sink`.
///
/// This trait has a blanket implementation implementation for any pair `(V,V)`
/// or triple `(V,V,W)`. Therefore, the easiest way to create an edge is to
/// simply use a pair. The triple can be used if the edge is weighted
pub trait Edge<V>
where
	V: Id,
{
	/// The source vertex of the edge.
	fn source(&self) -> V;
	/// the sink vertex of the edge.
	fn sink(&self) -> V;

	/// Returns `true` if the source and sink are the same vertex.
	fn is_loop(&self) -> bool
	{
		self.source() == self.sink()
	}

	/// Returns the other vertex than the one given.
	///
	/// If the one given is not in this edge, the sink is returned.
	fn other(&self, v: V) -> V
	{
		if self.sink() == v
		{
			self.source()
		}
		else
		{
			self.sink()
		}
	}
}

impl<V> Edge<V> for (V, V)
where
	V: Id,
{
	fn source(&self) -> V
	{
		self.0
	}

	fn sink(&self) -> V
	{
		self.1
	}
}

impl<V, W> Edge<V> for (V, V, W)
where
	V: Id,
{
	fn source(&self) -> V
	{
		self.0
	}

	fn sink(&self) -> V
	{
		self.1
	}
}

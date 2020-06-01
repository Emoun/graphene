use crate::core::{
	property::{NewVertex, RemoveVertex},
	trait_aliases::Id,
	Ensure, Graph,
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ProxyVertex<V: Id>
{
	Underlying(V),
	New(usize),
}

/// A helper proxy graph, that manages addition and removal of
/// vertices.
///
/// It does not handle addition or removal of edges in any way.
pub struct VertexProxyGraph<C: Ensure>
{
	/// The underlying graph
	graph: C,

	/// The number of vertices that aren't in the underlying graph,
	/// that have been added to the proxy.
	/// Since this struct does not guarantee that vertices keep their identifier
	/// upon removal, we just increment and decrement this number whenever a new
	/// number is added.
	new_count: usize,

	/// Vertices that have been removed from the underlying graph.
	removed: Vec<<C::Graph as Graph>::Vertex>,
}

impl<C: Ensure> VertexProxyGraph<C>
{
	pub fn new(underlying: C) -> Self
	{
		Self {
			graph: underlying,
			new_count: 0,
			removed: Vec::new(),
		}
	}
}

impl<C: Ensure> Graph for VertexProxyGraph<C>
{
	type Directedness = <C::Graph as Graph>::Directedness;
	type EdgeWeight = <C::Graph as Graph>::EdgeWeight;
	type Vertex = ProxyVertex<<C::Graph as Graph>::Vertex>;
	type VertexWeight = ();

	fn all_vertices_weighted<'a>(
		&'a self,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, &'a Self::VertexWeight)>>
	{
		Box::new(
			self.graph
				.graph()
				.all_vertices()
				.filter(move |v| !self.removed.contains(v))
				.map(|v| (ProxyVertex::Underlying(v), &()))
				.chain((0..self.new_count).map(|v| (ProxyVertex::New(v), &()))),
		)
	}

	fn all_edges<'a>(
		&'a self,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>
	{
		Box::new(
			self.graph
                .graph()
                .all_edges()
                // Remove any edges connected to a removed vertex
                .filter(move |(so, si, _)| !self.removed.contains(so) && !self.removed.contains(si))
                // Map vertices to ProxyVertices
                .map(|(so, si, w)| (ProxyVertex::Underlying(so), ProxyVertex::Underlying(si), w)),
		)
	}
}

impl<C: Ensure> NewVertex for VertexProxyGraph<C>
{
	fn new_vertex_weighted(&mut self, _: Self::VertexWeight) -> Result<Self::Vertex, ()>
	{
		let new_id = self.new_count;
		self.new_count += 1;
		Ok(ProxyVertex::New(new_id))
	}
}

impl<C: Ensure> RemoveVertex for VertexProxyGraph<C>
{
	fn remove_vertex(&mut self, v: &Self::Vertex) -> Result<Self::VertexWeight, ()>
	{
		if self.contains_vertex(v)
		{
			match v
			{
				ProxyVertex::New(_) =>
				{
					self.new_count -= 1;
					Ok(())
				},
				ProxyVertex::Underlying(v) =>
				{
					self.removed.push(*v);
					Ok(())
				},
			}
		}
		else
		{
			Err(())
		}
	}
}

base_graph! {
	use<C> VertexProxyGraph<C>
	where C: Ensure,
}

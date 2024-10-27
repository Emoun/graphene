use crate::core::{
	property::{NewVertex, RemoveVertex},
	Ensure, Graph,
};
use std::borrow::Borrow;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ProxyVertex<V: Copy + Eq>
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

	fn all_vertices_weighted(&self) -> impl Iterator<Item = (Self::Vertex, &Self::VertexWeight)>
	{
		self.graph
			.graph()
			.all_vertices()
			.filter(move |v| !self.removed.contains(v))
			.map(|v| (ProxyVertex::Underlying(v), &()))
			.chain((0..self.new_count).map(|v| (ProxyVertex::New(v), &())))
	}

	fn edges_between<'a: 'b, 'b>(
		&'a self,
		source: impl 'b + Borrow<Self::Vertex>,
		sink: impl 'b + Borrow<Self::Vertex>,
	) -> impl 'b + Iterator<Item = &'a Self::EdgeWeight>
	{
		match (source.borrow(), sink.borrow())
		{
			(ProxyVertex::Underlying(so), ProxyVertex::Underlying(si))
				if !(self.removed.contains(so) || self.removed.contains(si)) =>
			{
				Some((so.clone(), si.clone()))
			},
			_ => None,
		}
		.into_iter()
		.flat_map(|(so, si)| self.graph.graph().edges_between(so, si))
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
	fn remove_vertex(&mut self, v: impl Borrow<Self::Vertex>) -> Result<Self::VertexWeight, ()>
	{
		if self.contains_vertex(v.borrow())
		{
			match v.borrow()
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

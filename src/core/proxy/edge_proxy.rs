use crate::core::{
	property::{AddEdge, RemoveEdge, RemoveVertex},
	Directedness, Edge, Ensure, Graph, GraphDerefMut, GraphMut,
};
use delegate::delegate;
use std::borrow::Borrow;

/// A wrapper around a graph, that allows for addition and removal
/// of edges, without mutating the underlying graph.
///
/// This is useful when investigating the impact of an edge addition or removal
/// without having to actually add or remove the edge. E.g. if you only want to
/// remove an edge if some condition holds after the removal, but keep it
/// otherwise, then this proxy can be used to analyze the graph as if the edge
/// was removed.
///
/// This proxy can also be useful if the underlying graph doesn't implement edge
/// addition and removal trait. The proxy can then simulate how the graph would
/// look regardless.
///
/// If the underlying graph is mutable from the ensurer, then the edge proxy
/// can also be used to mutate vertices, however, this is done directly on the
/// underlying graph and not simulated as edge mutations are.
/// To also simulate vertex mutations, first wrap the underlying graph in
/// VertexProxy.
pub struct EdgeProxyGraph<C: Ensure>
{
	/// The underlying graph
	graph: C,
	/// Edges that have been added to the proxy and are not in the underlying
	/// graph.
	new: Vec<(<C::Graph as Graph>::Vertex, <C::Graph as Graph>::Vertex)>,
	/// Edges that have been removed from the underlying graph.
	removed: Vec<(<C::Graph as Graph>::Vertex, <C::Graph as Graph>::Vertex)>,
}

impl<C: Ensure> EdgeProxyGraph<C>
{
	pub fn new(underlying: C) -> Self
	{
		Self {
			graph: underlying,
			new: Vec::new(),
			removed: Vec::new(),
		}
	}
}

impl<C: Ensure> Graph for EdgeProxyGraph<C>
{
	type Directedness = <C::Graph as Graph>::Directedness;
	type EdgeWeight = ();
	type Vertex = <C::Graph as Graph>::Vertex;
	type VertexRef = <C::Graph as Graph>::VertexRef;
	type VertexWeight = <C::Graph as Graph>::VertexWeight;

	delegate! {
		to self.graph.graph() {
			fn all_vertices_weighted<'a>(
				&'a self,
			) -> Box<dyn 'a + Iterator<Item = (Self::VertexRef, &'a Self::VertexWeight)>>;
		}
	}

	fn edges_between<'a: 'b, 'b>(
		&'a self,
		source: impl 'b + Borrow<Self::Vertex>,
		sink: impl 'b + Borrow<Self::Vertex>,
	) -> Box<dyn 'b + Iterator<Item = &'a Self::EdgeWeight>>
	{
		let applicable = |so, si| {
			(source.borrow() == so && sink.borrow() == si)
				|| (!Self::Directedness::directed()
					&& (source.borrow() == si && sink.borrow() == so))
		};
		let removed_count = self
			.removed
			.iter()
			.filter(|(so, si)| applicable(so, si))
			.count();
		let added_count = self
			.new
			.iter()
			.filter(|(so, si)| applicable(so, si))
			.count();
		let underlying_count = self.graph.graph().edges_between(source, sink).count();

		Box::new((0..(underlying_count - removed_count + added_count)).map(|_| &()))
	}
}

impl<C: Ensure + GraphDerefMut> GraphMut for EdgeProxyGraph<C>
where
	C::Graph: GraphMut,
{
	delegate! {
		to self.graph.graph_mut() {
			fn all_vertices_weighted_mut<'a>(
				&'a mut self,
			) -> Box<dyn 'a + Iterator<Item = (Self::VertexRef, &'a mut Self::VertexWeight)>>;
		}
	}

	fn edges_between_mut<'a: 'b, 'b>(
		&'a mut self,
		source: impl 'b + Borrow<Self::Vertex>,
		sink: impl 'b + Borrow<Self::Vertex>,
	) -> Box<dyn 'b + Iterator<Item = &'a mut Self::EdgeWeight>>
	{
		// Safe because &mut () can't mutate anything
		Box::new(
			self.edges_between(source, sink)
				.map(|w| unsafe { &mut *((w as *const ()) as *mut ()) }),
		)
	}
}

impl<C: Ensure> AddEdge for EdgeProxyGraph<C>
{
	fn add_edge_weighted(
		&mut self,
		source: impl Borrow<Self::Vertex>,
		sink: impl Borrow<Self::Vertex>,
		_: Self::EdgeWeight,
	) -> Result<(), ()>
	{
		if self.contains_vertex(source.borrow()) && self.contains_vertex(sink.borrow())
		{
			self.new
				.push((source.borrow().clone(), sink.borrow().clone()));
			Ok(())
		}
		else
		{
			Err(())
		}
	}
}

impl<C: Ensure> RemoveEdge for EdgeProxyGraph<C>
{
	fn remove_edge_where_weight<F>(
		&mut self,
		source: impl Borrow<Self::Vertex>,
		sink: impl Borrow<Self::Vertex>,
		_: F,
	) -> Result<Self::EdgeWeight, ()>
	where
		F: Fn(&Self::EdgeWeight) -> bool,
	{
		// First try to find a valid new vertex
		let to_remove = self.new.iter().cloned().enumerate().find(|(_, e)| {
			(e.source() == *source.borrow() && e.sink() == *sink.borrow())
				|| !Self::Directedness::directed()
					&& (e.source() == *sink.borrow() && e.sink() == *source.borrow())
		});

		if let Some((idx, _)) = to_remove
		{
			self.new.remove(idx);
			Ok(())
		}
		else
		{
			// If no new vertex is valid, look through the existing ones.
			if let Some(_) = self
				.graph
				.graph()
				.edges_between(source.borrow(), sink.borrow())
				.next()
			{
				self.removed
					.push((source.borrow().clone(), sink.borrow().clone()));
				Ok(())
			}
			else
			{
				Err(())
			}
		}
	}
}

impl<C: Ensure + GraphDerefMut> RemoveVertex for EdgeProxyGraph<C>
where
	C::Graph: RemoveVertex,
{
	fn remove_vertex(&mut self, v: impl Borrow<Self::Vertex>) -> Result<Self::VertexWeight, ()>
	{
		self.new
			.retain(|e| e.source() != *v.borrow() && e.sink() != *v.borrow());
		self.graph.graph_mut().remove_vertex(v)
	}
}

base_graph! {
	use<C> EdgeProxyGraph<C>: NewVertex
	as (self.graph) : C
	where C: Ensure
}

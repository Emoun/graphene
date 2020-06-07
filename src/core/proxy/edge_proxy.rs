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
	type VertexWeight = <C::Graph as Graph>::VertexWeight;

	delegate! {
		to self.graph.graph() {
			fn all_vertices_weighted<'a>(
				&'a self,
			) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, &'a Self::VertexWeight)>>;
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

	fn all_edges<'a>(
		&'a self,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>
	{
		let underlying_edges = self.graph.graph().all_edges();
		let mut rem_used = Vec::with_capacity(self.removed.len());
		rem_used.extend(self.removed.iter().map(|_| false));
		let removed = underlying_edges
			.filter(move |e| {
				if let Some((idx, _)) = self.removed.iter().enumerate().find(|(idx, rem)| {
					!rem_used[*idx]
						&& ((rem.source() == e.source() && rem.sink() == e.sink())
							|| (!Self::Directedness::directed()
								&& rem.source() == e.sink() && rem.sink() == e.source()))
				})
				{
					rem_used[idx] = true;
					false
				}
				else
				{
					true
				}
			})
			.map(|e| (e.source(), e.sink(), &()));
		Box::new(
			self.new
				.iter()
				.cloned()
				.map(|e| (e.source(), e.sink(), &()))
				.chain(removed),
		)
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
			) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, &'a mut Self::VertexWeight)>>;
		}
	}

	fn all_edges_mut<'a>(
		&'a mut self,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>
	{
		Box::new(self.all_edges().map(|(so, si, w)| {
			(so, si, {
				let pointer = w as *const ();
				let pointer_mut = pointer as *mut ();
				unsafe { &mut *pointer_mut }
			})
		}))
	}
}

impl<C: Ensure> AddEdge for EdgeProxyGraph<C>
{
	fn add_edge_weighted(
		&mut self,
		source: &Self::Vertex,
		sink: &Self::Vertex,
		_: Self::EdgeWeight,
	) -> Result<(), ()>
	{
		if self.contains_vertex(source) && self.contains_vertex(sink)
		{
			self.new.push((*source, *sink));
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
		source: &Self::Vertex,
		sink: &Self::Vertex,
		_: F,
	) -> Result<Self::EdgeWeight, ()>
	where
		F: Fn(&Self::EdgeWeight) -> bool,
	{
		// First try to find a valid new vertex
		let to_remove = self.new.iter().cloned().enumerate().find(|(_, e)| {
			(e.source() == *source && e.sink() == *sink)
				|| !Self::Directedness::directed() && (e.source() == *sink && e.sink() == *source)
		});

		if let Some((idx, _)) = to_remove
		{
			self.new.remove(idx);
			Ok(())
		}
		else
		{
			// If no new vertex is valid, look through the existing ones.
			if let Some(_) = self.graph.graph().edges_between(source, sink).next()
			{
				self.removed.push((*source, *sink));
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
	fn remove_vertex(&mut self, v: &Self::Vertex) -> Result<Self::VertexWeight, ()>
	{
		self.new.retain(|e| e.source() != *v && e.sink() != *v);
		self.graph.graph_mut().remove_vertex(v)
	}
}

base_graph! {
	use<C> EdgeProxyGraph<C>: NewVertex
	as (self.graph) : C
	where C: Ensure
}

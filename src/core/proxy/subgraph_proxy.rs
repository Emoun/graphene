use crate::core::{
	property::{AddEdge, NewVertex, RemoveEdge, RemoveVertex, Subgraph},
	Edge, Ensure, Graph, GraphDerefMut, GraphMut,
};
use std::borrow::Borrow;

/// A subgraph of another graph.
///
/// This proxy graph will act at if only a specific subset of the underlying
/// graph's vertices exist, filtering out all other vertices and edges incident
/// on them.
pub struct SubgraphProxy<C: Ensure>
{
	/// The underlying graph
	graph: C,
	/// Which vertices are part of this subgraph
	verts: Vec<<C::Graph as Graph>::Vertex>,
	/// Edges who's sources are in this subgraph but who's sinks aren't.
	exit_edges: Vec<(<C::Graph as Graph>::Vertex, <C::Graph as Graph>::Vertex)>,
}

impl<C: Ensure> SubgraphProxy<C>
{
	pub fn new(underlying: C) -> Self
	{
		Self {
			graph: underlying,
			verts: Vec::new(),
			exit_edges: Vec::new(),
		}
	}

	pub fn expand(&mut self, v: <C::Graph as Graph>::Vertex) -> Result<(), ()>
	{
		if self.graph.graph().contains_vertex(v)
		{
			if !self.verts.contains(&v)
			{
				self.verts.push(v);

				// Remove any exit edge that is sinked in the vertex
				while let Some(idx) = self.exit_edges.iter().position(|e| e.sink() == v)
				{
					self.exit_edges.remove(idx);
				}

				// Add any exit edge that is sourced in the vertex
				for e in self.graph.graph().edges_sourced_in(v)
				{
					if !self.verts.contains(&e.0)
					{
						self.exit_edges.push((v, e.0));
					}
				}
			}
			Ok(())
		}
		else
		{
			Err(())
		}
	}
}

impl<C: Ensure> Graph for SubgraphProxy<C>
{
	type Directedness = <C::Graph as Graph>::Directedness;
	type EdgeWeight = <C::Graph as Graph>::EdgeWeight;
	type EdgeWeightRef<'a>
		= <C::Graph as Graph>::EdgeWeightRef<'a>
	where
		Self: 'a;
	type Vertex = <C::Graph as Graph>::Vertex;
	type VertexWeight = <C::Graph as Graph>::VertexWeight;

	fn all_vertices_weighted(&self) -> impl Iterator<Item = (Self::Vertex, &Self::VertexWeight)>
	{
		self.graph
			.graph()
			.all_vertices_weighted()
			.filter(move |(v, _)| self.verts.contains(v))
	}

	fn edges_between(
		&self,
		source: impl Borrow<Self::Vertex>,
		sink: impl Borrow<Self::Vertex>,
	) -> impl Iterator<Item = Self::EdgeWeightRef<'_>>
	{
		self.graph
			.graph()
			.edges_between(source.borrow().clone(), sink.borrow().clone())
			.filter(move |_| {
				self.contains_vertex(*source.borrow()) && self.contains_vertex(*sink.borrow())
			})
	}
}

impl<C: Ensure + GraphDerefMut> GraphMut for SubgraphProxy<C>
where
	C::Graph: GraphMut,
{
	fn all_vertices_weighted_mut(
		&mut self,
	) -> impl Iterator<Item = (Self::Vertex, &mut Self::VertexWeight)>
	{
		let verts = &self.verts;
		let graph = self.graph.graph_mut();

		graph
			.all_vertices_weighted_mut()
			.filter(move |(v, _)| verts.contains(v))
	}

	fn edges_between_mut(
		&mut self,
		source: impl Borrow<Self::Vertex>,
		sink: impl Borrow<Self::Vertex>,
	) -> impl Iterator<Item = &mut Self::EdgeWeight>
	{
		let return_any =
			self.contains_vertex(*source.borrow()) && self.contains_vertex(*sink.borrow());
		self.graph
			.graph_mut()
			.edges_between_mut(source, sink)
			.filter(move |_| return_any)
	}
}

impl<C: Ensure + GraphDerefMut> AddEdge for SubgraphProxy<C>
where
	C::Graph: AddEdge,
{
	fn add_edge_weighted(
		&mut self,
		source: impl Borrow<Self::Vertex>,
		sink: impl Borrow<Self::Vertex>,
		weight: Self::EdgeWeight,
	) -> Result<(), ()>
	{
		if self.contains_vertex(source.borrow()) && self.contains_vertex(sink.borrow())
		{
			self.graph
				.graph_mut()
				.add_edge_weighted(source, sink, weight)
		}
		else
		{
			Err(())
		}
	}
}

impl<C: Ensure + GraphDerefMut> RemoveEdge for SubgraphProxy<C>
where
	C::Graph: RemoveEdge,
{
	fn remove_edge_where_weight<F>(
		&mut self,
		source: impl Borrow<Self::Vertex>,
		sink: impl Borrow<Self::Vertex>,
		f: F,
	) -> Result<Self::EdgeWeight, ()>
	where
		F: Fn(&Self::EdgeWeight) -> bool,
	{
		if self.contains_vertex(source.borrow()) && self.contains_vertex(sink.borrow())
		{
			self.graph
				.graph_mut()
				.remove_edge_where_weight(source, sink, f)
		}
		else
		{
			Err(())
		}
	}
}

impl<C: Ensure + GraphDerefMut> NewVertex for SubgraphProxy<C>
where
	C::Graph: NewVertex,
{
	fn new_vertex_weighted(&mut self, w: Self::VertexWeight) -> Result<Self::Vertex, ()>
	{
		let v = self.graph.graph_mut().new_vertex_weighted(w)?;
		self.verts.push(v);
		Ok(v)
	}
}

impl<C: Ensure + GraphDerefMut> RemoveVertex for SubgraphProxy<C>
where
	C::Graph: RemoveVertex,
{
	fn remove_vertex(&mut self, v: impl Borrow<Self::Vertex>) -> Result<Self::VertexWeight, ()>
	{
		if self.contains_vertex(v.borrow())
		{
			let w = self.graph.graph_mut().remove_vertex(v.borrow())?;
			let index = self
				.verts
				.iter()
				.position(|t| t.borrow() == v.borrow())
				.expect("Couldn't find removed vertex in subgraph");
			self.verts.remove(index);
			Ok(w)
		}
		else
		{
			Err(())
		}
	}
}

impl<C: Ensure> Subgraph for SubgraphProxy<C>
{
	fn exit_edges<'a>(&'a self) -> impl 'a + Iterator<Item = (Self::Vertex, Self::Vertex)>
	{
		self.exit_edges.iter().cloned()
	}
}

base_graph! {
	use<C> SubgraphProxy<C>
	where C: Ensure
}

use crate::core::{
	property::{AddEdge, NewVertex, RemoveEdge, RemoveVertex, Subgraph},
	Edge, Ensure, Graph, GraphDerefMut, GraphMut,
};

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
		if self.graph.graph().contains_vertex(&v)
		{
			if !self.verts.contains(&v)
			{
				self.verts.push(v);

				// Remove any exit edge that is sinked in the vertex
				while let Some(idx) = self.exit_edges.iter().position(|e| e.sink() == v)
				{
					self.exit_edges.remove(idx);
				}

				for e in self.graph.graph().edges_sourced_in(&v)
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
	type Vertex = <C::Graph as Graph>::Vertex;
	type VertexWeight = <C::Graph as Graph>::VertexWeight;

	fn all_vertices_weighted<'a>(
		&'a self,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, &'a Self::VertexWeight)>>
	{
		Box::new(
			self.graph
				.graph()
				.all_vertices_weighted()
				.filter(move |(v, _)| self.verts.contains(v)),
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
				.filter(move |(v1, v2, _)| self.verts.contains(v1) && self.verts.contains(v2)),
		)
	}
}

impl<C: Ensure + GraphDerefMut> GraphMut for SubgraphProxy<C>
where
	C::Graph: GraphMut,
{
	fn all_vertices_weighted_mut<'a>(
		&'a mut self,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, &'a mut Self::VertexWeight)>>
	{
		let verts = &self.verts;
		let graph = self.graph.graph_mut();

		Box::new(
			graph
				.all_vertices_weighted_mut()
				.filter(move |(v, _)| verts.contains(v)),
		)
	}

	fn all_edges_mut<'a>(
		&'a mut self,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>
	{
		let verts = &self.verts;
		let graph = self.graph.graph_mut();

		Box::new(
			graph
				.all_edges_mut()
				.filter(move |(v1, v2, _)| verts.contains(v1) && verts.contains(v2)),
		)
	}
}

impl<C: Ensure + GraphDerefMut> AddEdge for SubgraphProxy<C>
where
	C::Graph: AddEdge,
{
	fn add_edge_weighted(
		&mut self,
		source: &Self::Vertex,
		sink: &Self::Vertex,
		weight: Self::EdgeWeight,
	) -> Result<(), ()>
	{
		if self.contains_vertex(source) && self.contains_vertex(sink)
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
		source: &Self::Vertex,
		sink: &Self::Vertex,
		f: F,
	) -> Result<Self::EdgeWeight, ()>
	where
		F: Fn(&Self::EdgeWeight) -> bool,
	{
		if self.verts.contains(source) && self.verts.contains(sink)
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
	fn remove_vertex(&mut self, v: &Self::Vertex) -> Result<Self::VertexWeight, ()>
	{
		if self.contains_vertex(v)
		{
			let w = self.graph.graph_mut().remove_vertex(v)?;
			let index = self
				.verts
				.iter()
				.position(|&t| t == *v)
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
	fn exit_edges<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex)>>
	{
		Box::new(self.exit_edges.iter().cloned())
	}
}

base_graph! {
	use<C> SubgraphProxy<C>
	where C: Ensure
}

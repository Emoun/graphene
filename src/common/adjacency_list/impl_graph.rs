use crate::{
	common::AdjListGraph,
	core::{
		constraint::{AddEdge, NewVertex, RemoveEdge, RemoveVertex},
		BaseGraph, Directedness, Edge, EdgeWeighted, Graph, GraphMut, ImplGraph,
		ImplGraphMut,
	},
};
use crate::core::constraint::VertexCount;

impl<Vw, Ew, D> Graph for AdjListGraph<Vw, Ew, D>
where
	D: Directedness,
{
	type Directedness = D;
	type EdgeWeight = Ew;
	type Vertex = usize;
	type VertexWeight = Vw;

	fn all_vertices_weighted<'a>(
		&'a self,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, &'a Self::VertexWeight)>>
	{
		Box::new(self.vertices.iter().enumerate().map(|(v, (w, _))| (v, w)))
	}

	fn all_edges<'a>(
		&'a self,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>
	{
		Box::new(
			self.vertices
				.iter()
				.enumerate()
				.flat_map(|(source_id, (_, out))| {
					out.iter()
						.map(move |(sink_idx, e_weight)| (source_id, *sink_idx, e_weight))
				}),
		)
	}
}

impl<Vw, Ew, D> GraphMut for AdjListGraph<Vw, Ew, D>
where
	D: Directedness,
{
	fn all_vertices_weighted_mut<'a>(
		&'a mut self,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, &'a mut Self::VertexWeight)>>
	{
		Box::new(
			self.vertices
				.iter_mut()
				.enumerate()
				.map(|(v, (w, _))| (v, w)),
		)
	}

	fn all_edges_mut<'a>(
		&'a mut self,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>
	{
		Box::new(
			self.vertices
				.iter_mut()
				.enumerate()
				.flat_map(|(source_id, (_, out))| {
					out.iter_mut()
						.map(move |(sink_idx, e_weight)| (source_id, *sink_idx, e_weight))
				}),
		)
	}
}

impl<Vw, Ew, D> NewVertex for AdjListGraph<Vw, Ew, D>
where
	D: Directedness,
{
	fn new_vertex_weighted(&mut self, w: Self::VertexWeight) -> Result<Self::Vertex, ()>
	{
		let new_v = self.vertices.len();
		self.vertices.push((w, Vec::new()));
		Ok(new_v)
	}
}
impl<Vw, Ew, D> RemoveVertex for AdjListGraph<Vw, Ew, D>
where
	D: Directedness,
{
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>
	{
		if v < self.vertices.len()
		{
			while let Ok(_) = self.remove_edge_where(|e| e.sink() == v || e.source() == v)
			{
				// Drop edge
			}
			Ok(self.vertices.remove(v).0)
		}
		else
		{
			Err(())
		}
	}
}

impl<Vw, Ew, D> AddEdge for AdjListGraph<Vw, Ew, D>
where
	D: Directedness,
{
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
	where
		E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>,
	{
		let len = self.vertices.len();
		if e.source() < len && e.sink() < len
		{
			self.vertices[e.source()]
				.1
				.push((e.sink(), e.weight_owned()));
			Ok(())
		}
		else
		{
			Err(())
		}
	}
}

impl<Vw, Ew, D> RemoveEdge for AdjListGraph<Vw, Ew, D>
where
	D: Directedness,
{
	fn remove_edge_where<F>(
		&mut self,
		f: F,
	) -> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
	where
		F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool,
	{
		let found = self
			.vertices
			.iter()
			.enumerate()
			.flat_map(|(so_i, (_, edges))| {
				edges
					.iter()
					.enumerate()
					.map(move |(si_i, (si, w))| ((so_i, si_i, si, w)))
			})
			.find(|(so_i, _, si, w)| f((*so_i, **si, w)));

		if let Some((so, si_i, _, _)) = found
		{
			let (si, w) = self.vertices[so].1.remove(si_i);
			Ok((so, si, w))
		}
		else
		{
			Err(())
		}
	}
}

impl<Vw, Ew, D> VertexCount for AdjListGraph<Vw, Ew, D>
where
	D: Directedness
{
	type Count = usize;
	
	fn vertex_count(&self) -> Self::Count
	{
		self.vertices.len()
	}
}

impl<Vw, Ew, D> ImplGraph for AdjListGraph<Vw, Ew, D>
where
	D: Directedness,
{
	type Graph = Self;

	fn graph(&self) -> &Self::Graph
	{
		self
	}
}
impl<Vw, Ew, D> ImplGraphMut for AdjListGraph<Vw, Ew, D>
where
	D: Directedness,
{
	fn graph_mut(&mut self) -> &mut Self::Graph
	{
		self
	}
}
impl<Vw, Ew, D> BaseGraph for AdjListGraph<Vw, Ew, D> where D: Directedness {}

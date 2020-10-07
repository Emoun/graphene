use crate::{
	common::AdjListGraph,
	core::{
		property::{AddEdge, EdgeCount, NewVertex, RemoveEdge, RemoveVertex, VertexCount},
		Directedness, Graph, GraphMut,
	},
};
use std::borrow::Borrow;

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

	fn edges_between<'a: 'b, 'b>(
		&'a self,
		source: impl 'b + Borrow<Self::Vertex>,
		sink: impl 'b + Borrow<Self::Vertex>,
	) -> Box<dyn 'b + Iterator<Item = &'a Self::EdgeWeight>>
	{
		let source = source.borrow().clone();
		let sink = sink.borrow().clone();

		Box::new(
			self.vertices
				.get(source)
				.into_iter()
				.flat_map(move |(_, edges)| {
					edges.iter().filter_map(move |(si, w)| {
						if *si == sink
						{
							Some(w)
						}
						else
						{
							None
						}
					})
				})
				.chain(
					self.vertices
						.get(sink)
						.into_iter()
						.flat_map(move |(_, edges)| {
							edges.iter().filter_map(move |(si, w)| {
								if !Self::Directedness::directed() && *si != sink && *si == source
								{
									Some(w)
								}
								else
								{
									None
								}
							})
						}),
				),
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

	fn edges_between_mut<'a: 'b, 'b>(
		&'a mut self,
		source: impl 'b + Borrow<Self::Vertex>,
		sink: impl 'b + Borrow<Self::Vertex>,
	) -> Box<dyn 'b + Iterator<Item = &'a mut Self::EdgeWeight>>
	{
		let source = source.borrow().clone();
		let sink = sink.borrow().clone();

		Box::new(
			self.vertices
				.iter_mut()
				.enumerate()
				.filter_map(move |(so, (_, edges))| {
					if source == so
					{
						Some((false, edges))
					}
					else if !Self::Directedness::directed() && (so == sink)
					{
						Some((true, edges))
					}
					else
					{
						None
					}
				})
				.flat_map(|(sink_first, edges)| {
					edges
						.iter_mut()
						.map(move |(si, weight)| (sink_first, si, weight))
				})
				.filter_map(move |(sink_first, si, weight)| {
					if sink_first
					{
						if source == *si
						{
							return Some(weight);
						}
					}
					else if sink == *si
					{
						return Some(weight);
					}
					None
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
	fn remove_vertex(&mut self, v: impl Borrow<Self::Vertex>) -> Result<Self::VertexWeight, ()>
	{
		if *v.borrow() < self.vertices.len()
		{
			let neighbors: Vec<_> = self.vertex_neighbors(v.borrow()).collect();

			for n in neighbors
			{
				while let Ok(_) = self.remove_edge(v.borrow(), &n)
				{}
				if D::directed()
				{
					while let Ok(_) = self.remove_edge(&n, v.borrow())
					{}
				}
			}
			Ok(self.vertices.remove(*v.borrow()).0)
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
	fn add_edge_weighted(
		&mut self,
		source: impl Borrow<Self::Vertex>,
		sink: impl Borrow<Self::Vertex>,
		weight: Self::EdgeWeight,
	) -> Result<(), ()>
	{
		let len = self.vertices.len();
		if *source.borrow() < len && *sink.borrow() < len
		{
			self.vertices[*source.borrow()]
				.1
				.push((*sink.borrow(), weight));
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
				.find(|(so_i, _, si, w)| {
					((so_i == source.borrow() && *si == sink.borrow())
						|| (!Self::Directedness::directed()
							&& (so_i == sink.borrow() && *si == source.borrow())))
						&& f(w)
				});
			if let Some((so_i, si_i, _, _)) = found
			{
				let (_, w) = self.vertices[so_i].1.remove(si_i);
				Ok(w)
			}
			else
			{
				Err(())
			}
		}
		else
		{
			Err(())
		}
	}
}

impl<Vw, Ew, D> VertexCount for AdjListGraph<Vw, Ew, D>
where
	D: Directedness,
{
	type Count = usize;

	fn vertex_count(&self) -> Self::Count
	{
		self.vertices.len()
	}
}

impl<Vw, Ew, D> EdgeCount for AdjListGraph<Vw, Ew, D>
where
	D: Directedness,
{
	type Count = usize;

	fn edge_count(&self) -> Self::Count
	{
		self.vertices
			.iter()
			.fold(0, |count, (_, edges)| count + edges.len())
	}
}

base_graph! {
	use<Vw, Ew, D> AdjListGraph<Vw, Ew, D>
	where D: Directedness,
}

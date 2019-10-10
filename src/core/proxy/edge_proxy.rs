use crate::core::{Directedness, Constrainer, Graph, Edge, ImplGraphMut, AddEdge, EdgeWeighted, ImplGraph, BaseGraph};

pub struct EdgeProxyGraph<C: Constrainer>{
	/// The underlying graph
	graph: C,
	/// Edges that have been added to the proxy and are not in the underlying graph.
	new: Vec<(<C::Graph as Graph>::Vertex, <C::Graph as Graph>::Vertex)>,
	/// Edges that have been removed from the underlying graph.
	removed: Vec<(<C::Graph as Graph>::Vertex, <C::Graph as Graph>::Vertex)>,
}

impl<C: Constrainer> EdgeProxyGraph<C>
{
	pub fn new(underlying: C) -> Self
	{
		Self{ graph: underlying, new: Vec::new(), removed: Vec::new()}
	}
}

impl<C: Constrainer> Graph for EdgeProxyGraph<C>
{
	type Vertex = <C::Graph as Graph>::Vertex;
	type VertexWeight = <C::Graph as Graph>::VertexWeight;
	type EdgeWeight = ();
	type Directedness = <C::Graph as Graph>::Directedness;
	
	fn all_vertices_weighted<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
		(Self::Vertex, &'a Self::VertexWeight)>>
	{
		self.graph.graph().all_vertices_weighted()
	}
	
	fn all_edges<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
		(Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>
	{
		let underlying_edges = self.graph.graph().all_edges();
		let mut rem_used = Vec::with_capacity(self.removed.len());
		rem_used.extend(self.removed.iter().map(|_| false));
		let removed = underlying_edges.filter(move |e| {
			if let Some((idx,_)) = self.removed.iter().enumerate().find(|(idx, rem)|
				!rem_used[*idx] && (
					(rem.source() == e.source() && rem.sink() == e.sink()) ||
						(!Self::Directedness::directed() && rem.source() == e.sink() &&
							rem.sink() == e.source())
				)
			){
				rem_used[idx] = true;
				false
			} else {
				true
			}
		})
		.map(|e| (e.source(), e.sink(), &()));
		Box::new(self.new.iter().cloned().map(|e| (e.source(), e.sink(), &())).chain(removed))
	}
}

impl<C: Constrainer + ImplGraphMut> AddEdge for EdgeProxyGraph<C>
	where C::Graph: AddEdge,
{
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
		where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>
	{
		self.new.push((e.source(), e.sink()));
		Ok(())
	}
	
	fn remove_edge_where<F>(&mut self, f: F)
		-> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
		where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool
	{
		let to_remove = self.new.iter().cloned().enumerate().find(|(_, e)| f((e.source(), e.sink(), &())));
		
		if let Some((idx,e)) = to_remove {
			self.new.remove(idx);
			Ok((e.source(), e.sink(), ()))
		} else {
			let to_remove = self.all_edges().map(|e| (e.source(), e.sink()))
				.find(|e| f((e.source(), e.sink(), &())));
			if let Some(e) = to_remove {
				self.removed.push((e.source(), e.sink()));
				Ok((e.source(), e.sink(), ()))
			} else {
				Err(())
			}
		}
	}
}

impl<C: Constrainer> ImplGraph for EdgeProxyGraph<C>
{
	type Graph = Self;
	
	fn graph(&self) -> &Self::Graph {
		self
	}
}
impl<C: Constrainer> ImplGraphMut for EdgeProxyGraph<C>
{
	fn graph_mut(&mut self) -> &mut Self::Graph {
		self
	}
}
impl<C: Constrainer> BaseGraph for EdgeProxyGraph<C> {}
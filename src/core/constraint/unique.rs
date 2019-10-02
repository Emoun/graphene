use crate::core::{Graph, EdgeWeighted, Directedness, Edge, AddVertex, Constrainer, GraphMut, AddEdge, ConstrainerMut, BaseGraph};

///
/// A marker trait for graphs containing only unique edges.
///
/// An edge is unique if it is the only edge in the graph
/// connecting two vertices.
/// If the graph is directed then between two vertices v1 and v2
/// two edges are allowed: (v1,v2,_) and (v2,v1,_).
/// If the graph is undirected, there may only be one edge of either
/// (v1,v2,_) or (v1,v2,_).
/// Regardless of directedness, only one loop is allowed for each vertex,
/// i.e. only one (v,v,_).
///
///
///
pub trait Unique: Graph
{
	fn edge_between(&self, v1: Self::Vertex, v2: Self::Vertex) -> Option<&Self::EdgeWeight>
	{
		self.edges_between(v1,v2).next().map(|(_,_,w)| w)
	}
}

#[derive(Clone, Debug)]
pub struct UniqueGraph<C: Constrainer>(C);

impl<C: Constrainer> UniqueGraph<C>
{
	///
	/// Constrains the given graph.
	///
	/// The given graph must be unique. This is not checked by this function.
	///
	pub fn unchecked(c: C) -> Self
	{
		Self(c)
	}
}

impl<C: Constrainer> Constrainer for UniqueGraph<C>
{
	type Base = C::Base;
	type Constrained = C;
	
	fn constrain_single(g: Self::Constrained) -> Result<Self, ()>{
		let edges: Vec<_> = g.base().all_edges().collect();
		let mut iter = edges.iter();
		while let  Some(e) = iter.next() {
			for e2 in iter.clone() {
				if (e.source() == e2.source() && e.sink() == e2.sink()) ||
					(e.source() == e2.sink() && e.sink() == e2.source() &&
						!<<<C as Constrainer>::Base as BaseGraph>::Graph as Graph>
							::Directedness::directed())
				{
					return Err(())
				}
			}
		}
		
		Ok(UniqueGraph(g))
	}
	
	fn constrained(&self) -> &Self::Constrained {
		&self.0
	}
	
	fn unconstrain_single(self) -> Self::Constrained{
		self.0
	}
}
impl<C: ConstrainerMut> ConstrainerMut for UniqueGraph<C>
{
	type BaseMut = C::BaseMut;
	type ConstrainedMut = C;
	
	fn constrained_mut(&mut self) -> &mut Self::ConstrainedMut {
		&mut self.0
	}
}

impl<C: Constrainer> Graph for UniqueGraph<C>
{
	type Vertex = <<C::Base as BaseGraph>::Graph as Graph>::Vertex;
	type VertexWeight = <<C::Base as BaseGraph>::Graph as Graph>::VertexWeight;
	type EdgeWeight = <<C::Base as BaseGraph>::Graph as Graph>::EdgeWeight;
	type Directedness = <<C::Base as BaseGraph>::Graph as Graph>::Directedness;
	
	fn all_vertices_weighted<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
		(Self::Vertex, &'a Self::VertexWeight)>>
	{
		self.base().all_vertices_weighted()
	}
	
	fn all_edges<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
		(Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>
	{
		self.base().all_edges()
	}
}

impl<C: ConstrainerMut>  GraphMut for UniqueGraph<C>
	where <C::Base as BaseGraph>::Graph: GraphMut
{
	fn all_vertices_weighted_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
		(Self::Vertex, &'a mut Self::VertexWeight)>>
	{
		self.base_mut().all_vertices_weighted_mut()
	}
	
	fn all_edges_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
		(Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>
	{
		self.base_mut().all_edges_mut()
	}
}

impl<C: ConstrainerMut> AddVertex for UniqueGraph<C>
	where <C::Base as BaseGraph>::Graph: AddVertex
{
	fn new_vertex_weighted(&mut self, w: Self::VertexWeight)
		-> Result<Self::Vertex, ()>
	{
		self.base_mut().new_vertex_weighted(w)
	}
	
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>
	{
		self.base_mut().remove_vertex(v)
	}
}

impl<C: ConstrainerMut> AddEdge for UniqueGraph<C>
	where <C::Base as BaseGraph>::Graph: AddEdge
{
	fn remove_edge_where<F>(&mut self, f: F) -> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
		where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool
	{
		self.base_mut().remove_edge_where(f)
	}
	
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
		where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>
	{
		if Self::Directedness::directed() {
			if self.edges_between(e.source(), e.sink())
				.any(|edge| e.source() == edge.source() && e.sink() == edge.sink()){
				return Err(());
			}
		} else {
			if self.edges_between(e.source(), e.sink()).next().is_some() {
				return Err(());
			}
		}
		self.base_mut().add_edge_weighted(e)
	}
}

impl<C: Constrainer> Unique for UniqueGraph<C>{}

impl_constraints!{
	UniqueGraph<C>: Unique
}


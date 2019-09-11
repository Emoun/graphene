use crate::core::{Graph, Edge, trait_aliases::*, EdgeWeighted, Directedness, AutoGraph, ManualGraph, Constrainer, BaseGraph};
use delegate::delegate;
use crate::algo::DFS;

///
/// A marker trait for graphs that are connected.
///
/// A graph is connected if there is  apath from any vertex to any other vertex.
/// Graphs with one or zero vertices count as connected.
///
pub trait Connected: Graph
{}

#[derive(Clone, Debug)]
pub struct ConnectedGraph<G: Graph>(G);

impl<G:Graph> ConnectedGraph<G>
{
	///
	/// Creates a new connected graph. The given graph *must* be connected.
	/// This method does not check for this!!
	///
	pub fn new(g: G) -> Self
	{
		Self(g)
	}
}

impl<G: Graph> Graph for ConnectedGraph<G>
{
	type Vertex = G::Vertex;
	type VertexWeight = G::VertexWeight;
	type EdgeWeight = G::EdgeWeight;
	type Directedness = G::Directedness;
	
	delegate! {
		target self.0 {
	
			fn all_vertices<I: IntoFromIter<Self::Vertex>>(&self) -> I;
			
			fn vertex_weight(&self, v: Self::Vertex) -> Option<&Self::VertexWeight> ;
			
			fn vertex_weight_mut(&mut self, v: Self::Vertex) -> Option<&mut Self::VertexWeight>;
			
			fn all_edges<'a, I>(&'a self) -> I
				where I: EdgeIntoFromIter<'a, Self::Vertex, Self::EdgeWeight>;
			
			fn all_edges_mut<'a, I>(&'a mut self) -> I
				where I: EdgeIntoFromIterMut<'a, Self::Vertex, Self::EdgeWeight>;
			
			fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
				where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>;
		}
	}
	
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>
	{
		Err(())
	}
	
	fn remove_edge_where<F>(&mut self, f: F)
		-> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
		where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool
	{
		unimplemented!()
	}
}

impl<G: AutoGraph> AutoGraph for ConnectedGraph<G>
{
	delegate! {
		target self.0 {
			fn new_vertex_weighted(&mut self, w: Self::VertexWeight)
				-> Result<Self::Vertex, ()>;
		}
	}
}

impl<G: ManualGraph> ManualGraph for ConnectedGraph<G>
{
	delegate! {
		target self.0 {
			fn add_vertex_weighted(&mut self, v: Self::Vertex, w: Self::VertexWeight)
				-> Result<(), ()>;
			
			// We delegate this method because it maintains connectivity
			// The default implementation will call unimplemented methods
			fn replace_vertex(&mut self, to_replace: Self::Vertex, replacement: Self::Vertex)
					  -> Result<(),()>;
		}
	}
}

impl<G: Graph> Connected for ConnectedGraph<G>{}

impl_constraints!{
	ConnectedGraph<G>: Connected
}

impl<B, C> Constrainer for ConnectedGraph<C>
	where B: BaseGraph, C: Constrainer<BaseGraph=B>
{
	type BaseGraph = B;
	type Constrained = C;
	
	fn constrain_single(g: Self::Constrained) -> Result<Self, ()>{
		unimplemented!()
	}
	fn unconstrain_single(self) -> Self::Constrained{
		self.0
	}
}
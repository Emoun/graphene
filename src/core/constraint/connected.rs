use crate::core::{Graph, EdgeWeighted, AutoGraph, Constrainer, BaseGraph};
use delegate::delegate;

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

delegate_graph!{
	ConnectedGraph<G>
	{
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

impl<G: Graph> Connected for ConnectedGraph<G>{}

impl_constraints!{
	ConnectedGraph<G>: Connected
}

impl<B, C> Constrainer for ConnectedGraph<C>
	where B: BaseGraph, C: Constrainer<BaseGraph=B>
{
	type BaseGraph = B;
	type Constrained = C;
	
	fn constrain_single(_: Self::Constrained) -> Result<Self, ()>{
		unimplemented!()
	}
	fn unconstrain_single(self) -> Self::Constrained{
		self.0
	}
}
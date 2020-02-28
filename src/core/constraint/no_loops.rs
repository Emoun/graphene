use crate::core::{
	constraint::AddEdge, Constrainer, EdgeWeighted, Graph, GraphDeref, GraphDerefMut, GraphMut,
};
use delegate::delegate;

/// A marker trait for graphs containing no graph loops.
///
/// In graph theory, a loop is an edge that connects a vertex to itself.
/// This trait guarantees that there are no loops in the graph and that no loops
/// can be added to it.
pub trait NoLoops: Graph
{
	fn no_loops_func(&self) {}
}

pub struct NoLoopsGraph<C: Constrainer>(C);

impl<C: Constrainer> GraphDeref for NoLoopsGraph<C>
{
	type Graph = Self;

	fn graph(&self) -> &Self::Graph
	{
		self
	}
}
impl<C: Constrainer> GraphDerefMut for NoLoopsGraph<C>
{
	fn graph_mut(&mut self) -> &mut Self::Graph
	{
		self
	}
}
impl<C: Constrainer> Constrainer for NoLoopsGraph<C>
{
	type Base = C::Base;
	type Constrained = C;

	fn constrain_single(c: Self::Constrained) -> Result<Self, ()>
	{
		if c.graph()
			.all_vertices()
			.any(|v| c.graph().edges_between(v, v).next().is_some())
		{
			Err(())
		}
		else
		{
			Ok(NoLoopsGraph(c))
		}
	}

	fn unconstrain_single(self) -> Self::Constrained
	{
		self.0
	}
}

impl<C: Constrainer> Graph for NoLoopsGraph<C>
{
	type Directedness = <C::Graph as Graph>::Directedness;
	type EdgeWeight = <C::Graph as Graph>::EdgeWeight;
	type Vertex = <C::Graph as Graph>::Vertex;
	type VertexWeight = <C::Graph as Graph>::VertexWeight;

	delegate! {
		to self.0.graph() {
			fn all_vertices_weighted<'a>(
				&'a self,
			) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, &'a Self::VertexWeight)>>;

			fn all_edges<'a>(
				&'a self,
			) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>;
		}
	}
}

impl<C: Constrainer + GraphDerefMut> GraphMut for NoLoopsGraph<C>
where
	C::Graph: GraphMut,
{
	delegate! {
		to self.0.graph_mut() {
			fn all_vertices_weighted_mut<'a>(
				&'a mut self,
			) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, &'a mut Self::VertexWeight)>>;

			fn all_edges_mut<'a>(
				&'a mut self,
			) -> Box<
				dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>
			>;
		}
	}
}

impl<C: Constrainer + GraphDerefMut> AddEdge for NoLoopsGraph<C>
where
	C::Graph: AddEdge,
{
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
	where
		E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>,
	{
		if e.source() == e.sink()
		{
			Err(())
		}
		else
		{
			self.0.graph_mut().add_edge_weighted(e)
		}
	}
}

impl<C: Constrainer> NoLoops for NoLoopsGraph<C> {}

impl_constraints! {
	NoLoopsGraph<C>: NoLoops, AddEdge
}

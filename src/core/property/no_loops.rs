use crate::core::{property::AddEdge, EdgeWeighted, Graph, GraphDerefMut, Insure};

/// A marker trait for graphs containing no graph loops.
///
/// In graph theory, a loop is an edge that connects a vertex to itself.
/// This trait guarantees that there are no loops in the graph and that no loops
/// can be added to it.
pub trait NoLoops: Graph
{
	fn no_loops_func(&self) {}
}

pub struct NoLoopsGraph<C: Insure>(C);

impl<C: Insure> Insure for NoLoopsGraph<C>
{
	fn insure_unvalidated(c: Self::Insured) -> Self
	{
		Self(c)
	}

	fn validate(c: &Self::Insured) -> bool
	{
		c.graph()
			.all_vertices()
			.all(|v| c.graph().edges_between(v, v).next().is_none())
	}
}

impl<C: Insure + GraphDerefMut> AddEdge for NoLoopsGraph<C>
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

impl<C: Insure> NoLoops for NoLoopsGraph<C> {}

impl_insurer! {
	NoLoopsGraph<C>: Insure, NoLoops, AddEdge
	for <C> as (self.0)
}

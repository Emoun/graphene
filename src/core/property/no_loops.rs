use crate::core::{property::AddEdge, EdgeWeighted, Ensure, Graph, GraphDerefMut};

/// A marker trait for graphs containing no graph loops.
///
/// In graph theory, a loop is an edge that connects a vertex to itself.
/// This trait guarantees that there are no loops in the graph and that no loops
/// can be added to it.
pub trait NoLoops: Graph
{
	fn no_loops_func(&self) {}
}

pub struct NoLoopsGraph<C: Ensure>(C);

impl<C: Ensure> Ensure for NoLoopsGraph<C>
{
	fn ensure_unvalidated(c: Self::Ensured, _: ()) -> Self
	{
		Self(c)
	}

	fn validate(c: &Self::Ensured, _: &()) -> bool
	{
		c.graph()
			.all_vertices()
			.all(|v| c.graph().edges_between(v, v).next().is_none())
	}
}

impl<C: Ensure + GraphDerefMut> AddEdge for NoLoopsGraph<C>
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

impl<C: Ensure> NoLoops for NoLoopsGraph<C> {}

impl_ensurer! {
	use<C> NoLoopsGraph<C>: Ensure, NoLoops, AddEdge
	as (self.0) : C
}

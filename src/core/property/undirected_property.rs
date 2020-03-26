use crate::core::{Directedness, Graph, Insure, Undirected};
use delegate::delegate;

/// A marker trait for graphs who's edges are undirected.
pub trait UndirectedConstraint: Graph
{
}

#[derive(Clone, Debug)]
pub struct UndirectedGraph<C: Insure>(C);

impl<C: Insure> Insure for UndirectedGraph<C>
{
	fn insure_unvalidated(c: Self::Insured) -> Self
	{
		Self(c)
	}

	fn validate(_: &Self::Insured) -> bool
	{
		!<<C::Graph as Graph>::Directedness as Directedness>::directed()
	}
}

impl<C: Insure> Graph for UndirectedGraph<C>
{
	type Directedness = Undirected;
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

impl<C: Insure> UndirectedConstraint for UndirectedGraph<C> {}

impl_insurer! {
	UndirectedGraph<C>: Insure, Graph, UndirectedConstraint
	for <C> as (self.0)
}

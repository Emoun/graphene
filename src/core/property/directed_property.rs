use crate::core::{Directed, Directedness, Graph, Insure};
use delegate::delegate;

/// A marker trait for graphs who's edges are directed.
pub trait DirectedConstraint: Graph
{
}

#[derive(Clone, Debug)]
pub struct DirectedGraph<C: Insure>(C);

impl<C: Insure> Insure for DirectedGraph<C>
{
	fn insure_unvalidated(c: Self::Insured) -> Self
	{
		Self(c)
	}

	fn validate(_: &Self::Insured) -> bool
	{
		<<C::Graph as Graph>::Directedness as Directedness>::directed()
	}
}

impl<C: Insure> Graph for DirectedGraph<C>
{
	type Directedness = Directed;
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

impl<C: Insure> DirectedConstraint for DirectedGraph<C> {}

impl_insurer! {
	DirectedGraph<C>: Graph, DirectedConstraint
}

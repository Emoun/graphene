use crate::core::{
	property::{Connected, Unilateral, Weak},
	Directed, Ensure, Graph, Undirected,
};
use delegate::delegate;
use std::borrow::Borrow;

/// A proxy that acts as an undirected version of the underlying directed graph
pub struct UndirectedProxy<C: Ensure>(C)
where
	C::Graph: Graph<Directedness = Directed>;

impl<C: Ensure> UndirectedProxy<C>
where
	C::Graph: Graph<Directedness = Directed>,
{
	pub fn new(underlying: C) -> Self
	{
		Self(underlying)
	}
}

impl<C: Ensure> Graph for UndirectedProxy<C>
where
	C::Graph: Graph<Directedness = Directed>,
{
	type Directedness = Undirected;
	type EdgeWeight = <C::Graph as Graph>::EdgeWeight;
	type EdgeWeightRef<'a>
		= <C::Graph as Graph>::EdgeWeightRef<'a>
	where
		Self: 'a;
	type Vertex = <C::Graph as Graph>::Vertex;
	type VertexWeight = <C::Graph as Graph>::VertexWeight;

	delegate! {
		to self.0.graph() {
			fn all_vertices_weighted(
				&self,
			) -> impl Iterator<Item = (Self::Vertex, &Self::VertexWeight)>;
		}
	}

	fn edges_between(
		&self,
		source: impl Borrow<Self::Vertex>,
		sink: impl Borrow<Self::Vertex>,
	) -> impl Iterator<Item = Self::EdgeWeightRef<'_>>
	{
		self.0
			.graph()
			.edges_between(source.borrow().clone(), sink.borrow().clone())
			.chain(
				self.0
					.graph()
					.edges_between(sink.borrow().clone(), source.borrow().clone())
					.filter(move |_| source.borrow() != sink.borrow()),
			)
	}
}

impl<C: Ensure> Connected for UndirectedProxy<C> where C::Graph: Unilateral<Directedness = Directed> {}
impl<C: Ensure> Unilateral for UndirectedProxy<C> where C::Graph: Weak<Directedness = Directed> {}

base_graph! {
	use<C> UndirectedProxy<C>: GraphMut, NewVertex, RemoveVertex, NoLoops, Reflexive, Subgraph, Weak
	as (self.0): C
	where
		C: Ensure,
		C::Graph: Graph<Directedness = Directed>
}

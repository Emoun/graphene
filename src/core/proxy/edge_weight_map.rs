use crate::core::{Ensure, Graph, Release};
use delegate::delegate;
use std::borrow::Borrow;

/// Wraps a graph, mapping its edge weights to `Ew`.
///
/// Useful for when a graph with specific edge weights are needed, but the graph
/// you have does not have the correct edge weight type. E.g., using this
/// struct, whatever edge weights a graph has can be mapped to integers and
/// provided to
/// [DijkstraShortestPaths](../../algo/struct.DijkstraShortestPaths.html), which
/// requires unsigned integers as weights.
#[derive(Clone, Debug)]
pub struct EdgeWeightMap<C: Ensure, Ew>(
	C,
	fn(
		<C::Graph as Graph>::Vertex,
		<C::Graph as Graph>::Vertex,
		&<C::Graph as Graph>::EdgeWeight,
	) -> Ew,
);

impl<C: Ensure, Ew> EdgeWeightMap<C, Ew>
{
	pub fn new(
		c: <Self as Release>::Ensured,
		map: fn(
			<C::Graph as Graph>::Vertex,
			<C::Graph as Graph>::Vertex,
			&<C::Graph as Graph>::EdgeWeight,
		) -> Ew,
	) -> Self
	{
		Self(c, map)
	}
}

impl<C: Ensure, Ew> Ensure for EdgeWeightMap<C, Ew>
{
	fn ensure_unvalidated(
		c: Self::Ensured,
		map: fn(
			<C::Graph as Graph>::Vertex,
			<C::Graph as Graph>::Vertex,
			&<C::Graph as Graph>::EdgeWeight,
		) -> Ew,
	) -> Self
	{
		Self(c, map)
	}

	fn validate(
		_: &Self::Ensured,
		_: &fn(
			<C::Graph as Graph>::Vertex,
			<C::Graph as Graph>::Vertex,
			&<C::Graph as Graph>::EdgeWeight,
		) -> Ew,
	) -> bool
	{
		true
	}
}

impl<C: Ensure, Ew> Graph for EdgeWeightMap<C, Ew>
{
	type Directedness = <C::Graph as Graph>::Directedness;
	type EdgeWeight = Ew;
	type EdgeWeightRef<'a>
		= Ew
	where
		Self: 'a;
	type Vertex = <C::Graph as Graph>::Vertex;
	type VertexWeight = <C::Graph as Graph>::VertexWeight;

	delegate! {
		to self.0.graph() {
			fn all_vertices_weighted(&self)
				-> impl Iterator<Item=(Self::Vertex, &Self::VertexWeight)>;
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
			.edges_between(*source.borrow(), *sink.borrow())
			.map(move |e| (self.1)(*source.borrow(), *sink.borrow(), e.borrow()))
	}
}

impl_ensurer! {
	use<C,Ew> EdgeWeightMap<C, Ew>: Ensure, Graph, GraphMut, Reflexive, AddEdge, RemoveEdge
	as (self.0) : C
	as (self.1) : fn(
		<C::Graph as Graph>::Vertex,
		<C::Graph as Graph>::Vertex,
		&<C::Graph as Graph>::EdgeWeight,
	) -> Ew
}

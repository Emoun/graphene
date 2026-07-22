//! Tests for [`MaybeOwned`] and, in particular, recovering a graph-lifetime
//! reference to an edge weight via
//! [`into_borrowed`](graphene::core::MaybeOwned::into_borrowed).

use graphene::{
	common::AdjListGraph,
	core::{
		property::{AddEdge, NewVertex},
		proxy::EdgeWeightMap,
		Directed, Graph, MaybeOwned,
	},
};

/// Recovers a reference to an edge weight that lives as long as the graph
/// borrow `'a`, going through the edge-accessing method
/// [`Graph::edges_between`].
///
/// This is first and foremost a *compile-time* assertion. The `EdgeWeightRef`
/// yielded by `edges_between`, and the iterator producing it, are both dropped
/// at the end of the inner block, yet `recovered` escapes that block and is
/// returned as `&'a _`. If `into_borrowed` handed back a reference tied to the
/// `EdgeWeightRef` temporary rather than to the graph, this function would fail
/// to compile.
fn recover_graph_lifetime<'a, G>(
	graph: &'a G,
	source: G::Vertex,
	sink: G::Vertex,
) -> &'a G::EdgeWeight
where
	G: Graph,
{
	let mut edges = graph.edges_between(source, sink);
	edges.next().unwrap().into_borrowed().unwrap()
}

/// A graph that borrows its edge weights (`EdgeWeightRef = &'a W`) must let
/// `into_borrowed` return a reference carrying the graph's lifetime, so that
/// callers can hold onto it after the `EdgeWeightRef` is gone.
#[test]
fn into_borrowed_recovers_graph_lifetime()
{
	let mut graph = AdjListGraph::<(), i32, Directed>::new();
	let v0 = graph.new_vertex_weighted(()).unwrap();
	let v1 = graph.new_vertex_weighted(()).unwrap();
	graph.add_edge_weighted(v0, v1, 42).unwrap();

	let recovered = recover_graph_lifetime(&graph, v0, v1);

	assert_eq!(recovered, &42);
}

/// The counterpart contract: a graph that computes its edge weights on the fly
/// (here [`EdgeWeightMap`], whose `EdgeWeightRef` is `Owned<_>`) has nothing in
/// the graph to borrow, so `into_borrowed` must report that by returning `None`
/// while still dereferencing to the computed weight.
#[test]
fn into_borrowed_is_none_for_owned_weights()
{
	let mut graph = AdjListGraph::<(), i32, Directed>::new();
	let v0 = graph.new_vertex_weighted(()).unwrap();
	let v1 = graph.new_vertex_weighted(()).unwrap();
	graph.add_edge_weighted(v0, v1, 42).unwrap();

	let mapped = EdgeWeightMap::new(&graph, |_, _, w| *w * 2);
	let weight_ref = mapped.edges_between(v0, v1).next().unwrap();

	assert_eq!(*weight_ref, 84);
	assert_eq!(weight_ref.into_borrowed(), None);
}

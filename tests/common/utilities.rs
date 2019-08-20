
use mock_graphs::*;
use graphene::core::{
	Graph, ManualGraph,
	trait_aliases::*,
};

///
/// Initialises a graph based on the given mock graph and passes it to
/// the given function, returning the value the function returns, except
/// if the graph initialization fails, in which case false is always returned
/// (without running the function)
///
pub fn graph_init<G,F>(g: &mut G, mock: &MockGraph, holds: F) -> bool
	where
		G: ManualGraph + Graph<
			Vertex=MockVertex,
			VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight,
		>,
		F: FnOnce(&mut G) -> bool
{
	mock.all_vertices::<Vec<_>>().into_iter().for_each( |v|
		g.add_vertex_weighted(v, mock.vertex_weight(v).unwrap().clone()).unwrap()
	);
	
	mock.all_edges::<Vec<_>>().into_iter().for_each(|(so,si,w)|
		g.add_edge_weighted((so,si,w.clone())).unwrap()
	);
	
	holds(g)
}

///
/// Initialises a graph based on the given mock graph,
/// an appropriate edge is added to it (see `add_appropriate_edge()`),
/// after which the graph and the added edge are passed to the given function
/// who's return value is then returned by this function, except if the graph
/// initialization or the edge addition fails, in which case either false is returned or it panics.
///
pub fn graph_init_and_add_edge<G,F>(g: &mut G, mock: &MockGraph, source_i_cand: usize,
									   sink_i_cand:usize, weight: MockEdgeWeight,
									 holds: F)
	-> bool
	where
		G: ManualGraph + Graph<
			Vertex=MockVertex,
			VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight,
		>,
		F: FnOnce(&mut G, (MockVertex, MockVertex, MockEdgeWeight)) -> bool
{
	graph_init(g, mock , |g| {
		let edge = add_appropriate_edge(g, mock, source_i_cand, sink_i_cand, weight);
		holds(g, edge)
	})
}

///
/// Adds an appropriate edge (I.e. incident on valid vertices) to the graph returning the
/// edge added.
///
/// Assumes the graph is equals to the given description.
///
/// #Panics:
/// If the graph and description are not equal causing the addition of an invalid edge.
///
pub fn add_appropriate_edge<G>(g: &mut G, mock: &MockGraph, source_i_cand: usize,
								  sink_i_cand:usize, weight: MockEdgeWeight,)
   -> (MockVertex,MockVertex,MockEdgeWeight)
	where
		G: ManualGraph + Graph<
			Vertex=MockVertex,
			VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight,
		>,
{
	let source_v = utilities::appropriate_vertex_value_from_index(mock, source_i_cand);
	let sink_v = utilities::appropriate_vertex_value_from_index(mock,sink_i_cand);
	let added_edge = (source_v, sink_v, weight);
	g.add_edge_weighted(added_edge.clone()).unwrap();
	added_edge
}

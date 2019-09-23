use quickcheck::{Arbitrary, Gen};
use crate::mock_graph::{MockVertex, MockT, MockGraph, MockVertexWeight, MockEdgeWeight};
use graphene::core::{Directedness, Graph, AutoGraph};
use rand::Rng;
use crate::mock_graph::arbitrary::max_vertex_count;

impl Arbitrary for MockVertex
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self
	{
		Self{value: usize::arbitrary(g)}
	}
	
	fn shrink(&self) -> Box<dyn Iterator<Item = Self>>
	{
		Box::new(self.value.shrink().map(|v| Self{value: v}))
	}
}

impl Arbitrary for MockT
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self
	{
		Self{value: u32::arbitrary(g)}
	}
	
	fn shrink(&self) -> Box<dyn Iterator<Item = Self>>
	{
		Box::new(self.value.shrink().map(|v| Self{value: v}))
	}
}

impl<D> Arbitrary for MockGraph<D>
	where D: Directedness + Clone + Send + 'static
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		
		// Set the maximum amount of vertices and edges
		let v_max = max_vertex_count(g);
		let mut graph = MockGraph::empty();
		
		//Decide the amount of vertices
		let vertex_count = g.gen_range(0, v_max);
		
		/* If the amount of vertices is 0, no edges can be created.
		 */
		if vertex_count > 0 {
			// Create vertices
			for _ in 0..vertex_count {
				graph.new_vertex_weighted(MockVertexWeight::arbitrary(g)).unwrap();
			}
			let vertices = graph.all_vertices().collect::<Vec<_>>();
			
			//Decide the amount of edges
			let edge_count = g.gen_range(0, v_max);
			
			/* Create edges
			 */
			//For each edge
			for _ in 0..edge_count {
				/* Create a valid edge
				 */
				let t_source = vertices[g.gen_range(0, vertex_count)];
				let t_sink = vertices[g.gen_range(0, vertex_count)];
				let t_weight = MockEdgeWeight::arbitrary(g);
				graph.add_edge_weighted((t_source, t_sink, t_weight)).unwrap();
			}
		}
		graph
	}
	
	fn shrink(&self) -> Box<dyn Iterator<Item=Self>> {
		
		/* Base case
		 */
		if self.vertices.len() == 0 {
			return Box::new(std::iter::empty());
		}
		
		let mut result = Vec::new();
		
		/* Shrink by shrinking vertex weight
		 */
		self.vertices.iter()
			//Get all possible shrinkages
			.flat_map(|(v,weight)| weight.shrink().map(move|shrunk| (v,shrunk)))
			//For each shrunk weight,
			//create a new graph where the vertex has that weight
			.for_each(|(v, shrunk_weight)|{
				let mut new_graph = self.clone();
				new_graph.vertices.insert(*v, shrunk_weight);
				result.push(new_graph);
			});
		
		/* Shrink by shrinking edge weight
		 */
		//For each edge
		self.all_edges().for_each(|(source,sink,ref weight)|{
			let shrunk_weights = weight.shrink();
			
			shrunk_weights.for_each( |s_w| {
				let mut shrunk_graph = self.clone();
				shrunk_graph.remove_edge_where_weight((source, sink),
													  |ref w| w.value == weight.value
				).unwrap();
				shrunk_graph.add_edge_weighted((source, sink, s_w)).unwrap();
				result.push(shrunk_graph);
			});
		});
		
		/* Shrink by removing an edge
		 */
		//For each edge
		for e in self.all_edges(){
			/* Add to the result a copy of the graph
			 * without the edge
			 */
			let mut shrunk_graph = self.clone();
			shrunk_graph.remove_edge(e).unwrap();
			result.push(shrunk_graph);
		}
		
		/* Shrink by removing a vertex that has no edges.
		 * We don't remove any edges in this step (to be able to remove a vertex)
		 * because we are already shrinking by removing edges, which means, there
		 * should be a set of edge shrinkages that result in a removable vertex.
		 */
		for v in self.all_vertices(){
			if self.edges_incident_on(v).next().is_none(){
				let mut shrunk_graph = self.clone();
				shrunk_graph.remove_vertex(v).unwrap();
				result.push(shrunk_graph);
			}
		}
		
		Box::new(result.into_iter())
	}
}
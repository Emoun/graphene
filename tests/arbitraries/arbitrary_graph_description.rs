
use super::*;

///
/// A description of a graph with vertex values of type `V` and edge
/// weights of type `W`.
///
/// There are `.values.len()` vertices in the graph. Each vertex is identified by an index
/// (`0...values.len()`) where vertex `0` has value `.values[0]`, `1` has value `.values[1]`, etc.
/// Vertices have unique values in the graph.
///
/// There are `.edges.len()` edges in the graph. Each edge `e` goes from vertex `e.0` (using the
/// vertices indeces) to vertex `e.1` and has weight `e.2`. Edges are unordered and can be duplicated.
///
///
#[derive(Clone,Debug)]
pub struct GraphDescription<V,W>
where
	V: ArbVertex,
	W: ArbWeight,
{
	pub values: Vec<V>,
	pub edges: Vec<(usize,usize,W)>,
}

impl<V,W> Arbitrary for GraphDescription<V,W>
where
	V: ArbVertex,
	W: ArbWeight,
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		
		// Set the maximum amount of vertices and edges
		let COUNT = 10;
		let mut vertex_values = Vec::new();
		let mut edges = Vec::new();
		
		//Decide the amount of vertices
		let vertex_count = g.gen_range(0,COUNT);
		
		/* If the amount of vertices is 0, no edges can be created.
		 */
		if vertex_count > 0 {
			//Decide the amount of edges
			let edge_count = g.gen_range(0, COUNT);
			
			/* Create vertex values ensuring that
			 * each vertex has a unique value
			 */
			let mut next_value = V::arbitrary(g);
			//For each vertex
			for _ in 0..vertex_count {
				// As long as the created value is already used by another vertex
				while vertex_values.contains(&next_value) {
					// Create a new value
					next_value = V::arbitrary(g);
				}
				vertex_values.push(next_value);
			}
			
			/* Create edges
			 */
			//For each edge
			for _ in 0..edge_count {
				/* Create a valid edge
				 */
				let t_source = usize::arbitrary(g) % vertex_count;
				let t_sink = usize::arbitrary(g) % vertex_count;
				let t_weight = W::arbitrary(g);
				edges.push((t_source, t_sink, t_weight))
			}
		}
		GraphDescription { values: vertex_values, edges}
	}
	
	fn shrink(&self) -> Box<Iterator<Item=Self>> {
		
		/* Base case
		 */
		if self.values.len() == 0 {
			return Box::new(Vec::new().into_iter());
		}
		
		let mut result = Vec::new();
		
		/* Shrink by shrinking vertices
		 */
		let mut new_values;
		//For each vertex
		for (i,&val) in self.values.iter().enumerate(){
			//Get all possible shrinkages
			let shrunk_values = val.shrink();
			//For each shrunk value
			for s in shrunk_values{
				//If no other vertex has that value
				if !self.values.contains(&s) {
					/* Add to the result a desc copy where that vertex
					 * has been shrunk to the value.
					 */
					new_values = self.values.clone();
					new_values[i] = s;
					result.push(GraphDescription {
						values: new_values, edges: self.edges.clone()});
				}
			}
		}
		
		/* Shrink by shrinking edge weights
		 */
		let mut new_edges;
		//For each edge
		for (i, &(so,si,we)) in self.edges.iter().enumerate() {
			//Get all possible shrinkages
			let shrunk_weights = we.shrink();
			//For each shrunk weight
			for s_w in shrunk_weights{
				/* Add to the result a desc copy where that
				 * edge weight has been shrunk to the value.
				 */
				new_edges = self.edges.clone();
				new_edges[i] = (so,si,s_w);
				result.push(GraphDescription {
					values: self.values.clone(), edges: new_edges});
			}
		}
		
		/* Shrink by removing an edge
		 */
		//For each edge
		for (i, _ ) in self.edges.iter().enumerate(){
			/* Add to the result a copy of the desc
			 * without the edge
			 */
			new_edges = self.edges.clone();
			new_edges.remove(i);
			result.push(GraphDescription {
				values: self.values.clone(),
				edges: new_edges });
		}
		
		/* Shrink by removing a vertex
		 */
		let mut t_edge;
		//For each vertex (v)
		for (i,_) in self.values.iter().enumerate(){
			// Clone the vertices
			new_values = self.values.clone();
			new_edges = Vec::new();
			
			//For all edges
			for &e in self.edges.iter(){
				/* Any edge originally connected to v is removed.
				 * Any edge connected to the last vertex is changed
				 * to connect to v. Later, v is removed and the last vertex
				 * takes its place, meaning all the edges now point to the right
				 * vertices again.
				 */
				// Any edge not connected to the vertex
				if e.0 != i && e.1 != i {
					t_edge = e;
					
					// If its source is the last vertex
					if e.0 == self.values.len()-1 {
						// Make v its source
						t_edge.0 = i;
					}
					// If its sink is the last vertex
					if e.1 == self.values.len()-1 {
						// Make v its sink
						t_edge.1 = i;
					}
					// Keep the edge in the desc
					new_edges.push(t_edge);
				}
			}
			
			//Replace v with the last vertex
			new_values.swap_remove(i);
			
			// Add to the result a copy of the desc where the above changes are in effect
			result.push(GraphDescription { values: new_values, edges: new_edges});
		}
		
		Box::new(result.into_iter())
	}
}

/*
quickcheck! {
	fn test_arbitrary_graph(Ag: GraphDescription<u32,u32>) -> bool{
		println!("Original: {:?}", Ag);
		
		for a in Ag.shrink(){
			println!("Shrunk: {:?}", a);
		}
		true
	}
}
*/
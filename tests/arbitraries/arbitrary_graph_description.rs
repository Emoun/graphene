
///
/// A description of a graph with vertex values of type `V` and edge
/// ids of type `E`.
///
/// There are `.values.len()` vertices in the graph. Each vertex is identified by an index
/// (`0...values.len()`) where vertex `0` has value `.values[0]`, `1` has value `.values[1]`, etc.
/// Vertices have unique values in the graph.
///
/// There are `.edges.len()` edges in the graph. Each edge `e` goes from vertex `e.0` (using the
/// vertices indices) to vertex `e.1` and has weight `e.2`. Edges are unordered and can be duplicated.
///
///
#[derive(Clone,Debug)]
pub struct GraphDescription<V,W>
where
	V: ArbId,
	W: ArbWeight,
{
	///Values of the verices
	pub values: Vec<V>,
	/// Edges between the vertices (given by their index in `.values`
	pub edges: Vec<(usize,usize,W)>,
}


impl<V,W> GraphDescription<V,W>
	where
		V: ArbId,
		W: ArbWeight,
{
	///
	/// Returns all the edges by the value of the vertices they are incident on
	///
	pub fn edges_by_value(&self) -> Vec<(V,V,W)>
	{
		let mut edges = Vec::new();
		
		for e in &self.edges{
			let t_source = self.values[e.0];
			let t_sink = self.values[e.1];
			edges.push((t_source, t_sink, e.2));
		}
		edges
	}
}







use implementations::adjacency_list::*;
use graph::*;



impl<'a, T> BaseGraph<'a,
	T,
	Vec<T>,
	Vec<BaseEdge<T>>,
>
for AdjListGraph<T>
	where
		T: Copy + Eq
{
	fn all_vertices(&'a self) -> Vec<T> {
		let mut result = Vec::new();
		
		//For each value, output a copy
		for i in 0..self.values.len() {
			result.push(self.values[i]);
		}
		result
	}
	
	fn all_edges(&'a self) -> Vec<BaseEdge<T>> {
		let mut result = Vec::new();
		
		//For each vertex (source)
		for (source_i, ref out) in self.edges.iter().enumerate() {
			let source_value = self.values[source_i];
			//For each outgoing edge (sink)
			for &sink_i in out.iter() {
				let sink_value = self.values[sink_i];
				//Return the edge
				result.push(BaseEdge { source: source_value, sink: sink_value });
			}
		}
		result
	}
}














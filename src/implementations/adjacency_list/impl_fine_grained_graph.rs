use implementations::adjacency_list::*;
use graph::*;



impl<'a, T> FineGrainedGraph<'a,
	T,
	Vec<T>,
	Vec<BaseEdge<T>>,
	Vec<BaseEdge<T>>,
	Vec<BaseEdge<T>>,
>
for AdjListGraph<T>
	where
		T: Copy + Eq
{
	fn vertex_count(&'a self) -> usize {
		self.values.len()
	}
	
	fn edge_count(&'a self) -> usize {
		let mut sum = 0;
		//For each vertex, count the outgoing edges
		for v in self.edges.iter() {
			sum += v.len();
		}
		sum
	}
	
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
	
	fn outgoing_edges(&'a self, v: T) -> Result<Vec<BaseEdge<T>>, ()> {
		if let Some(i) = self.get_index(v){
			Ok(self.edges[i].iter().map(
				|&sink_i|
					BaseEdge{source: v,
					sink: self.get_value(sink_i).unwrap()}
			).collect())
		}else {
			Err(())
		}
		
	}
	
	fn incoming_edges(&'a self, v: T) -> Result<Vec<BaseEdge<T>>, ()> {
		unimplemented!()
	}
	
	fn edges_between(&'a self, v1: T, v2: T) -> Result<Vec<BaseEdge<T>>, ()> {
		unimplemented!()
	}
}














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
		T: Copy,
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
		unimplemented!()
	}
	
	fn outgoing_edges(&'a self, v: T) -> Result<Vec<BaseEdge<T>>, ()> {
		unimplemented!()
	}
	
	fn incoming_edges(&'a self, v: T) -> Result<Vec<BaseEdge<T>>, ()> {
		unimplemented!()
	}
	
	fn edges_between(&'a self, v1: T, v2: T) -> Result<Vec<BaseEdge<T>>, ()> {
		unimplemented!()
	}
}














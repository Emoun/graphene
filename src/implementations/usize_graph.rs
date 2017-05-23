use std::vec::Vec;
use std::result::Result;

use graph::*;

#[derive(Clone, Debug)]
pub struct UsizeEdge<'a> {
	pub source: &'a usize,
	pub sink: &'a usize,
}

impl<'a> Sourced<&'a usize> for UsizeEdge<'a> {
	fn source(&self) -> &'a usize {
		self.source
	}
}

impl<'a> Sinked<&'a usize> for UsizeEdge<'a> {
	fn sink(&self) -> &'a usize {
		&self.sink
	}
}

#[derive(Clone, Debug)]
pub struct UsizeGraph {
	edges: Vec<Vec<usize>>,
	values:Vec<usize>,
}

impl UsizeGraph {
	pub fn new(values: Vec<usize>, edges: Vec<(usize, usize)>) -> Option<UsizeGraph> {
		let mut g = UsizeGraph { edges: Vec::new(), values: values };
		
		//Validate all edges point to vertices
		for &(source, sink) in &edges {
			if source >= g.values.len() || sink >= g.values.len(){
				return None;
			}
		}
		
		//Initialise adjacency list
		for _ in 0..g.values.len(){
			g.edges.push(Vec::new());
		}
		
		//Insert each edge
		for &(source, sink) in &edges {
			g.edges[source].push(sink);
		}
		Some(g)
	}
	
	#[allow(dead_code)]
	fn valid_ref(&self, v: &usize) -> Result<(),()>{
		let borrowed_values_enum = (&self.values).iter().enumerate();
		let mut valid = false;
		for (_, value_b) in borrowed_values_enum {
			if value_b == v {
				valid = true;
			}
		}
		if !valid {
			Err(())
		}else{
			Ok(())
		}
	}
	
	fn find_indices(&self, refs: Vec<&usize>) -> Vec<Result<usize,()>>{
		let mut result = Vec::new();
		result.reserve(refs.len());
		
		//For each ref
		'outer:
		for (ref_i, &ref_) in refs.iter().enumerate(){
			//Go through all values
			let borrowed_values_enum = (&self.values).iter().enumerate();
			for (value_i, value_ref ) in borrowed_values_enum{
				//If the reference points to the value
				if value_ref == ref_ {
					//Collect the index of the value
					result[ref_i] = Ok(value_i);
					continue 'outer;
				}
			}
			//The reference does not point to a value in the graph
			result[ref_i] = Err(());
		}
		result
	}
	
	fn find_refs(&self, indices: &Vec<usize>) -> Vec<Result<&usize,()>>{
		let mut result = Vec::new();
		result.reserve(indices.len());
		
		//for each index
		for i in 0..indices.len(){
			if indices[i] < self.values.len() {
				//Get its reference from the values
				result.push(Ok(&self.values[indices[i]]));
			}else{
				result.push(Err(()));
			}
		}
		return result
	}
}

impl<'a> Graph<'a> for UsizeGraph {
	type Vertex =&'a usize;
	type Edge = UsizeEdge<'a>;
	type Outgoing = UsizeEdge<'a>;
	type Incoming = UsizeEdge<'a>;
	
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
	
	fn all_vertices(&'a self) -> Vec<Self::Vertex> {
		let mut result = Vec::new();
		
		//For each value, output a reference to it
		for value_b in &self.values {
			result.push(value_b);
		}
		result
	}
	
	fn all_edges(&'a self) -> Vec<Self::Edge> {
		let mut result = Vec::new();
		
		//For each vertex
		for (i, v_out) in self.edges.iter().enumerate() {
			let source_b = &self.values[i];
			//For each outgoing edge
			for sink_b in v_out.iter() {
				//Return the edge
				result.push(UsizeEdge { source: source_b, sink: sink_b });
			}
		}
		
		result
	}
	
	fn outgoing_edges(&'a self, v: Self::Vertex) -> Result<Vec<Self::Outgoing>, ()> {
		
		//validate reference
		let v_i = self.find_indices(vec![v])[0]?;
		
		let mut result = Vec::new();
		//extract all outgoing edges
		let refs = self.find_refs(&self.edges[v_i]);
		for ref_result in refs {
			match ref_result {
				Ok(ref_) => result.push(UsizeEdge{source: v, sink: ref_}),
				_ => panic!("Impossible"),
			}
		}
		Ok(result)
	}
	
	fn incoming_edges(&'a self, v: Self::Vertex) -> Result<Vec<Self::Incoming>, ()> {
		
		//validate reference
		let v_i = self.find_indices(vec![v])[0]?;
		
		let mut result = Vec::new();
		//Go through all vertices
		for (source_i, out_v) in self.edges.iter().enumerate() {
			//Go through all outgoing edges for each
			for &sink_i in out_v {
				//If an edge points to v, collect it
				if sink_i == v_i {
					result.push(UsizeEdge{source: &self.values[source_i], sink: v});
				}
			}
		}
		Ok(result)
	}
	
	fn edges_between(&'a self, v1: Self::Vertex, v2: Self::Vertex) -> Result<Vec<Self::Edge>,()> {
		
		//Get both indices
		let indices = self.find_indices(vec![v1, v2]);
		
		//validate references
		let v1_i = indices[0]?;
		let v2_i = indices[1]?;
		
		//For each, find all edges to the other
		let mut result = Vec::new();
		
		//Go through all v1's outgoing edges
		for &e in &self.edges[v1_i]{
			if e == v2_i{
				result.push(UsizeEdge{source: v1, sink: v2});
			}
		}
		//Go through all v2's outgoing edges
		for &e in &self.edges[v2_i]{
			if e == v1_i{
				result.push(UsizeEdge{source: v2, sink: v1});
			}
		}
		Ok(result)
	}
}

impl<'a> StableGraph<
	'a,
	usize,
	UsizeEdge<'a>,
	UsizeEdge<'a>,
	UsizeEdge<'a>
> for UsizeGraph{}
/*
impl<'a> Mutating<'a, UsizeGraph> for UsizeGraph{
	type Vertex = usize;
	type Edge = UsizeEdge<'a>;
	
	fn add_vertex(mut self, v: Self::Vertex)
		-> Result<UsizeGraph,(UsizeGraph, Self::Vertex)>
	{
		//Index value
		self.values.push(v);
		self.edges.push(Vec::new());
		
		//Return a reference to it
		Ok(self)
	}
	
	fn remove_vertex(mut self, v: &'a Self::Vertex)
		-> Result	<	(UsizeGraph, Self::Vertex),
						(UsizeGraph, &'a Self::Vertex)
					>
	{
		//Get vertex index and validate
		let v_i;
		match self.find_indices(vec![v])[0] {
			Err(_) => return Err((self, v)),
			Ok(i) => v_i = i,
		}
		
		//Remove all incoming edges to v
		//Go through all vertices
		for (_, out_v) in self.edges.iter().enumerate() {
			let mut to_remove = Vec::new();
			//Go through all outgoing edges
			for (edge_i, &sink_i) in out_v.iter().enumerate() {
				//If an edge points to v, collect its index
				if sink_i == v_i {
					to_remove.push(edge_i);
				}
			}
			//Delete all collected edges
			for i in to_remove.len()..0{
				//Delete the last indices first so
				//so that the other indices aren't invalidated
				to_remove.remove(i);
			}
		}
		
		//re-point all edges pointing to last value (last)
		//to point to v
		{
			let old_last_i = self.values.len();
			//For each vertex
			for v_out in self.edges.iter_mut() {
				//any edge pointing to the old last value
				//should now point to v
				for edge_i in 0..v_out.len() {
					if v_out[edge_i] == old_last_i {
						v_out[edge_i] = v_i;
					}
				}
			}
		}
		
		//Remove v, swapping in the value of last
		let v_value = self.values[v_i];
		self.values.swap_remove(v_i);
		self.edges.swap_remove(v_i);
		
		Ok((self,v_value))
	}
	
	fn add_edge(mut self, e: Self::Edge)
		-> Result<(UsizeGraph, Self::Edge), (UsizeGraph, Self::Edge)>
	{
		//Find source and sink indices
		let indices =  self.find_indices(vec![e.source, e.sink]);
		
		//Validate
		match (indices[0], indices[1]) {
			(Ok(source_i), Ok(sink_i)) => {
				//add edge
				self.edges[source_i].push(sink_i);
				Ok((self, e))
			}
			//Invalid edge
			_ => Err((self, e))
		}
	}
	
	fn remove_edge(mut self, e: Self::Edge)
		-> Result<UsizeGraph,(UsizeGraph, Self::Edge)>
	{
		//Find source and sink indices
		let indices =  self.find_indices(vec![e.source, e.sink]);
		
		//Validate
		match (indices[0], indices[1]) {
			(Ok(source_i), Ok(sink_i)) => {
				//Find edge index in outgoing list
				match self.edges[source_i].iter().position(|&i| i == sink_i){
					//Edge present
					Some(edge_i) => {
						//Edge found, remove it
						self.edges[source_i].remove(edge_i);
						Ok(self)
					}
					//Edge not present
					_ => Err((self, e))
				}
			}
			//Invalid vertex references
			_ => Err((self, e))
		}
	}
}
*/

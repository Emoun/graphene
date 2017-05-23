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

impl<'a> FineGrainedGraph<'a,
	&'a usize,
	Vec<&'a usize>,
	Vec<UsizeEdge<'a>>,
	Vec<UsizeEdge<'a>>,
	Vec<UsizeEdge<'a>>,
> for UsizeGraph {
	
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
	
	fn all_vertices(&'a self) -> Vec<&'a usize> {
		let mut result = Vec::new();
		
		//For each value, output a reference to it
		for value_b in &self.values {
			result.push(value_b);
		}
		result
	}
	
	fn all_edges(&'a self) -> Vec<UsizeEdge<'a>> {
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
	
	fn outgoing_edges(&'a self, v: &'a usize) -> Result<Vec<UsizeEdge<'a>>, ()> {
		
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
	
	fn incoming_edges(&'a self, v: &'a usize) -> Result<Vec<UsizeEdge<'a>>, ()> {
		
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
	
	fn edges_between(&'a self, v1: &'a usize, v2: &'a usize) -> Result<Vec<UsizeEdge<'a>>,()> {
		
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

impl<'a> Graph<
	'a,
	&'a usize,
	UsizeEdge<'a>,
	UsizeEdge<'a>,
	UsizeEdge<'a>
> for UsizeGraph{}

impl<'a> StableGraph<
	'a,
	usize,
	UsizeEdge<'a>,
	UsizeEdge<'a>,
	UsizeEdge<'a>
> for UsizeGraph
{
	
	fn valid_ref(&self, v: &usize) -> bool{
		let borrowed_values_enum = (&self.values).iter().enumerate();
		let mut valid = false;
		for (_, value_b) in borrowed_values_enum {
			if value_b == v {
				valid = true;
			}
		}
		valid
	}
	
}


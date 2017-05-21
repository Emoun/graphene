use std::collections::HashSet;
use std::vec::Vec;
use std::result::Result;

use graph::*;

#[derive(Clone,Debug)]
pub struct UsizeEdge {
	pub source: usize,
	pub sink: usize,
}

impl Sourced<usize> for UsizeEdge {
	fn source(&self) -> usize {
		self.source
	}
}

impl Sinked<usize> for UsizeEdge {
	fn sink(&self) -> usize {
		self.sink
	}
}

#[derive(Clone,Debug)]
pub struct UsizeGraph {
	edges: Vec<Vec<usize>>,
}

impl UsizeGraph {
	pub fn new(n: usize) -> UsizeGraph {
		let mut g = UsizeGraph { edges: Vec::new() };
		for _ in 0..n {
			g.edges.push(Vec::new());
		}
		g
	}
	
	pub fn new_vertex(&mut self)-> usize{
		//if self.edges.len() < 9 {
			self.edges.push(Vec::new());
		//}
		self.edges.len() - 1
	}
	
	pub fn set_edge(mut self, source: usize, target: usize) -> UsizeGraph {
		self.edges[source].push(target);
		self
	}
	
	pub fn delete_edge(mut self, source: usize, sink:usize) -> UsizeGraph{
		let (index,_) = self.edges[source].iter().enumerate().find(move|v| *v.1 == sink).unwrap();
		self.edges[source].remove(index);
		self
	}
}


impl Graph for UsizeGraph {
	type Vertex = usize;
	type Edge = UsizeEdge;
	type Outgoing = UsizeEdge;
	type Incoming = UsizeEdge;
	
	fn number_of_vertices(&self) -> usize {
		self.edges.len()
	}
	
	fn number_of_edges(&self) -> usize {
		let mut sum = 0;
		for v in self.edges.iter() {
			sum += v.len();
		}
		sum
	}
	
	fn all_vertices(&self) -> HashSet<Self::Vertex> {
		let mut result = HashSet::new();
		for i in 0..self.edges.len() {
			result.insert(i);
		}
		result
	}
	
	fn all_edges(&self) -> Vec<Self::Edge> {
		let mut result = Vec::new();
		
		for i in 0..self.edges.len() {
			for j in self.edges[i].iter() {
				result.push(UsizeEdge { source: i, sink: *j });
			}
		}
		
		result
	}
	
	fn outgoing_edges(&self, v: &Self::Vertex) -> Result<Vec<Self::Outgoing>, ()> {
		if *v >= self.edges.len() {
			Err(())
		} else {
			let mut result = Vec::new();
			let ref out_edges = self.edges[*v];
			for i in 0..out_edges.len()  {
				result.push(UsizeEdge{source: *v, sink: out_edges[i]});
			}
			Ok(result)
		}
	}
	
	fn incoming_edges(&self, v: &Self::Vertex) -> Result<Vec<Self::Incoming>, ()> {
		if *v >= self.edges.len() {
			return Err(());
		}
		let mut result = Vec::new();
		for i in 0..self.edges.len() {
			if self.edges[i].contains(v) {
				result.push(UsizeEdge{source: i, sink: *v});
			}
		}
		Ok(result)
	}
	
	fn edges_between(&self, v1: &Self::Vertex, v2: &Self::Vertex) -> Result<Vec<Self::Edge>,()> {
		let mut result = Vec::new();
		
		let len = self.edges.len();
		if *v1 < len && *v2 < len {
			let ref v1_out = self.edges[*v1];
			for i in v1_out{
				if *i == *v2{
					result.push(UsizeEdge{source: *v1, sink: *v2});
				}
			}
			let ref v2_out = self.edges[*v2];
			for i in v2_out {
				if *i == *v1 {
					result.push(UsizeEdge{source: *v2, sink: *v1});
				}
			}
			Ok(result)
		}else{
			Err(())
		}
	}
}

impl Mutating<UsizeGraph> for UsizeGraph{
	type Vertex = usize;
	type Edge = UsizeEdge;
	
	fn add_vertex(mut self, v: Self::Vertex) -> (UsizeGraph, bool) {
		if v == self.edges.len() {
			self.edges.push(Vec::new());
			(self, true)
		}else{
			(self, false)
		}
	}
	
	fn remove_vertex(mut self, v: Self::Vertex) -> (UsizeGraph, bool) {
		let nr_v_old = self.edges.len();
		if v < nr_v_old {
			
			//Remove all incoming edges to v
			let v_old_in = self.incoming_edges(&v).unwrap();
			for e in v_old_in{
				self = self.remove_edge(e).0;
			}
			
			//Remove v, swapping the last edge in
			self.edges.swap_remove(v);
			
			//remap all edges from last to new v
			let nr_v_new = self.edges.len();
			for so in 0..nr_v_new {
				//For each vertice
				//any edge pointing to the old last value
				//should now point to v
				for si in 0..self.edges[so].len(){
					if self.edges[so][si] == nr_v_new {
						self.edges[so][si] = v;
					}
				}
			}
			return (self, true);
		}
		(self, false)
	}
	
	fn add_edge(mut self, e: Self::Edge) -> (UsizeGraph, bool) {
		let nr_v = self.edges.len();
		if e.source < nr_v && e.sink < nr_v {
			self.edges[e.source].push(e.sink);
			return (self,true);
		}
		(self, false)
	}
	
	fn remove_edge(mut self, e: Self::Edge) -> (UsizeGraph, bool) {
		if e.source < self.edges.len() {
			let index;
			match self.edges[e.source].iter().find(|&&v| v == e.sink) {
				Some(i) => 	index = (*i, true),
				None => 	index =  (0, false),
			}
			if index.1 {
				self.edges[e.source].remove(index.0);
				return (self, true);
			}
		}
		(self,false)
	}
}
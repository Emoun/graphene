use std::collections::HashSet;
use std::vec::Vec;
use std::result::Result;

use graph::{Graph, Sourced, Sinked, Weighted};

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

impl Weighted<()> for UsizeEdge {
	fn weight(&self) -> () {
		()
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
	
	pub fn remove_edge(mut self, source: usize, sink:usize) -> UsizeGraph{
		let (index,_) = self.edges[source].iter().enumerate().find(move|v| *v.1 == sink).unwrap();
		self.edges[source].remove(index);
		self
	}
}


impl Graph for UsizeGraph {
	type Vertex = usize;
	type Weight = ();
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
	
	fn incoming_edges(&self, v: &Self::Vertex) -> Result<Vec<Self::Incomming>, ()> {
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
	
	
	fn edges_between(&self, source: &Self::Vertex, target: &Self::Vertex) -> Result<Vec<Self::Weight>, ()> {
		let len = self.edges.len();
		if *source >= len || *target >= len {
			return Err(());
		}
		
		let mut result = Vec::new();
		if self.edges[*source].contains(target) {
			result.push(());
		}
		Ok(result)
	}
}

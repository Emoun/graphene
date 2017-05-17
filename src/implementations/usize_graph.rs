use std::collections::HashSet;
use std::vec::Vec;
use std::result::Result;

use graph::{Graph, Sourced, Sinked, Weighted};

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
	
	pub fn set_edge(&mut self, source: usize, target: usize) -> () {
		self.edges[source].push(target);
	}
}


impl Graph for UsizeGraph {
	type Vertex = usize;
	type Weight = ();
	type Edge = UsizeEdge;
	
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
	
	fn outgoing_edges(&self, v: &Self::Vertex) -> Result<Vec<Self::Vertex>, ()> {
		if *v >= self.edges.len() {
			Err(())
		} else {
			Ok(self.edges[*v].clone())
		}
	}
	
	fn incoming_edges(&self, v: &Self::Vertex) -> Result<Vec<Self::Vertex>, ()> {
		if *v >= self.edges.len() {
			return Err(());
		}
		let mut result = Vec::new();
		for i in 0..self.edges.len() {
			if self.edges[i].contains(v) {
				result.push(i);
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


use core::{WeightedGraph, EdgeWeightedGraph, Id, Edge, BaseGraph};
use common::AdjListGraph;

impl<V,W> WeightedGraph for AdjListGraph<V,W>
	where
		V: Id
{
	type Weight = W;
	type WeightRef = Self::Edge;
	
	fn add_weight(&mut self, w: Self::Weight) -> Result<Self::WeightRef,()>
	{
		self.edge_weights.push(w);
		Ok(self.edge_weights.len() - 1)
	}
	
	fn remove_weight(&mut self, w: Self::WeightRef) -> Result<Self::Weight,()>
	{
		//Check that nothing references the weight
		for source in &self.edges{
			for &(_, e_ref) in source{
				if e_ref == w {
					return Err(());
				}
			}
		}
		
		let result = self.edge_weights.remove(w);
		
		// Go through all references and reallign them
		for source in self.edges.iter_mut(){
			for e in source.iter_mut(){
				if e.1 > w {
					e.1 -= 1;
				}
			}
		}
		Ok(result)
	}
	
	fn weight_ref(&self, r: Self::WeightRef) -> Result<&Self::Weight, ()>
	{
		if r < self.edge_weights.len() {
			Ok(&self.edge_weights[r])
		}else{
			Err(())
		}
	}
}

impl<V,W> EdgeWeightedGraph for AdjListGraph<V,W>
	where
		V: Id
{
	type EdgeWeight = W;
	
	fn add_edge_weighted<E>(&mut self, e: E, w: Self::EdgeWeight)
							  -> Result<(Self::Vertex,Self::Vertex,Self::Edge), ()>
		where E: Edge<Self::Vertex,()>
	{
		let source = *e.source();
		let sink = *e.sink();
		if self.valid_adjacency(&e) {
			if let Ok(r) = self.add_weight(w) {
				if let Ok(()) = self.add_edge_copy((source, sink, r)) {
					return Ok((source, source, r));
				}else{
					self.remove_weight(r).unwrap();
				}
			}
		}
		Err(())
	}
}
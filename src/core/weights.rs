use core::{BaseGraph,Id,Edge};

pub trait WeightedGraph<W,R>: BaseGraph
	where R: Id
{
	
	///
	/// Adds a weight, returning the reference to it.
	///
	fn add_weight(&mut self, w: W) -> Result<R,()>;
	
	///
	/// Removes a weight, assuming no elements in the graph point to it.
	///
	fn remove_weight(&mut self, w: R) -> Result<W,()>;
	
	///
	/// Gets the weight referenced
	///
	fn weight_ref(&self, r: R) -> Result<&W, ()>;
	
	fn weight_of<E>(&self, e: E) -> Result<&W, ()>
		where E: Edge<Self::Vertex, R>
	{
		self.weight_ref(*e.edge())
	}
}

pub trait VertexWeightedGraph:
	WeightedGraph<
		<Self as VertexWeightedGraph>::VertexWeight,
		<Self as VertexWeightedGraph>::VertexWeightRef
	>
{
	type VertexWeight;
	/// Have to have a specific type for the reference
	/// of vertex weights because two vertices cannot have the same Id
	/// Therefore, if we use the Id as a reference, two vertices
	/// could not reference the same weight
	type VertexWeightRef: Id;
	
	fn add_vertex_weighted(&mut self, v: Self::Vertex, w: Self::VertexWeight)
		-> Result<Self::VertexWeightRef, ()>;
	
	
}

pub trait EdgeWeightedGraph:
	WeightedGraph<
		<Self as EdgeWeightedGraph>::EdgeWeight,
		<Self as BaseGraph>::Edge
	>
{
	type EdgeWeight;
	
	///
	/// Add an edge with a new weight. Returns the created edge
	///
	fn add_edge_weighted<E>(&mut self, e: E, w: Self::EdgeWeight)
							-> Result<(Self::Vertex,Self::Vertex,Self::Edge), ()>
		where E: Edge<Self::Vertex,()>;
	
}


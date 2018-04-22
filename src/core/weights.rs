use core::{BaseGraph,Id,Edge};

pub trait WeightedGraph: BaseGraph
{
	type Weight;
	type WeightRef: Id;
	
	///
	/// Adds a weight, returning the reference to it.
	///
	fn add_weight(&mut self, w: Self::Weight) -> Result<Self::WeightRef,()>;
	
	///
	/// Removes a weight, assuming no elements in the graph point to it.
	///
	fn remove_weight(&mut self, w: Self::WeightRef) -> Result<Self::WeightRef,()>;
	
	///
	/// Gets the weight referenced
	///
	fn weight(&self, r: Self::WeightRef) -> Result<&Self::Weight, ()>;
}

pub trait VertexWeightedGraph: BaseGraph
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

pub trait EdgeWeightedGraph: BaseGraph
{
	type EdgeWeight;
	
	///
	/// Add an edge with a new weight. Returns the created edge
	///
	fn add_edge_weighted<E,W>(&mut self, e: E, w: Self::EdgeWeight)
							-> Result<(Self::Vertex,Self::Vertex,Self::Edge), ()>
		where E: Edge<Self::Vertex,W>;
	
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BiWeightRef<V,E>
	where V: Id, E:Id
{
	Vertex(V),
	Edge(E)
}
pub enum BiWeight<V,E>
{
	Vertex(V),
	Edge(E)
}


pub trait BiWeightedGraph: VertexWeightedGraph + EdgeWeightedGraph +
	WeightedGraph<
		Weight=BiWeight<<Self as VertexWeightedGraph>::VertexWeight, <Self as EdgeWeightedGraph>::EdgeWeight>,
		WeightRef=BiWeightRef<<Self as VertexWeightedGraph>::VertexWeightRef, <Self as BaseGraph>::Edge>
	>
{
	fn add_weight_vertex(&mut self, w: Self::VertexWeight) -> Result<Self::VertexWeightRef, ()>;
	
	fn add_weight_edge(&mut self, w: Self::EdgeWeight) -> Result<Self::Edge, ()>;
	
	
	
}
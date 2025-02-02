use crate::core::{
	property::{NewVertex, VertexInGraph},
	Ensure, Graph, GraphDerefMut,
};

pub trait Ensured: Ensure
{
	fn ensured(self) -> EnsuredGraph<Self>
	{
		EnsuredGraph(self)
	}
}
impl<G: Ensure> Ensured for G {}

pub struct EnsuredGraph<G: Ensure>(G);

impl<G: Ensure> EnsuredGraph<G>
{
	pub fn contains_vertex(self, v: <G::Graph as Graph>::Vertex) -> Option<VertexInGraph<G>>
	{
		if self.0.graph().contains_vertex(v)
		{
			Some(VertexInGraph::ensure_unchecked(self.0, [v]))
		}
		else
		{
			None
		}
	}
}

impl<G: Ensure + GraphDerefMut> EnsuredGraph<G>
where
	G::Graph: NewVertex,
{
	pub fn new_vertex_weighted(
		mut self,
		w: <G::Graph as Graph>::VertexWeight,
	) -> Result<VertexInGraph<G>, ()>
	{
		let v = self.0.graph_mut().new_vertex_weighted(w)?;
		Ok(VertexInGraph::ensure_unchecked(self.0, [v]))
	}

	pub fn new_vertex(self) -> Result<VertexInGraph<G>, ()>
	where
		<G::Graph as Graph>::VertexWeight: Default,
	{
		self.new_vertex_weighted(Default::default())
	}
}

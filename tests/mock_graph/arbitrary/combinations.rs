
use quickcheck::{Arbitrary, Gen};
use crate::mock_graph::{MockVertex, MockEdgeWeight, MockVertexWeight};
use graphene::core::{Edge, ManualGraph, EdgeDeref, EdgeWeighted};
use rand::{ Rng };

///
/// An arbitrary graph and two vertices in it.
///
/// Note: All graphs will have at least 1 vertex, meaning this type never includes
/// the empty graph.
///
#[derive(Clone, Debug)]
pub struct ArbTwoVerticesIn<G>(pub G, pub MockVertex, pub MockVertex)
	where
		G: Arbitrary + ManualGraph<Vertex=MockVertex, VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight>;

impl<Gr> Arbitrary for ArbTwoVerticesIn<Gr>
	where
		Gr: Arbitrary + ManualGraph<Vertex=MockVertex, VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight>
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		// Create a graph with at least 1 vertex
		let graph = {
			let mut candidate_graph = Gr::arbitrary(g);
			while candidate_graph.all_vertices::<Vec<_>>().len()==0 {
				candidate_graph = Gr::arbitrary(g);
			}
			candidate_graph
		};
		
		let verts: Vec<_> = graph.all_vertices();
		let v1 = verts[g.gen_range(0, verts.len())];
		let v2 = verts[g.gen_range(0, verts.len())];
		
		ArbTwoVerticesIn(graph, v1, v2)
	}
	
	fn shrink(&self) -> Box<dyn Iterator<Item = Self>>
	{
		let mut result = Vec::new();
		
		if self.0.all_vertices::<Vec<_>>().len() > 1 {
			/*	First we shrink the graph, using only the shrunk graphs where the vertices are valid
			*/
			result.extend(
				self.0.shrink()
					.filter(|g|{
						let verts: Vec<_> = g.all_vertices();
						verts.contains(&self.1) && verts.contains(&self.2)
					})
					.map(|g| ArbTwoVerticesIn(g, self.1, self.2))
			);
			
			/*	We shrink vertices by updating the graph's vertices too
			*/
			result.extend(
				// Get all the shrunk values
				self.1.shrink()
					// Remove any that match an existing edge in the graph
					.filter(|v| !self.0.all_vertices::<Vec<_>>().contains(&v))
					// Output the shrunk value by updating it in the graph and the vertex
					.map(|v| {
						let mut clone = self.0.clone();
						clone.replace_vertex(self.1, v).unwrap();
						ArbTwoVerticesIn(clone, v, self.2)
					})
			);
			// Do the same for the second vertex
			result.extend(
				self.2.shrink()
					.filter(|v| !self.0.all_vertices::<Vec<_>>().contains(&v))
					.map(|v| {
						let mut clone = self.0.clone();
						clone.replace_vertex(self.2, v).unwrap();
						ArbTwoVerticesIn(clone, self.1, v)
					})
			);
			
			// Lastly, if the graph has only 2 vertices and no edges, remove one and
			// update the corresponding vertex to the remaining one
			if self.0.all_vertices::<Vec<_>>().len() == 2 && self.0.all_edges::<Vec<_>>().len() == 0{
				let mut verts = self.0.all_vertices::<Vec<_>>().into_iter();
				let v1 = verts.next().unwrap();
				let v2 = verts.next().unwrap();
				
				let mut clone = self.0.clone();
				clone.remove_vertex(v1);
				result.push(ArbTwoVerticesIn(clone, v2, v2));
				
				let mut clone = self.0.clone();
				clone.remove_vertex(v2);
				result.push(ArbTwoVerticesIn(clone, v1, v1));
			}
		}
		
		Box::new(result.into_iter())
	}
	
}

///
/// An arbitrary graph and a vertex in it.
///
/// Note: All graphs will have at least 1 vertex, meaning this type never includes
/// the empty graph.
///
#[derive(Clone, Debug)]
pub struct ArbVertexIn<G>(pub G, pub MockVertex)
	where
		G: Arbitrary + ManualGraph<Vertex=MockVertex, VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight>;
impl<Gr> Arbitrary for ArbVertexIn<Gr>
	where
		Gr: Arbitrary + ManualGraph<Vertex=MockVertex, VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight>
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		let arb = ArbTwoVerticesIn::arbitrary(g);
		ArbVertexIn(arb.0, arb.1)
	}
	
	fn shrink(&self) -> Box<dyn Iterator<Item=Self>> {
		Box::new(ArbTwoVerticesIn(self.0.clone(), self.1, self.1).shrink()
			.map(|ArbTwoVerticesIn(g, v, _)| ArbVertexIn(g, v)))
	}
}

///
/// An arbitrary graph and a vertex that is guaranteed to not be in it.
///
#[derive(Clone, Debug)]
pub struct ArbVertexOutside<G>(pub G, pub MockVertex)
	where
		G: Arbitrary + ManualGraph<Vertex=MockVertex, VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight>;
impl<Gr> Arbitrary for ArbVertexOutside<Gr>
	where
		Gr: Arbitrary + ManualGraph<Vertex=MockVertex, VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight>
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		let graph = Gr::arbitrary(g);
		let mut v = MockVertex::arbitrary(g);
		while graph.all_vertices::<Vec<_>>().contains(&v) {
			v = MockVertex::arbitrary(g)
		}
		
		ArbVertexOutside(graph, v)
	}
	
	fn shrink(&self) -> Box<dyn Iterator<Item=Self>> {
		let mut result = Vec::new();
		/*	First shrink the graph, keeping only the shrunk ones where the vertex
			stays invalid
		*/
		result.extend(
			self.0.shrink().filter(|g| !g.all_vertices::<Vec<_>>().contains(&self.1))
				.map(|g| ArbVertexOutside(g, self.1))
		);
		
		// We then shrink the vertex, keeping only the shrunk values
		// that are invalid in the graph
		let verts: Vec<_> = self.0.all_vertices();
		result.extend(
			self.1.shrink().filter(|v| !verts.contains(v))
				.map(|v| ArbVertexOutside(self.0.clone(), v))
		);
		
		Box::new(result.into_iter())
	}
}

///
/// An arbitrary graph and two vertices where at least one is not in the graph.
///
#[derive(Clone, Debug)]
pub struct ArbEdgeOutside<G>(pub G, pub MockVertex, pub MockVertex)
	where
		G: Arbitrary + ManualGraph<Vertex=MockVertex, VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight>;
impl<Gr> Arbitrary for ArbEdgeOutside<Gr>
	where
		Gr: Arbitrary + ManualGraph<Vertex=MockVertex, VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight>
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		let single_invalid = ArbVertexOutside::arbitrary(g);
		Self(single_invalid.0, single_invalid.1, MockVertex::arbitrary(g))
	}
	
	fn shrink(&self) -> Box<dyn Iterator<Item=Self>> {
		let mut result = Vec::new();
		/*	Shrink the graph, keeping only the shrunk graphs where the edge is still invalid.
		*/
		result.extend(
			self.0.shrink().filter(|g| {
				let verts = g.all_vertices::<Vec<_>>();
				!verts.contains(&self.1) || !verts.contains(&self.2)
			})
				.map(|g| Self(g, self.1, self.2))
		);
		
		/*	We then shrink the vertices, ensuring that at least one of them stays invalid
		*/
		result.extend(
			self.1.shrink().filter(|v| {
				let verts = self.0.all_vertices::<Vec<_>>();
				!verts.contains(&v) || !verts.contains(&self.2)
			})
				.map(|v| Self(self.0.clone(), v, self.2))
		);
		result.extend(
			self.2.shrink().filter(|v| {
				let verts = self.0.all_vertices::<Vec<_>>();
				!verts.contains(&self.1) || !verts.contains(&v)
			})
				.map(|v| Self(self.0.clone(), self.1, v))
		);
		Box::new(result.into_iter())
	}
}

///
/// An arbitrary graph with an edge that is guaranteed to be in the graph (the weight is a clone)
///
#[derive(Clone, Debug)]
pub struct ArbEdgeIn<G>(pub G, pub (MockVertex, MockVertex, MockEdgeWeight))
	where
		G: Arbitrary + ManualGraph<Vertex=MockVertex, VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight>;
impl<Gr> Arbitrary for ArbEdgeIn<Gr>
	where
		Gr: Arbitrary + ManualGraph<Vertex=MockVertex, VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight>
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		let ArbTwoVerticesIn::<Gr>(mut mock, v1, v2) = ArbTwoVerticesIn::arbitrary(g);
		let weight = MockEdgeWeight::arbitrary(g);
		mock.add_edge_weighted((v1,v2,weight.clone())).unwrap();
		Self(mock, (v1,v2,weight))
	}
	
	fn shrink(&self) -> Box<dyn Iterator<Item=Self>> {
		let mut result = Vec::new();
		/*	First, we can simply shrink the weight
		*/
		result.extend(
			(self.1).2.shrink().map(|shrunk| {
				let mut clone = self.0.clone();
				let edge = clone.all_edges_mut::<Vec<_>>().into_iter()
					.find(|e|
						e.source() == self.1.source() &&
							e.sink() == self.1.sink() &&
							e.weight() == self.1.weight_ref()
					).unwrap().2;
				*edge = shrunk.clone();
				Self(clone, ((self.1).0, (self.1).1, shrunk))
			})
		);
		
		/* We shrink each vertex in the edge
		*/
		let mut without_edge = self.0.clone();
		without_edge.remove_edge_where(|e|
			e.source() == self.1.source() &&
				e.sink() == self.1.sink() &&
				e.weight() == self.1.weight_ref()
		).unwrap();
		result.extend(
			ArbTwoVerticesIn(without_edge, (self.1).0, (self.1).1).shrink()
				.map(|ArbTwoVerticesIn(mut g, v1, v2)| {
					g.add_edge_weighted((v1,v2,(self.1).2.clone())).unwrap();
					Self(g, (v1,v2,(self.1).2.clone()))
				})
		);
		
		Box::new(result.into_iter())
	}
}


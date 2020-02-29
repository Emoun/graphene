/// Implements constraints for the given struct.
///
/// Doesn't implement the constraint given after the struct. That constraint
/// must use the original name of the trait.
///
/// Supported constraint traits:
/// Directed, Undirected, Unique, NoLoops, Reflexive, Weak, Unilateral,
/// Connected, Subgraph
#[macro_export]
macro_rules! impl_constraints {

	{
		$struct:ident<$generic_graph:ident>: $($trait:ident),*
		$(where $($bounds:tt)*)?
	} => {

		//GraphDeref
		impl_constraints!{
			@inner
			@struct_id $struct
			@generic $generic_graph
			@exclude [ $($trait)* ]
			@bounds [ $($($bounds)*)? ]
			@trait_id GraphDeref [$crate::core]
			@implement {
				type Graph = Self;

				fn graph(&self) -> &Self::Graph
				{
					self
				}
			}
		}

		//GraphDerefMut
		impl_constraints!{
			@inner
			@struct_id $struct
			@generic $generic_graph
			@exclude [ $($trait)* ]
			@bounds [ $($($bounds)*)? ]
			@trait_id GraphDerefMut [$crate::core]
			@implement {
				fn graph_mut(&mut self) -> &mut Self::Graph
				{
					self
				}
			}
		}

		//Graph
		impl_constraints!{
			@inner
			@struct_id $struct
			@generic $generic_graph
			@exclude [ $($trait)* ]
			@bounds [ $($($bounds)*)? ]
			@trait_id Graph [$crate::core]
			@implement {
				type Directedness = <C::Graph as Graph>::Directedness;
				type EdgeWeight = <C::Graph as Graph>::EdgeWeight;
				type Vertex = <C::Graph as Graph>::Vertex;
				type VertexWeight = <C::Graph as Graph>::VertexWeight;

				delegate! {
					to self.0.graph() {
						fn all_vertices_weighted<'a>(
							&'a self,
						) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, &'a Self::VertexWeight)>>;

						fn all_edges<'a>(
							&'a self,
						) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>;
					}
				}
			}
		}

		//GraphMut
		impl_constraints!{
			@inner
			@struct_id $struct
			@generic $generic_graph
			@exclude [ $($trait)* ]
			@bounds [
				$generic_graph: $crate::core::GraphDerefMut,
				$generic_graph::Graph: $crate::core::GraphMut,
				$($($bounds)*)?
			]
			@trait_id GraphMut [$crate::core]
			@implement {
				delegate! {
					to self.0.graph_mut() {
						fn all_vertices_weighted_mut<'a>(
							&'a mut self,
						) -> Box<dyn 'a + Iterator<
							Item = (Self::Vertex, &'a mut Self::VertexWeight)
						>>;

						fn all_edges_mut<'a>(
							&'a mut self,
						) -> Box<dyn 'a + Iterator<
							Item = (Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)
						>>;
					}
				}
			}
		}

		//Directed
		impl_constraints!{
			@inner
			@struct_id $struct
			@generic $generic_graph
			@exclude [ $($trait)* ]
			@bounds [
				$generic_graph: $crate::core::constraint::DirectedConstraint,
				$($($bounds)*)?
			]
			@trait_id DirectedConstraint [$crate::core::constraint]
			@implement {}
		}

		//Undirected
		impl_constraints!{
			@inner
			@struct_id $struct
			@generic $generic_graph
			@exclude [ $($trait)* ]
			@bounds [
				$generic_graph: $crate::core::constraint::UndirectedConstraint,
				$($($bounds)*)?
			]
			@trait_id UndirectedConstraint [$crate::core::constraint]
			@implement {}
		}

		// NewVertex
		impl_constraints!{
			@inner
			@struct_id $struct
			@generic $generic_graph
			@exclude [ $($trait)* ]
			@bounds [
				$generic_graph: $crate::core::GraphDerefMut + $crate::core::constraint::NewVertex,
				$generic_graph::Graph: $crate::core::constraint::NewVertex,
				$($($bounds)*)?
			]
			@trait_id NewVertex [$crate::core::constraint]
			@implement {
				delegate! {
					to self.0.graph_mut() {
						fn new_vertex_weighted(&mut self, w: Self::VertexWeight)
							-> Result<Self::Vertex, ()>;
					}
				}
			}
		}

		// RemoveVertex
		impl_constraints!{
			@inner
			@struct_id $struct
			@generic $generic_graph
			@exclude [ $($trait)* ]
			@bounds [
				$generic_graph: $crate::core::GraphDerefMut + $crate::core::constraint::RemoveVertex,
				$generic_graph::Graph: $crate::core::constraint::RemoveVertex,
				$($($bounds)*)?
			]
			@trait_id RemoveVertex [$crate::core::constraint]
			@implement {
				delegate! {
					to self.0.graph_mut() {
						fn remove_vertex(&mut self, v: Self::Vertex)
							-> Result<Self::VertexWeight, ()>;
					}
				}
			}
		}

		// AddEdge
		impl_constraints!{
			@inner
			@struct_id $struct
			@generic $generic_graph
			@exclude [ $($trait)* ]
			@bounds [
				$generic_graph: $crate::core::GraphDerefMut + $crate::core::constraint::AddEdge,
				$generic_graph::Graph: $crate::core::constraint::AddEdge,
				$($($bounds)*)?
			]
			@trait_id AddEdge [$crate::core::constraint]
			@implement {
				delegate! {
					to self.0.graph_mut() {
						fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
						where
							E: $crate::core::EdgeWeighted<Self::Vertex, Self::EdgeWeight>;
					}
				}
			}
		}

		// RemoveEdge
		impl_constraints!{
			@inner
			@struct_id $struct
			@generic $generic_graph
			@exclude [ $($trait)* ]
			@bounds [
				$generic_graph: $crate::core::GraphDerefMut + $crate::core::constraint::RemoveEdge,
				$generic_graph::Graph: $crate::core::constraint::RemoveEdge,
				$($($bounds)*)?
			]
			@trait_id RemoveEdge [$crate::core::constraint]
			@implement {
				delegate! {
					to self.0.graph_mut() {
						fn remove_edge_where<F>(
							&mut self,
							f: F,
						) -> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
						where
							F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool;
					}
				}
			}
		}

		// Unique
		impl_constraints!{
			@inner
			@struct_id $struct
			@generic $generic_graph
			@exclude [ $($trait)* ]
			@bounds [$generic_graph: $crate::core::constraint::Unique,$($($bounds)*)?]
			@trait_id Unique [$crate::core::constraint]
			@implement {}
		}

		// NoLoops
		impl_constraints!{
			@inner
			@struct_id $struct
			@generic $generic_graph
			@exclude [ $($trait)* ]
			@bounds [$generic_graph: $crate::core::constraint::NoLoops, $($($bounds)*)?]
			@trait_id NoLoops [$crate::core::constraint]
			@implement {}
		}

		// Reflexive
		impl_constraints!{
			@inner
			@struct_id $struct
			@generic $generic_graph
			@exclude [ $($trait)* ]
			@bounds [
				$generic_graph: $crate::core::constraint::Reflexive,
				$generic_graph::EdgeWeight: Default,
				<$generic_graph::Graph as Graph>::EdgeWeight: Default,
				$($($bounds)*)?
			]
			@trait_id Reflexive [$crate::core::constraint]
			@implement {}
		}

		// Weak
		impl_constraints!{
			@inner
			@struct_id $struct
			@generic $generic_graph
			@exclude [ $($trait)* ]
			@bounds [$generic_graph: $crate::core::constraint::Weak, $($($bounds)*)?]
			@trait_id Weak [$crate::core::constraint]
			@implement {}
		}

		// Unilateral
		impl_constraints!{
			@inner
			@struct_id $struct
			@generic $generic_graph
			@exclude [ $($trait)* ]
			@bounds [$generic_graph: $crate::core::constraint::Unilateral, $($($bounds)*)?]
			@trait_id Unilateral [$crate::core::constraint]
			@implement {}
		}

		// Connected
		impl_constraints!{
			@inner
			@struct_id $struct
			@generic $generic_graph
			@exclude [ $($trait)* ]
			@bounds [$generic_graph: $crate::core::constraint::Connected, $($($bounds)*)?]
			@trait_id Connected [$crate::core::constraint]
			@implement {}
		}

		// Subgraph
		impl_constraints!{
			@inner
			@struct_id $struct
			@generic $generic_graph
			@exclude [ $($trait)* ]
			@bounds [
				$generic_graph: $crate::core::constraint::Subgraph<Vertex=Self::Vertex>,
				$($($bounds)*)?
			]
			@trait_id Subgraph [$crate::core::constraint]
			@implement {
				fn exit_edges<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
					(Self::Vertex, Self::Vertex)>>
				{
					self.0.exit_edges()
				}
			}
		}

		// NonNull
		impl_constraints!{
			@inner
			@struct_id $struct
			@generic $generic_graph
			@exclude [ $($trait)* ]
			@bounds [$generic_graph: $crate::core::constraint::NonNull, $($($bounds)*)?]
			@trait_id NonNull [$crate::core::constraint]
			@implement {}
		}
	};

	{
		@inner
		@struct_id $struct:ident
		@generic $generic_graph:ident
		@exclude [ $trait:ident $($trait_rest:ident)* ]
		@bounds [$($bounds:tt)*]
		@trait_id $trait_id:ident [ $($trait_path:tt)* ]
		@implement {$($impl:tt)*}
	} => {
		tt_call::tt_if!{
			condition = [{tt_equal::tt_equal}]
			input = [{$trait $trait_id}]
			true = [{}]
			false = [{
				impl_constraints!{
					@inner
					@struct_id $struct
					@generic $generic_graph
					@exclude [ $($trait_rest)* ]
					@bounds [$($bounds)*]
					@trait_id $trait_id [ $($trait_path)* ]
					@implement {$($impl)*}
				}
			}]
		}
	};

	{
		@inner
		@struct_id $struct:ident
		@generic $generic_graph:ident
		@exclude []
		@bounds [$($bounds:tt)*]
		@trait_id $trait_id:ident [ $($trait_path:tt)* ]
		@implement {$($impl:tt)*}
	} => {
		impl<$generic_graph: $crate::core::Constrainer> $($trait_path)*::$trait_id
			for $struct<$generic_graph>
			where
				$($bounds)*
		{$($impl)*}
	};
}

/// Implements properties for the given struct.
///
/// Doesn't implement the property given after the struct. That property
/// must use the original name of the trait.
///
/// Supported property traits:
/// Directed, Undirected, Unique, NoLoops, Reflexive, Weak, Unilateral,
/// Connected, Subgraph
#[macro_export]
macro_rules! impl_insurer {

	{
		$struct:ident$(<$($generics:ident),+>)?
		$(: $($trait:ident),+)?
		for <$generic_graph:ident> as (self $($delegate:tt)+)
		$(where $($bounds:tt)*)?
	} => {
		impl_insurer!{
			@inner
			@struct_id $struct
			@generic [ $($($generics)+)? ]
			@delegate [ @generic $generic_graph ]
			@delegate_to [ $($delegate)+ ]
			@exclude [ $($($trait)+)? ]
			@bounds [ $($($bounds)*)? ]
		}
	};

	{
		$struct:ident$(<$($generics:ident),+>)?
		$(: $($trait:ident),+)?
		for $type_graph:ty as (self $($delegate:tt)+)
		$(where $($bounds:tt)*)?
	} =>{
		impl_insurer!{
			@inner
			@struct_id $struct
			@generic [ $($($generics)+)? ]
			@delegate [ @type $type_graph ]
			@delegate_to [ $($delegate)+ ]
			@exclude [ $($($trait)+)? ]
			@bounds [$($($bounds)*)?]
		}
	};

	{
		@inner
		@struct_id $struct:ident
		@generic [ $($generics:ident)* ]
		@delegate [ $(@generic $generic_graph:ident)? $(@type $type_graph:ty)? ]
		@delegate_to [ $($delegate:tt)+ ]
		@exclude [ $($trait:ident)* ]
		@bounds [$($bounds:tt)*]
	} => {

		//GraphDeref
		impl_insurer!{
			@inner
			@struct_id $struct
			@generic [ $($generics)* ]
			@delegate [ $(@generic $generic_graph)? ]
			@exclude [ $($trait)* ]
			@bounds [ $($bounds)* ]
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
		impl_insurer!{
			@inner
			@struct_id $struct
			@generic [ $($generics)* ]
			@delegate [ $(@generic $generic_graph)? ]
			@exclude [ $($trait)* ]
			@bounds [ $($bounds)* ]
			@trait_id GraphDerefMut [$crate::core]
			@implement {
				fn graph_mut(&mut self) -> &mut Self::Graph
				{
					self
				}
			}
		}

		//Release
		impl_insurer!{
			@inner
			@struct_id $struct
			@generic [ $($generics)* ]
			@delegate [ $(@generic $generic_graph)? ]
			@exclude [ $($trait)* ]
			@bounds [ $($bounds)* ]
			@trait_id Release [$crate::core]
			@implement {
				type Base = <$($generic_graph)? $($type_graph)? as $crate::core::Release>::Base ;
				type Insured = $($generic_graph)? $($type_graph)?;

				fn release(self) -> Self::Insured
				{
					self$($delegate)+
				}
			}
		}

		//Insure
		impl_insurer!{
			@inner
			@struct_id $struct
			@generic [ $($generics)* ]
			@delegate [ $(@generic $generic_graph)? ]
			@exclude [ $($trait)* ]
			@bounds [ $($bounds)* ]
			@trait_id Insure [$crate::core]
			@implement {
				fn insure_unvalidated(c: Self::Insured) -> Self
				{
					Self(c)
				}

				fn validate(_: &Self::Insured) -> bool
				{
					true
				}
			}
		}

		//Graph
		impl_insurer!{
			@inner
			@struct_id $struct
			@generic [ $($generics)* ]
			@delegate [ $(@generic $generic_graph)? ]
			@exclude [ $($trait)* ]
			@bounds [ $($bounds)* ]
			@trait_id Graph [$crate::core]
			@implement {
				type Directedness = <<$($generic_graph)? $($type_graph)? as $crate::core::GraphDeref>::Graph as $crate::core::Graph>::Directedness;
				type EdgeWeight = <<$($generic_graph)? $($type_graph)? as $crate::core::GraphDeref>::Graph as $crate::core::Graph>::EdgeWeight;
				type Vertex = <<$($generic_graph)? $($type_graph)? as $crate::core::GraphDeref>::Graph as $crate::core::Graph>::Vertex;
				type VertexWeight = <<$($generic_graph)? $($type_graph)? as $crate::core::GraphDeref>::Graph as $crate::core::Graph>::VertexWeight;

				delegate::delegate! {
					to (self$($delegate)+).graph() {
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
		impl_insurer!{
			@inner
			@struct_id $struct
			@generic [ $($generics)* ]
			@delegate [ $(@generic $generic_graph)? ]
			@exclude [ $($trait)* ]
			@bounds [
				$($generic_graph)? $($type_graph)?: $crate::core::GraphDerefMut,
				$($generic_graph)? $(<$type_graph as $crate::core::GraphDeref>)?::Graph: $crate::core::GraphMut,
				$($bounds)*
			]
			@trait_id GraphMut [$crate::core]
			@implement {
				delegate::delegate! {
					to (self$($delegate)+).graph_mut() {
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

		// NewVertex
		impl_insurer!{
			@inner
			@struct_id $struct
			@generic [ $($generics)* ]
			@delegate [ $(@generic $generic_graph)? ]
			@exclude [ $($trait)* ]
			@bounds [
				$($generic_graph)? $($type_graph)?: $crate::core::GraphDerefMut + $crate::core::property::NewVertex,
				$($generic_graph)? $(<$type_graph as $crate::core::GraphDeref>)?::Graph: $crate::core::property::NewVertex,
				$($bounds)*
			]
			@trait_id NewVertex [$crate::core::property]
			@implement {
				delegate::delegate! {
					to (self$($delegate)+).graph_mut() {
						fn new_vertex_weighted(&mut self, w: Self::VertexWeight)
							-> Result<Self::Vertex, ()>;
					}
				}
			}
		}

		// RemoveVertex
		impl_insurer!{
			@inner
			@struct_id $struct
			@generic [ $($generics)* ]
			@delegate [ $(@generic $generic_graph)? ]
			@exclude [ $($trait)* ]
			@bounds [
				$($generic_graph)? $($type_graph)?: $crate::core::GraphDerefMut + $crate::core::property::RemoveVertex,
				$($generic_graph)? $(<$type_graph as $crate::core::GraphDeref>)?::Graph: $crate::core::property::RemoveVertex,
				$($bounds)*
			]
			@trait_id RemoveVertex [$crate::core::property]
			@implement {
				delegate::delegate! {
					to (self$($delegate)+).graph_mut() {
						fn remove_vertex(&mut self, v: Self::Vertex)
							-> Result<Self::VertexWeight, ()>;
					}
				}
			}
		}

		// AddEdge
		impl_insurer!{
			@inner
			@struct_id $struct
			@generic [ $($generics)* ]
			@delegate [ $(@generic $generic_graph)? ]
			@exclude [ $($trait)* ]
			@bounds [
				$($generic_graph)? $($type_graph)?: $crate::core::GraphDerefMut + $crate::core::property::AddEdge,
				$($generic_graph)? $(<$type_graph as $crate::core::GraphDeref>)?::Graph: $crate::core::property::AddEdge,
				$($bounds)*
			]
			@trait_id AddEdge [$crate::core::property]
			@implement {
				delegate::delegate! {
					to (self$($delegate)+).graph_mut() {
						fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
						where
							E: $crate::core::EdgeWeighted<Self::Vertex, Self::EdgeWeight>;
					}
				}
			}
		}

		// RemoveEdge
		impl_insurer!{
			@inner
			@struct_id $struct
			@generic [ $($generics)* ]
			@delegate [ $(@generic $generic_graph)? ]
			@exclude [ $($trait)* ]
			@bounds [
				$($generic_graph)? $($type_graph)?: $crate::core::GraphDerefMut + $crate::core::property::RemoveEdge,
				$($generic_graph)? $(<$type_graph as $crate::core::GraphDeref>)?::Graph: $crate::core::property::RemoveEdge,
				$($bounds)*
			]
			@trait_id RemoveEdge [$crate::core::property]
			@implement {
				delegate::delegate! {
					to (self$($delegate)+).graph_mut() {
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
		impl_insurer!{
			@inner
			@struct_id $struct
			@generic [ $($generics)* ]
			@delegate [ $(@generic $generic_graph)? ]
			@exclude [ $($trait)* ]
			@bounds [$($generic_graph: $crate::core::property::Unique,)? $($bounds)*]
			@trait_id Unique [$crate::core::property]
			@implement {}
		}

		// NoLoops
		impl_insurer!{
			@inner
			@struct_id $struct
			@generic [ $($generics)* ]
			@delegate [ $(@generic $generic_graph)? ]
			@exclude [ $($trait)* ]
			@bounds [$($generic_graph: $crate::core::property::NoLoops,)?  $($bounds)*]
			@trait_id NoLoops [$crate::core::property]
			@implement {}
		}

		// Reflexive
		impl_insurer!{
			@inner
			@struct_id $struct
			@generic [ $($generics)* ]
			@delegate [ $(@generic $generic_graph)? ]
			@exclude [ $($trait)* ]
			@bounds [
				$($generic_graph: $crate::core::property::Reflexive,)?
				$($generic_graph::EdgeWeight: Default,)?
				$(<$generic_graph::Graph as $crate::core::Graph>::EdgeWeight: Default,)?
				$($bounds)*
			]
			@trait_id Reflexive [$crate::core::property]
			@implement {}
		}

		// Weak
		impl_insurer!{
			@inner
			@struct_id $struct
			@generic [ $($generics)* ]
			@delegate [ $(@generic $generic_graph)? ]
			@exclude [ $($trait)* ]
			@bounds [$($generic_graph: $crate::core::property::Weak,)? $($bounds)*]
			@trait_id Weak [$crate::core::property]
			@implement {}
		}

		// Unilateral
		impl_insurer!{
			@inner
			@struct_id $struct
			@generic [ $($generics)* ]
			@delegate [ $(@generic $generic_graph)? ]
			@exclude [ $($trait)* ]
			@bounds [$($generic_graph: $crate::core::property::Unilateral,)? $($bounds)*]
			@trait_id Unilateral [$crate::core::property]
			@implement {}
		}

		// Connected
		impl_insurer!{
			@inner
			@struct_id $struct
			@generic [ $($generics)* ]
			@delegate [ $(@generic $generic_graph)? ]
			@exclude [ $($trait)* ]
			@bounds [$($generic_graph: $crate::core::property::Connected,)? $($bounds)*]
			@trait_id Connected [$crate::core::property]
			@implement {}
		}

		// Subgraph
		impl_insurer!{
			@inner
			@struct_id $struct
			@generic [ $($generics)* ]
			@delegate [ $(@generic $generic_graph)? ]
			@exclude [ $($trait)* ]
			@bounds [
				$($generic_graph: $crate::core::property::Subgraph<Vertex=Self::Vertex>,)?
				$($type_graph: $crate::core::property::Subgraph,)?
				$($bounds)*
			]
			@trait_id Subgraph [$crate::core::property]
			@implement {
				fn exit_edges<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
					(Self::Vertex, Self::Vertex)>>
				{
					(self$($delegate)+).exit_edges()
				}
			}
		}

		// NonNull
		impl_insurer!{
			@inner
			@struct_id $struct
			@generic [ $($generics)* ]
			@delegate [ $(@generic $generic_graph)? ]
			@exclude [ $($trait)* ]
			@bounds [
				$($generic_graph)? $($type_graph)?: $crate::core::property::NonNull,
				$($generic_graph)? $(<$type_graph as $crate::core::GraphDeref>)?::Graph: $crate::core::property::NonNull,
				$($bounds)*
			]
			@trait_id NonNull [$crate::core::property]
			@implement {
				delegate::delegate! {
					to (self$($delegate)+).graph() {
						fn get_vertex(&self) -> Self::Vertex;
					}
				}
			}
		}
	};

	{
		@inner
		@struct_id $struct:ident
		@generic [ $($generics:ident)* ]
		@delegate [ $(@generic $generic_graph:ident)? ]
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
				impl_insurer!{
					@inner
					@struct_id $struct
					@generic [ $($generics)* ]
					@delegate [ $(@generic $generic_graph)? ]
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
		@generic [ $($($generics:ident)+)? ]
		@delegate [ $(@generic $generic_graph:ident)? ]
		@exclude []
		@bounds [$($bounds:tt)*]
		@trait_id $trait_id:ident [ $($trait_path:tt)* ]
		@implement {$($impl:tt)*}
	} => {
		impl$(<$($generics)+>)? $($trait_path)*::$trait_id
			for $struct$(<$($generics)+>)?
			where
				$($generic_graph: $crate::core::Insure,)?
				$($bounds)*
		{$($impl)*}
	};
}

/// Implements properties for the given struct.
///
/// Doesn't implement the property given after the struct. That property
/// must use the original name of the trait.
///
/// Supported property traits:
/// Directed, Undirected, Unique, NoLoops, Reflexive, Weak, Unilateral,
/// Connected, Subgraph
#[macro_export]
macro_rules! impl_ensurer {
	{
		$(use <$($generics:ident),+>)? $struct:ty :
		$($exclude_props:ident),*
		for $type_graph:ty as (self $($delegate:tt)+)
		$(where $($bounds:tt)*)?
	} =>{
		$crate::impl_ensurer_inner!{
			@struct [ $struct ]
			@generic [ $($($generics)+)? ]
			@delegate [ $type_graph ]
			@delegate_to [ $($delegate)+ ]
			@exclude [ $($exclude_props)* ]
			@bounds [$($($bounds)*)?]
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_ensurer_inner {
	{
		@struct [ $struct:ty ]
		@generic [ $($generics:ident)* ]
		@delegate [ $delegate_type:ty ]
		@delegate_to [ $($delegate:tt)+ ]
		@exclude [ $($exclude_props:ident)* ]
		@bounds [$($bounds:tt)*]
	} => {

		//GraphDeref
		$crate::impl_ensurer_inner!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@delegate [ $delegate_type ]
			@exclude [ $($exclude_props)* ]
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
		$crate::impl_ensurer_inner!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@delegate [ $delegate_type ]
			@exclude [ $($exclude_props)* ]
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
		$crate::impl_ensurer_inner!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@delegate [ $delegate_type ]
			@exclude [ $($exclude_props)* ]
			@bounds [ $($bounds)* ]
			@trait_id Release [$crate::core]
			@implement {
				type Base = <$delegate_type as $crate::core::Release>::Base ;
				type Ensured = $delegate_type;

				fn release(self) -> Self::Ensured
				{
					self$($delegate)+
				}
			}
		}

		//Ensure
		$crate::impl_ensurer_inner!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@delegate [ $delegate_type ]
			@exclude [ $($exclude_props)* ]
			@bounds [ $($bounds)* ]
			@trait_id Ensure [$crate::core]
			@implement {
				fn ensure_unvalidated(c: Self::Ensured) -> Self
				{
					Self(c)
				}

				fn validate(_: &Self::Ensured) -> bool
				{
					true
				}
			}
		}

		//Graph
		$crate::impl_ensurer_inner!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@delegate [ $delegate_type ]
			@exclude [ $($exclude_props)* ]
			@bounds [ $($bounds)* ]
			@trait_id Graph [$crate::core]
			@implement {
				type Directedness = <<$delegate_type as
					$crate::core::GraphDeref>::Graph as $crate::core::Graph>::Directedness;
				type EdgeWeight = <<$delegate_type as
					$crate::core::GraphDeref>::Graph as $crate::core::Graph>::EdgeWeight;
				type Vertex = <<$delegate_type as
					$crate::core::GraphDeref>::Graph as $crate::core::Graph>::Vertex;
				type VertexWeight = <<$delegate_type as
					$crate::core::GraphDeref>::Graph as $crate::core::Graph>::VertexWeight;

				delegate::delegate! {
					to (self$($delegate)+).graph() {
						fn all_vertices_weighted<'a>(
							&'a self,
						) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, &'a Self::VertexWeight)>>;

						fn all_edges<'a>(
							&'a self,
						) -> Box<dyn 'a + Iterator<
							Item = (Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>;
					}
				}
			}
		}

		//GraphMut
		$crate::impl_ensurer_inner!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@delegate [ $delegate_type ]
			@exclude [ $($exclude_props)* ]
			@bounds [
				$delegate_type: $crate::core::GraphDerefMut,
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::GraphMut,
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
		$crate::impl_ensurer_inner!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@delegate [ $delegate_type ]
			@exclude [ $($exclude_props)* ]
			@bounds [
				$delegate_type: $crate::core::GraphDerefMut,
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::NewVertex,
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
		$crate::impl_ensurer_inner!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@delegate [ $delegate_type ]
			@exclude [ $($exclude_props)* ]
			@bounds [
				$delegate_type: $crate::core::GraphDerefMut,
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::RemoveVertex,
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
		$crate::impl_ensurer_inner!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@delegate [ $delegate_type ]
			@exclude [ $($exclude_props)* ]
			@bounds [
				$delegate_type: $crate::core::GraphDerefMut,
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::AddEdge,
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
		$crate::impl_ensurer_inner!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@delegate [ $delegate_type ]
			@exclude [ $($exclude_props)* ]
			@bounds [
				$delegate_type: $crate::core::GraphDerefMut,
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::RemoveEdge,
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
		$crate::impl_ensurer_inner!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@delegate [ $delegate_type ]
			@exclude [ $($exclude_props)* ]
			@bounds [
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::Unique,
				$($bounds)*
			]
			@trait_id Unique [$crate::core::property]
			@implement {}
		}

		// NoLoops
		$crate::impl_ensurer_inner!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@delegate [ $delegate_type ]
			@exclude [ $($exclude_props)* ]
			@bounds [
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::NoLoops,
				$($bounds)*
			]
			@trait_id NoLoops [$crate::core::property]
			@implement {}
		}

		// Reflexive
		$crate::impl_ensurer_inner!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@delegate [ $delegate_type ]
			@exclude [ $($exclude_props)* ]
			@bounds [
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::Reflexive,
				<<$delegate_type as $crate::core::GraphDeref>::Graph as
					$crate::core::Graph>::EdgeWeight: Default,
				$($bounds)*
			]
			@trait_id Reflexive [$crate::core::property]
			@implement {}
		}

		// Weak
		$crate::impl_ensurer_inner!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@delegate [ $delegate_type ]
			@exclude [ $($exclude_props)* ]
			@bounds [
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::Weak,
				$($bounds)*
			]
			@trait_id Weak [$crate::core::property]
			@implement {}
		}

		// Unilateral
		$crate::impl_ensurer_inner!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@delegate [ $delegate_type ]
			@exclude [ $($exclude_props)* ]
			@bounds [
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::Unilateral,
				$($bounds)*
			]
			@trait_id Unilateral [$crate::core::property]
			@implement {}
		}

		// Connected
		$crate::impl_ensurer_inner!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@delegate [ $delegate_type ]
			@exclude [ $($exclude_props)* ]
			@bounds [
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::Connected,
				$($bounds)*
			]
			@trait_id Connected [$crate::core::property]
			@implement {}
		}

		// Subgraph
		$crate::impl_ensurer_inner!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@delegate [ $delegate_type ]
			@exclude [ $($exclude_props)* ]
			@bounds [
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::Subgraph,
				$($bounds)*
			]
			@trait_id Subgraph [$crate::core::property]
			@implement {
				delegate::delegate!{
					to (self$($delegate)+).graph() {
						fn exit_edges<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
							(Self::Vertex, Self::Vertex)>>;
					}
				}
			}
		}

		// NonNull
		$crate::impl_ensurer_inner!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@delegate [ $delegate_type ]
			@exclude [ $($exclude_props)* ]
			@bounds [
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::NonNull,
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
		@struct [ $struct:ty ]
		@generic [ $($generics:ident)* ]
		@delegate [ $delegate_type:ty ]
		@exclude [ $exclude_props:ident $($exclude_props_rest:ident)* ]
		@bounds [$($bounds:tt)*]
		@trait_id $exclude_props_id:ident [ $($exclude_props_path:tt)* ]
		@implement {$($impl:tt)*}
	} => {
		tt_call::tt_if!{
			condition = [{tt_equal::tt_equal}]
			input = [{$exclude_props $exclude_props_id}]
			true = [{}]
			false = [{
				$crate::impl_ensurer_inner!{
					@struct [ $struct ]
					@generic [ $($generics)* ]
					@delegate [ $delegate_type ]
					@exclude [ $($exclude_props_rest)* ]
					@bounds [$($bounds)*]
					@trait_id $exclude_props_id [ $($exclude_props_path)* ]
					@implement {$($impl)*}
				}
			}]
		}
	};

	{
		@struct [ $struct:ty ]
		@generic [ $($($generics:ident)+)? ]
		@delegate [ $delegate_type:ty ]
		@exclude []
		@bounds [$($bounds:tt)*]
		@trait_id $exclude_props_id:ident [ $($exclude_props_path:tt)* ]
		@implement {$($impl:tt)*}
	} => {
		impl$(<$($generics)+>)? $($exclude_props_path)*::$exclude_props_id
			for $struct
			where
				$delegate_type: $crate::core::Ensure,
				$($bounds)*
		{$($impl)*}
	};
}
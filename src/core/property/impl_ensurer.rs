/// Implements properties for the given struct.
///
/// Doesn't implement the property given after the struct. That property
/// must use the original name of the trait.
///
/// Supported property traits:
/// Directed, Undirected, Unique, NoLoops, Reflexive, Weak, Unilateral,
/// Connected, Subgraph, Simple
#[macro_export]
macro_rules! impl_ensurer {
	{
		$(use <
			$($generics:ident),+
			$(; $(const $const_gen_id:ident : $const_gen_ty:ty),+)?
		>)?
		$struct:ty
		$(: $( $exclude_props:ident),+)?
		as (self $($delegate:tt)+) : $type_graph:ty
		$(as (self $($payload_to:tt)+) : $payload_type:ty)?
		$(where $($bounds:tt)*)?
	} =>{
		$crate::impl_properties!{
			@struct [ $struct ]
			@generic [ $($($generics)+)? ]
			@const_generic [ $($( $([$const_gen_id $const_gen_ty])+ )?)? ]
			@delegate [ $type_graph ]
			@delegate_to [ $($delegate)+ ]
			$(	@payload [$payload_type]
				@payload_to [$($payload_to)+]
			)?
			@exclude [ $($($exclude_props)+)? ]
			@bounds [$($($bounds)*)?]
		}
	};
}

#[macro_export]
macro_rules! base_graph {
	{
		$(use <$($generics:tt),+>)? $struct:ty
		$(: $($include_props:ident),+
		as (self $($delegate:tt)+) : $delegate_type:ty)?
		$(where $($bounds:tt)*)?
	} => {
		$crate::base_graph_inner! {
			@struct [ $struct ]
			@generics [ $($($generics)+)? ]
			$(
				@delegate [ $delegate_type ]
				@delegate_to [ $($delegate)+ ]
				@include [ $($include_props)+ ]
			)?
			@bounds [ $($($bounds)*)? ]
		}
	}
}

#[doc(hidden)]
#[macro_export]
macro_rules! base_graph_inner {
	{
		@struct [  $struct:ty ]
		@generics [ $($($generics:tt)+)? ]
		@delegate [ $delegate_type:ty ]
		@delegate_to [ $($delegate:tt)+ ]
		@include [ $($include_props:ident)+ ]
		@bounds [ $($bounds:tt)* ]
	} => {
		$crate::base_graph_inner!{
			@struct [  $struct ]
			@generics [ $($($generics)+)? ]
			@bounds [ $($bounds)* ]
		}
		$crate::impl_properties! {
			@struct [ $struct ]
			@generic [ $($($generics)+)? ]
			@const_generic [ ]
			@delegate [ $delegate_type ]
			@delegate_to [ $($delegate)+ ]
			@include [ $($include_props)+ ]
			@bounds [$($bounds)*]
		}
	};
	{
		@struct [  $struct:ty ]
		@generics [ $($($generics:tt)+)? ]
		@bounds [ $($bounds:tt)* ]
	} => {
		impl$(<$($generics),+>)? $crate::core::GraphDeref for $struct
		where $($bounds)*
		{
			type Graph = Self;

			fn graph(&self) -> &Self::Graph
			{
				self
			}
		}

		impl$(<$($generics),+>)? $crate::core::GraphDerefMut for $struct
		where $($bounds)*
		{
			fn graph_mut(&mut self) -> &mut Self::Graph
			{
				self
			}
		}

		impl$(<$($generics),+>)? $crate::core::BaseGraph for $struct
		where $($bounds)*
		{}
	}
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_properties {
	{
		@struct [ $struct:ty ]
		@generic [ $($generics:tt)* ]
		@const_generic [ $($const_generics:tt)* ]
		@delegate [ $delegate_type:ty ]
		@delegate_to [ $($delegate:tt)+ ]
		$(	@payload [$payload_type:ty]
			@payload_to [$($payload_to:tt)+]
		)?
		@exclude $exclude_tt:tt
		@include $include_tt:tt
		@bounds [$($bounds:tt)*]
	} => {
		std::compile_error!("'impl_properties' doesn't accept both 'include' and 'exclude' properties:\n"
		 + std::stringify!(
		 	@exclude $exclude_tt
			@include $include_tt
		));
	};
	{
		@struct [ $struct:ty ]
		@generic [ $($generics:tt)* ]
		@const_generic [ $([$const_gen_id:ident $const_gen_ty:ty])* ]
		@delegate [ $delegate_type:ty ]
		@delegate_to [ $($delegate:tt)+ ]
		$(	@payload [$payload_type:ty]
			@payload_to [$($payload_to:tt)+]
		)?
		$(@exclude [ $($exclude_props:ident)* ])?
		$(@include [ $($include_props:ident)* ])?
		@bounds [$($bounds:tt)*]
	} => {

		//GraphDeref
		$crate::impl_properties!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@const_generic [ $([$const_gen_id $const_gen_ty])* ]
			@delegate [ $delegate_type ]
			$(@exclude [ $($exclude_props)* ])?
			$(@include [ $($include_props)* ])?
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
		$crate::impl_properties!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@const_generic [ $([$const_gen_id $const_gen_ty])* ]
			@delegate [ $delegate_type ]
			$(@exclude [ $($exclude_props)* ])?
			$(@include [ $($include_props)* ])?
			@bounds [ $($bounds)* ]
			@trait_id GraphDerefMut [$crate::core]
			@implement {
				fn graph_mut(&mut self) -> &mut Self::Graph
				{
					self
				}
			}
		}

		//ReleasePayload
		$crate::impl_properties!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@const_generic [ $([$const_gen_id $const_gen_ty])* ]
			@delegate [ $delegate_type ]
			$(@exclude [ $($exclude_props)* ])?
			$(@include [ $($include_props)* ])?
			@bounds [ $($bounds)* ]
			@trait_id ReleasePayload [$crate::core]
			@implement {
				type Base = <$delegate_type as $crate::core::ReleasePayload>::Base ;
				type Ensured = $delegate_type;
				#[allow(unused_parens)]
				type Payload = ($($payload_type,)?<$delegate_type as $crate::core::ReleasePayload>::Payload);
				#[allow(unused_parens)]
				fn release(self) -> (Self::Ensured, ($($payload_type)?))
				{
					(self$($delegate)+, ($(self$($payload_to)+)?))
				}
			}
		}

		//Ensure
		$crate::impl_properties!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@const_generic [ $([$const_gen_id $const_gen_ty])* ]
			@delegate [ $delegate_type ]
			$(@exclude [ $($exclude_props)* ])?
			$(@include [ $($include_props)* ])?
			@bounds [ $($bounds)* ]
			@trait_id Ensure [$crate::core]
			@implement {
				fn ensure_unchecked(c: Self::Ensured, _p:($($payload_type)?)) -> Self
				{
					$crate::make_ensurer!(c, _p $($payload_type)?)
				}

				fn can_ensure(_: &Self::Ensured, _:&($($payload_type)?)) -> bool
				{
					true
				}
			}
		}

		//Graph
		$crate::impl_properties!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@const_generic [ $([$const_gen_id $const_gen_ty])* ]
			@delegate [ $delegate_type ]
			$(@exclude [ $($exclude_props)* ])?
			$(@include [ $($include_props)* ])?
			@bounds [ $($bounds)* ]
			@trait_id Graph [$crate::core]
			@implement {
				type Directedness = <<$delegate_type as
					$crate::core::GraphDeref>::Graph as $crate::core::Graph>::Directedness;
				type EdgeWeight = <<$delegate_type as
					$crate::core::GraphDeref>::Graph as $crate::core::Graph>::EdgeWeight;
				type EdgeWeightRef<'a> = <<$delegate_type as
					$crate::core::GraphDeref>::Graph as $crate::core::Graph>::EdgeWeightRef<'a>
				where
					$delegate_type: 'a
					$(, $generics: 'a)*
				;
				type Vertex = <<$delegate_type as
					$crate::core::GraphDeref>::Graph as $crate::core::Graph>::Vertex;
				type VertexWeight = <<$delegate_type as
					$crate::core::GraphDeref>::Graph as $crate::core::Graph>::VertexWeight;

				delegate::delegate! {
					to $crate::core::GraphDeref::graph(&self$($delegate)+){
						fn all_vertices_weighted(
							&self,
						) -> impl Iterator<Item = (Self::Vertex, &Self::VertexWeight)>;

						fn edges_between(
							&self,
							source: impl std::borrow::Borrow<Self::Vertex>,
							sink: impl std::borrow::Borrow<Self::Vertex>,
						) -> impl Iterator<Item = Self::EdgeWeightRef<'_>>;
					}
				}
			}
		}

		//GraphMut
		$crate::impl_properties!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@const_generic [ $([$const_gen_id $const_gen_ty])* ]
			@delegate [ $delegate_type ]
			$(@exclude [ $($exclude_props)* ])?
			$(@include [ $($include_props)* ])?
			@bounds [
				$delegate_type: $crate::core::GraphDerefMut,
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::GraphMut,
				$($bounds)*
			]
			@trait_id GraphMut [$crate::core]
			@implement {
				delegate::delegate! {
					to $crate::core::GraphDerefMut::graph_mut(&mut self$($delegate)+) {
						fn all_vertices_weighted_mut(
							&mut self,
						) -> impl Iterator<Item = (Self::Vertex, &mut Self::VertexWeight)>;

						fn edges_between_mut(
							&mut self,
							source: impl std::borrow::Borrow<Self::Vertex>,
							sink: impl std::borrow::Borrow<Self::Vertex>,
						) -> impl Iterator<Item = &mut Self::EdgeWeight>;
					}
				}
			}
		}

		// NewVertex
		$crate::impl_properties!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@const_generic [ $([$const_gen_id $const_gen_ty])* ]
			@delegate [ $delegate_type ]
			$(@exclude [ $($exclude_props)* ])?
			$(@include [ $($include_props)* ])?
			@bounds [
				$delegate_type: $crate::core::GraphDerefMut,
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::NewVertex,
				$($bounds)*
			]
			@trait_id NewVertex [$crate::core::property]
			@implement {
				delegate::delegate! {
					to $crate::core::GraphDerefMut::graph_mut(&mut self$($delegate)+){
						fn new_vertex_weighted(&mut self, w: Self::VertexWeight)
							-> Result<Self::Vertex, ()>;
					}
				}
			}
		}

		// RemoveVertex
		$crate::impl_properties!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@const_generic [ $([$const_gen_id $const_gen_ty])* ]
			@delegate [ $delegate_type ]
			$(@exclude [ $($exclude_props)* ])?
			$(@include [ $($include_props)* ])?
			@bounds [
				$delegate_type: $crate::core::GraphDerefMut,
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::RemoveVertex,
				$($bounds)*
			]
			@trait_id RemoveVertex [$crate::core::property]
			@implement {
				delegate::delegate! {
					to $crate::core::GraphDerefMut::graph_mut(&mut self$($delegate)+) {
						fn remove_vertex(&mut self, v: impl std::borrow::Borrow<Self::Vertex>)
							-> Result<Self::VertexWeight, ()>;
					}
				}
			}
		}

		// AddEdge
		$crate::impl_properties!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@const_generic [ $([$const_gen_id $const_gen_ty])* ]
			@delegate [ $delegate_type ]
			$(@exclude [ $($exclude_props)* ])?
			$(@include [ $($include_props)* ])?
			@bounds [
				$delegate_type: $crate::core::GraphDerefMut,
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::AddEdge,
				$($bounds)*
			]
			@trait_id AddEdge [$crate::core::property]
			@implement {
				delegate::delegate! {
					to $crate::core::GraphDerefMut::graph_mut(&mut self$($delegate)+) {
						fn add_edge_weighted(
							&mut self,
							source: impl std::borrow::Borrow<Self::Vertex>,
							sink: impl std::borrow::Borrow<Self::Vertex>,
							weight: Self::EdgeWeight,
						) -> Result<(), ()>;
					}
				}
			}
		}

		// RemoveEdge
		$crate::impl_properties!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@const_generic [ $([$const_gen_id $const_gen_ty])* ]
			@delegate [ $delegate_type ]
			$(@exclude [ $($exclude_props)* ])?
			$(@include [ $($include_props)* ])?
			@bounds [
				$delegate_type: $crate::core::GraphDerefMut,
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::RemoveEdge,
				$($bounds)*
			]
			@trait_id RemoveEdge [$crate::core::property]
			@implement {
				delegate::delegate! {
					to $crate::core::GraphDerefMut::graph_mut(&mut self$($delegate)+) {
						fn remove_edge_where_weight<F>(
							&mut self,
							source: impl std::borrow::Borrow<Self::Vertex>,
							sink: impl std::borrow::Borrow<Self::Vertex>,
							f: F,
						) -> Result<Self::EdgeWeight, ()>
							where
								F: Fn(&Self::EdgeWeight) -> bool;
					}
				}
			}
		}

		// VertexCount
		$crate::impl_properties!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@const_generic [ $([$const_gen_id $const_gen_ty])* ]
			@delegate [ $delegate_type ]
			$(@exclude [ $($exclude_props)* ])?
			$(@include [ $($include_props)* ])?
			@bounds [
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::VertexCount,
				$($bounds)*
			]
			@trait_id VertexCount [$crate::core::property]
			@implement {
				type Count = <<$delegate_type as $crate::core::GraphDeref>::Graph
					as $crate::core::property::VertexCount>::Count;
			}
		}

		// EdgeCount
		$crate::impl_properties!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@const_generic [ $([$const_gen_id $const_gen_ty])* ]
			@delegate [ $delegate_type ]
			$(@exclude [ $($exclude_props)* ])?
			$(@include [ $($include_props)* ])?
			@bounds [
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::EdgeCount,
				$($bounds)*
			]
			@trait_id EdgeCount [$crate::core::property]
			@implement {
				type Count = <<$delegate_type as $crate::core::GraphDeref>::Graph
					as $crate::core::property::EdgeCount>::Count;
			}
		}

		// Unique
		$crate::impl_properties!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@const_generic [ $([$const_gen_id $const_gen_ty])* ]
			@delegate [ $delegate_type ]
			$(@exclude [ $($exclude_props)* ])?
			$(@include [ $($include_props)* ])?
			@bounds [
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::Unique,
				$($bounds)*
			]
			@trait_id Unique [$crate::core::property]
			@implement {}
		}

		// NoLoops
		$crate::impl_properties!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@const_generic [ $([$const_gen_id $const_gen_ty])* ]
			@delegate [ $delegate_type ]
			$(@exclude [ $($exclude_props)* ])?
			$(@include [ $($include_props)* ])?
			@bounds [
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::NoLoops,
				$($bounds)*
			]
			@trait_id NoLoops [$crate::core::property]
			@implement {}
		}

		// Reflexive
		$crate::impl_properties!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@const_generic [ $([$const_gen_id $const_gen_ty])* ]
			@delegate [ $delegate_type ]
			$(@exclude [ $($exclude_props)* ])?
			$(@include [ $($include_props)* ])?
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
		$crate::impl_properties!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@const_generic [ $([$const_gen_id $const_gen_ty])* ]
			@delegate [ $delegate_type ]
			$(@exclude [ $($exclude_props)* ])?
			$(@include [ $($include_props)* ])?
			@bounds [
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::Weak,
				$($bounds)*
			]
			@trait_id Weak [$crate::core::property]
			@implement {}
		}

		// Unilateral
		$crate::impl_properties!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@const_generic [ $([$const_gen_id $const_gen_ty])* ]
			@delegate [ $delegate_type ]
			$(@exclude [ $($exclude_props)* ])?
			$(@include [ $($include_props)* ])?
			@bounds [
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::Unilateral,
				$($bounds)*
			]
			@trait_id Unilateral [$crate::core::property]
			@implement {}
		}

		// Connected
		$crate::impl_properties!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@const_generic [ $([$const_gen_id $const_gen_ty])* ]
			@delegate [ $delegate_type ]
			$(@exclude [ $($exclude_props)* ])?
			$(@include [ $($include_props)* ])?
			@bounds [
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::Connected,
				$($bounds)*
			]
			@trait_id Connected [$crate::core::property]
			@implement {}
		}

		// Subgraph
		$crate::impl_properties!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@const_generic [ $([$const_gen_id $const_gen_ty])* ]
			@delegate [ $delegate_type ]
			$(@exclude [ $($exclude_props)* ])?
			$(@include [ $($include_props)* ])?
			@bounds [
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::Subgraph,
				$($bounds)*
			]
			@trait_id Subgraph [$crate::core::property]
			@implement {
				delegate::delegate!{
					to $crate::core::GraphDeref::graph(&self$($delegate)+) {
						fn exit_edges(&self)
							-> impl '_ + Iterator<Item=(Self::Vertex, Self::Vertex)>;
					}
				}
			}
		}

		// HasVertex
		$crate::impl_properties!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@const_generic [ $([$const_gen_id $const_gen_ty])* [IMPL_PROPERTIES_HASVERTEX_V usize] ]
			@delegate [ $delegate_type ]
			$(@exclude [ $($exclude_props)* ])?
			$(@include [ $($include_props)* ])?
			@bounds [
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::HasVertex<IMPL_PROPERTIES_HASVERTEX_V>,
				$($bounds)*
			]
			@trait_id HasVertex <IMPL_PROPERTIES_HASVERTEX_V> [$crate::core::property]
			@implement {
				delegate::delegate! {
					to $crate::core::GraphDeref::graph(&self$($delegate)+) {
						fn get_vertex_at<const N: usize>(&self) -> Self::Vertex;
					}
				}
			}
		}

		// Rooted
		$crate::impl_properties!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@const_generic [ $([$const_gen_id $const_gen_ty])* ]
			@delegate [ $delegate_type ]
			$(@exclude [ $($exclude_props)* ])?
			$(@include [ $($include_props)* ])?
			@bounds [
				$delegate_type: $crate::core::GraphDerefMut,
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::Rooted,
				$($bounds)*
			]
			@trait_id Rooted [$crate::core::property]
			@implement {
				delegate::delegate! {
					to $crate::core::GraphDeref::graph(&self$($delegate)+) {
						fn root(&self) -> Self::Vertex;
					}
					to $crate::core::GraphDerefMut::graph_mut(&mut self$($delegate)+) {
						fn set_root(&mut self, v: impl std::borrow::Borrow<Self::Vertex>)
							-> std::result::Result<(), ()>;
					}
				}
			}
		}

		// Simple
		$crate::impl_properties!{
			@struct [ $struct ]
			@generic [ $($generics)* ]
			@const_generic [ $([$const_gen_id $const_gen_ty])* ]
			@delegate [ $delegate_type ]
			$(@exclude [ $($exclude_props)* ])?
			$(@include [ $($include_props)* ])?
			@bounds [
				<$delegate_type as $crate::core::GraphDeref>::Graph:
					$crate::core::property::Simple,
				$($bounds)*
			]
			@trait_id Simple [$crate::core::property]
			@implement {}
		}
	};

	{
		@struct [ $struct:ty ]
		@generic [ $($generics:tt)* ]
		@const_generic [ $($const_generics:tt)* ]
		@delegate [ $delegate_type:ty ]
		@include [ $include_props:ident $($include_props_rest:ident)* ]
		@bounds [$($bounds:tt)*]
		@trait_id $include_props_id:ident $(<$($trait_gens:ident),+>)? [ $($include_props_path:tt)* ]
		@implement {$($impl:tt)*}
	} => {
		tt_call::tt_if!{
			condition = [{tt_equal::tt_equal}]
			input = [{ $include_props $include_props_id}]
			true = [{
				$crate::impl_properties!{
					@struct [ $struct ]
					@generic [ $($generics)* ]
					@const_generic [ $($const_generics)* ]
					@delegate [ $delegate_type ]
					@exclude []
					@bounds [$($bounds)*]
					@trait_id $include_props_id $(<$($trait_gens),+>)?[ $($include_props_path)* ]
					@implement {$($impl)*}
				}
			}]
			false = [{
				$crate::impl_properties!{
					@struct [ $struct ]
					@generic [ $($generics)* ]
					@const_generic [ $($const_generics)* ]
					@delegate [ $delegate_type ]
					@include [ $($include_props_rest)* ]
					@bounds [$($bounds)*]
					@trait_id $include_props_id $(<$($trait_gens),+>)? [ $($include_props_path)* ]
					@implement {$($impl)*}
				}
			}]
		}
	};

	{
		@struct [ $struct:ty ]
		@generic [ $($generics:tt)* ]
		@const_generic [ $($const_generics:tt)* ]
		@delegate [ $delegate_type:ty ]
		@exclude [ $exclude_props:ident $($exclude_props_rest:ident)* ]
		@bounds [$($bounds:tt)*]
		@trait_id $exclude_props_id:ident $(<$($trait_gens:ident),+>)?[ $($exclude_props_path:tt)* ]
		@implement {$($impl:tt)*}
	} => {
		tt_call::tt_if!{
			condition = [{tt_equal::tt_equal}]
			input = [{ $exclude_props $exclude_props_id}]
			true = [{}]
			false = [{
				$crate::impl_properties!{
					@struct [ $struct ]
					@generic [ $($generics)* ]
					@const_generic [ $($const_generics)* ]
					@delegate [ $delegate_type ]
					@exclude [ $($exclude_props_rest)* ]
					@bounds [$($bounds)*]
					@trait_id $exclude_props_id $(<$($trait_gens),+>)?[ $($exclude_props_path)* ]
					@implement {$($impl)*}
				}
			}]
		}
	};

	{
		@struct [ $struct:ty ]
		@generic [ $($($generics:tt)+)? ]
		@const_generic [ $([$const_gen_id:ident $const_gen_ty:ty])* ]
		@delegate [ $delegate_type:ty ]
		@exclude []
		@bounds [$($bounds:tt)*]
		@trait_id $exclude_props_id:ident $(<$($trait_gens:ident),+>)? [ $($exclude_props_path:tt)* ]
		@implement {$($impl:tt)*}
	} => {
		impl<$($($generics,)+)? $(const $const_gen_id : $const_gen_ty,)*> $($exclude_props_path)*::$exclude_props_id $(<$($trait_gens,)+>)?
			for $struct
			where
				$delegate_type: $crate::core::Ensure,
				$($bounds)*
		{$($impl)*}
	};
	{
		@struct [ $struct:ty ]
		@generic [ $($($generics:tt)+)? ]
		@const_generic [ $($const_generics:tt)* ]
		@delegate [ $delegate_type:ty ]
		@include []
		@bounds [$($bounds:tt)*]
		@trait_id $exclude_props_id:ident $(<$($trait_gens:ident),+>)? [ $($exclude_props_path:tt)* ]
		@implement {$($impl:tt)*}
	} => {}
}

#[doc(hidden)]
#[macro_export]
macro_rules! make_ensurer {
	{
		$ensured:ident
		, $payload:ident $($rest:tt)+
	} => {
		Self($ensure, $payload)
	};
	{
		$ensured:ident
		, $payload:ident
	} => {
		Self($ensured)
	}
}

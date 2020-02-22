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
		//Directed
		impl_constraints!{
			@inner
			$struct<$generic_graph>: [$($trait)*]
			$([$($bounds)*])? DirectedConstraint
			{}
		}

		//Undirected
		impl_constraints!{
			@inner
			$struct<$generic_graph>: [$($trait)*]
			$([$($bounds)*])? UndirectedConstraint
			{}
		}

		// NewVertex
		impl_constraints!{
			@inner
			$struct<$generic_graph>: [$($trait)*]
			[
				$generic_graph: $crate::core::ImplGraphMut,
				$generic_graph::Graph: $crate::core::constraint::NewVertex,
				$($($bounds)*)?
			]
			NewVertex
			{
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
			$struct<$generic_graph>: [$($trait)*]
			[
				$generic_graph: $crate::core::ImplGraphMut,
				$generic_graph::Graph: $crate::core::constraint::RemoveVertex,
				$($($bounds)*)?
			]
			RemoveVertex
			{
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
			$struct<$generic_graph>: [$($trait)*]
			[
				$generic_graph: $crate::core::ImplGraphMut,
				$generic_graph::Graph: $crate::core::constraint::AddEdge,
				$($($bounds)*)?
			]
			AddEdge
			{
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
			$struct<$generic_graph>: [$($trait)*]
			[
				$generic_graph: $crate::core::ImplGraphMut,
				$generic_graph::Graph: $crate::core::constraint::RemoveEdge,
				$($($bounds)*)?
			]
			RemoveEdge
			{
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
			$struct<$generic_graph>: [$($trait)*]
			$([$($bounds)*])? Unique
			{}
		}

		// NoLoops
		impl_constraints!{
			@inner
			$struct<$generic_graph>: [$($trait)*]
			$([$($bounds)*])? NoLoops
			{}
		}

		// Reflexive
		impl_constraints!{
			@inner
			$struct<$generic_graph>: [$($trait)*]
			[
				$generic_graph::EdgeWeight: Default,
				<$generic_graph::Graph as Graph>::EdgeWeight: Default,
				$($($bounds)*)?
			]
			Reflexive
			{}
		}

		// Weak
		impl_constraints!{
			@inner
			$struct<$generic_graph>: [$($trait)*]
			$([$($bounds)*])? Weak
			{}
		}

		// Unilateral
		impl_constraints!{
			@inner
			$struct<$generic_graph>: [$($trait)*]
			$([$($bounds)*])? Unilateral
			{}
		}

		// Connected
		impl_constraints!{
			@inner
			$struct<$generic_graph>: [$($trait)*]
			$([$($bounds)*])? Connected
			{}
		}

		// Subgraph
		impl_constraints!{
			@inner
			$struct<$generic_graph>: [$($trait)*]
			[
				$generic_graph: $crate::core::constraint::Subgraph<Vertex=Self::Vertex>,
				$($($bounds)*)?
			]
			Subgraph
			{
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
			$struct<$generic_graph>: [$($trait)*]
			$([$($bounds)*])? NonNull
			{}
		}
	};

	{
		@inner
		$struct:ident<$generic_graph:ident>: [ $trait:ident $($trait_rest:ident)* ]
		$([$($bounds:tt)*])? $constraint:ident
		{$($impl:tt)*}
	} => {
		tt_call::tt_if!{
			condition = [{tt_equal::tt_equal}]
			input = [{$trait $constraint}]
			true = [{}]
			false = [{
				impl_constraints!{
					@inner
					$struct<$generic_graph>: [ $($trait_rest)* ]
					$([$($bounds)*])? $constraint
					{$($impl)*}
				}
			}]
		}
	};

	{
		@inner
		$struct:ident<$generic_graph:ident>: [ ]
		$([$($bounds:tt)*])? $constraint:ident
		{$($impl:tt)*}
	} => {
		impl<$generic_graph: $crate::core::Constrainer> $crate::core::constraint::$constraint
			for $struct<$generic_graph>
			where
				$generic_graph: $crate::core::constraint::$constraint,
				$($($bounds)*)?
		{$($impl)*}
	}
}

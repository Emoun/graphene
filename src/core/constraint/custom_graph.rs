
///
/// A macro for defining a custom graph with a specific set of constraints.
///
/// The resulting graph is generic over vertices and weights (<V,W>) and implement
/// `GraphWrapper` and the constraints given.
///
/// Syntax:
///
/// ``` text
/// struct <graph name>
/// where <name of type implementing ConstrainedGraph>
/// impl (<constraint>),*
/// use (<GraphWrapper to use to uphold the constraint, i.e. will wrap the ConstrainedGraph>),*
/// ```
///
/// The 'impl' and 'use' clauses are optional.
///
///
#[macro_export]
macro_rules! custom_graph{
	{
		struct $graph_name:ident where $base_graph:ident
	} => {
		custom_graph!{struct $graph_name where $base_graph use}
	};
	{
		pub struct $graph_name:ident where $base_graph:ident
	} => {
		custom_graph!{pub struct $graph_name where $base_graph use}
	};
	{
		struct $graph_name:ident where $base_graph:ident use $($con_graph:ident),*
	} => {
		custom_graph!{@declare_struct $graph_name; $base_graph; $($con_graph),*}
		custom_graph!{@impl_minimum_traits_and_derives $graph_name; $base_graph; $($con_graph),*}
	};
	{
		pub struct $graph_name:ident where $base_graph:ident use $($con_graph:ident),*
	} => {
		custom_graph!{@declare_struct pub $graph_name; $base_graph; $($con_graph),*}
		custom_graph!{@impl_minimum_traits_and_derives $graph_name; $base_graph; $($con_graph),*}
	};
	{
		struct $graph_name:ident where $base_graph:ident impl $($con_traits:ident),*
	} => {
		custom_graph!{struct $graph_name where $base_graph}
		custom_graph!{@impl_constraint_traits $graph_name; $($con_traits),*}
	};
	{
		pub struct $graph_name:ident where $base_graph:ident impl $($con_traits:ident),*
	} => {
		custom_graph!{pub struct $graph_name where $base_graph }
		custom_graph!{@impl_constraint_traits $graph_name; $($con_traits),*}
	};
	{
		struct $graph_name:ident where $base_graph:ident impl $($con_traits:ident),*
		use $($con_graph:ident),*
	} => {
		custom_graph!{struct $graph_name where $base_graph use $($con_graph),*}
		custom_graph!{@impl_constraint_traits $graph_name; $($con_traits),*}
	};
	{
		pub struct $graph_name:ident where $base_graph:ident impl $($con_traits:ident),*
		use $($con_graph:ident),*
	} => {
		custom_graph!{pub struct $graph_name where $base_graph use $($con_graph),*}
		custom_graph!{@impl_constraint_traits $graph_name; $($con_traits),*}
	};
	
//helpers
	{
		@declare_struct
		pub $graph_name:ident; $base_graph:ident; $($con_graph:ident),*
	}=>{
		// Define graph struct
		pub struct $graph_name<V,W>
			where
				V: Vertex,
				W: Weight,
		{
			wraps:
			custom_graph!{
				@in_struct
				$($con_graph,$base_graph<V,W> >>)*
				$base_graph<V,W>
			}
		}
	};
	{
		@declare_struct
		$graph_name:ident; $base_graph:ident; $($con_graph:ident),*
	}=>{
		// Define graph struct
		struct $graph_name<V,W>
			where
				V: Vertex,
				W: Weight,
		{
			wraps:
			custom_graph!{
				@in_struct
				$($con_graph,$base_graph<V,W> >>)*
				$base_graph<V,W>
			}
		}
	};
	{
		@impl_minimum_traits_and_derives
		$graph_name:ident; $base_graph:ident; $($con_graph:ident),*
	}=>{
		custom_graph!{@impl_graph_wrapper $graph_name; $base_graph; $($con_graph),*}
		custom_graph!{@impl_base_graph $graph_name}
		custom_graph!{@impl_contained_graph $graph_name}
		custom_graph!{@derive_debug $graph_name}
		custom_graph!{@derive_clone $graph_name}
	};
	{
		@impl_graph_wrapper
		$graph_name:ident; $base_graph:ident; $($con_graph:ident),*
	}=>{
		// Impl GraphWrapper
		impl<V,W> GraphWrapper for $graph_name<V,W>
			where
				V: Vertex,
				W: Weight,
		{
			custom_graph!{
				@as_associated
				custom_graph!{
					@in_struct
					$($con_graph,$base_graph<V,W> >>)*
					$base_graph<V,W>
				}
			}
			
			fn wrap(g: Self::Wrapped) -> Self{
				$graph_name{wraps: g}
			}
			
			fn wrapped(&self) -> &Self::Wrapped{
				&self.wraps
			}
			
			fn wrapped_mut(&mut self) -> &mut Self::Wrapped{
				&mut self.wraps
			}
			
			fn unwrap(self) -> Self::Wrapped{
				self.wraps
			}
		}
	};
	{
		@impl_base_graph
		$graph_name:ident
	}=>{
		// Impl BaseGraph
		impl<V,W> BaseGraph for $graph_name<V,W>
			where
				V: Vertex,
				W: Weight,
		{
			type Vertex = V;
			type Weight = W;
			type VertexIter = <<Self as GraphWrapper>::Wrapped as BaseGraph>::VertexIter;
			type EdgeIter = <<Self as GraphWrapper>::Wrapped as BaseGraph>::EdgeIter;
		
			fn empty_graph() -> Self{
				$graph_name::wrap(
					<Self as GraphWrapper>::Wrapped::empty_graph()
				)
			}
			wrapped_method!{all_vertices(&self) -> Self::VertexIter}
	
			wrapped_method!{all_edges(&self) -> Self::EdgeIter}
			
			wrapped_method!{add_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>}
			
			wrapped_method!{remove_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>}
			
			wrapped_method!{add_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()>}
			
			wrapped_method!{remove_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()>}
		}
	};
	{
		@impl_contained_graph
		$graph_name:ident
	}=>{
		//Impl ConstrainedGraph
		impl<V,W> ConstrainedGraph for $graph_name<V,W>
			where
				V: Vertex,
				W: Weight,
		{
			wrapped_method!{invariant_holds(&self) -> bool}
			
			wrapped_uncon_methods!{}
		}
	};
	{
		@derive_debug
		$graph_name:ident
	}=>{
		// Derive Debug
		impl<V,W> std::fmt::Debug for $graph_name<V,W>
			where
				V: Vertex + std::fmt::Debug,
				W: Weight + std::fmt::Debug,
		{
			fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result{
				write!(f, "{} {{ wraps: {:?} }}", stringify!($graph_name), self.wraps)
			}
		}
	};
	{
		@derive_clone
		$graph_name:ident
	}=>{
		//Derive Clone
		impl<V,W> Clone for $graph_name<V,W>
			where
				V: Vertex,
				W: Weight,
		{
			fn clone(&self) -> $graph_name<V,W>{
				$graph_name::wrap(self.wraps.clone())
			}
		}
	};
	{
		@impl_constraint_traits
		$graph_name:ident; $($con_traits:ident),*
	}=>{
		// Impl the constraint traits
		$(
			impl<V,W> $con_traits for $graph_name<V,W>
				where
				V: Vertex,
				W: Weight,
			{}
		)*
	};
	{
		@as_associated
		$($rest:tt)*
	}=>{
		type Wrapped = $($rest)* ;
	};
	{
		@in_struct
		$base_graph:ident<$V:ident, $W:ident>
	}=>{
		$base_graph<$V,$W>
	};
	{
		@in_struct
		$con_graph:ident,$base_graph:ident<$V:ident, $W:ident> >>
		$($rest:tt)*
	} => {
		$con_graph<
			custom_graph!{
				@in_struct
				$($rest)*
			}
		>
	};
}














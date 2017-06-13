
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
/// use (<GraphWrapper to use to uphold the constraint, i.e. will wrap the ConstrainedGraph>).*
/// ```
///
/// The 'impl' and 'use' clauses are optional.
///
///
#[macro_export]
macro_rules! custom_graph{
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
	{
		struct $graph_name:ident
		
		where $base_graph:ident
	} => {
		custom_graph!{
			struct $graph_name
		
			where $base_graph
		
			use
		}
	};
	{
		struct $graph_name:ident
		
		where $base_graph:ident
		
		use $($con_graph:ident),*
	} => {
		
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
		struct $graph_name:ident
		
		where $base_graph:ident
		
		impl $($con_trait:ident),*
		
		use $($con_graph:ident),*
	} => {
		custom_graph!{
			struct $graph_name
			
			where $base_graph
			
			use $($con_graph),*
		}
		// Impl the constraint traits
		$(
			impl<V,W> $con_trait for $graph_name<V,W>
				where
				V: Vertex,
				W: Weight,
			{}
		)*
	}
}














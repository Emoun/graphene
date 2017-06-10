
///
/// A macro for defining a custom graph with a specific set of constraints.
///
///
#[macro_export]
macro_rules! custom_graph{
	{
		@in_empty_graph
		$base_graph:ident<$V:ident, $W:ident>
	}=>{
		$base_graph::<$V,$W>::empty_graph()
	};
	{
		@in_empty_graph
		$con_graph:ident,$base_graph:ident<$V:ident, $W:ident> >>
		$($rest:tt)*
	} => {
		$con_graph{	graph:
					custom_graph!{@in_empty_graph $($rest)*}
					}
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
		$con_graph<	$V,$W,
					<$base_graph<$V,$W> as BaseGraph>::VertexIter,
					<$base_graph<$V,$W> as BaseGraph>::EdgeIter,
					custom_graph!{@in_struct $($rest)*}
					>
	};
	{
		// Name of the resulting graph.
		$graph_name:ident
		
		//Name of the basic graph implementation to use
		where $base_graph:ident
		
		// Name of the constraint implementations
		impl $($con_trait:ident),*
		
		//Constraint traits to implement
		use $($con_graph:ident),*
	} => {
		// Derive Debug
		impl<V,W> std::fmt::Debug for $graph_name<V,W>
			where
				V: Vertex + std::fmt::Debug,
				W: Weight + std::fmt::Debug,
		{
			fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result{
				write!(f, "{} {{ graph: {:?} }}", stringify!($graph_name), self.graph)
			}
		}
		
		//Derive Clone
		impl<V,W> Clone for $graph_name<V,W>
			where
				V: Vertex,
				W: Weight,
		{
			fn clone(&self) -> $graph_name<V,W>{
				$graph_name{graph: self.graph.clone()}
			}
		}
		
		// Define graph struct
		struct $graph_name<V,W>
			where
				V: Vertex,
				W: Weight,
		{
			pub graph:
			custom_graph!{
				@in_struct
				$($con_graph,$base_graph<V,W> >>)*
				$base_graph<V,W>
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
			type VertexIter = <$base_graph<V,W> as BaseGraph>::VertexIter;
			type EdgeIter = <$base_graph<V,W> as BaseGraph>::EdgeIter;
		
			fn empty_graph() -> Self{
				$graph_name{ graph:
					custom_graph!{
						@in_empty_graph
						$($con_graph,$base_graph<V,W> >>)*
						$base_graph<V,W>
					}
				}
			}
			wrap!{graph.all_vertices(&self) -> Self::VertexIter}
	
			wrap!{graph.all_edges(&self) -> Self::EdgeIter}
			
			wrap!{graph.add_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>}
			
			wrap!{graph.remove_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>}
			
			wrap!{graph.add_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()>}
			
			wrap!{graph.remove_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()>}
		}
		
		//Impl ConstrainedGraph
		impl<V,W> ConstrainedGraph for 	$graph_name<V,W>
			where
				V: Vertex,
				W: Weight,
		{
			wrap!{graph.invariant_holds(&self) -> bool}
			
			wrap_uncon_methods!{graph}
		}
		
		// Impl the constraint traits
		$(
			impl<V,W> $con_trait for $graph_name<V,W>
				where
					V: Vertex,
					W: Weight,{}
		)*
		
	};
}

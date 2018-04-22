//!
//! Contains the basic traits and structs needed to define graphs and work on them.
//!
//!
//!

///
/// Expands to the body of a Constraint implementation that
/// has no constraints.
///
/// I.e. invariant_holds() always returns `true` and the other functions
/// all call `BaseGraph`'s constrained functions.
///
///
#[macro_export]
macro_rules! impl_base_constraint{
{} => {
	fn invariant_holds(&self) -> bool {
		true
	}
	
	unsafe fn uncon_add_vertex(&mut self, v: Self::Vertex) -> Result<(), ()> {
		self.add_vertex(v)
	}
	
	unsafe fn uncon_remove_vertex(&mut self, v: Self::Vertex) -> Result<(), ()> {
		self.remove_vertex(v)
	}
	
	unsafe fn uncon_add_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()> {
		self.add_edge(e)
	}
	
	unsafe fn uncon_remove_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()> {
		self.remove_edge(e)
	}
}
}
//#[macro_use]
//mod graph_wrapper;
mod base_graph;
mod edge;
mod weights;
//mod constrained_graph;
//mod exact_graph;

//#[macro_use]
//pub mod constraint;


pub use self::{base_graph::*, edge::*, weights::*};
/*
pub use self::constrained_graph::*;
pub use self::graph_wrapper::*;
pub use self::exact_graph::*;
*/
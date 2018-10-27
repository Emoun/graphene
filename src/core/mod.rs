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
pub mod trait_aliases;
mod graph;
mod edge;
mod constrained_graph;


//#[macro_use]
//pub mod constraint;


pub use self::{
	graph::*,
	edge::*,
	constrained_graph::*,
};
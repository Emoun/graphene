
//#[macro_use]
//mod custom_graph;

#[macro_use]
mod impl_constraints;
mod unique;
mod no_loops;
mod reflexive;

pub use self::{
	unique::*,
	no_loops::*,
	reflexive::*,
	impl_constraints::*,
};




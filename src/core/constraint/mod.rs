
//#[macro_use]
//mod custom_graph;
mod unique;
mod no_loops;
mod reflexive;

//pub use self::undirected::*;
pub use self::{
	unique::*,
	no_loops::*,
	reflexive::*,
};




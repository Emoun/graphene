


#[macro_use]
mod impl_constraints;
mod unique;
mod no_loops;
mod reflexive;
mod connected;

pub use self::{
	impl_constraints::*,
	unique::*,
	no_loops::*,
	reflexive::*,
	connected::*,
};


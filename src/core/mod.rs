//!
//! Contains the basic traits and structs needed to define graphs and work on them.
//!
//!
//!
#[macro_use]
mod delegate_graph;
pub mod constraint;
pub mod trait_aliases;
mod graph;
mod edge;
mod directedness;
mod constrainer;
mod reverse_graph;
pub mod proxy;

pub use self::{
	graph::*,
	edge::*,
	directedness::*,
	constrainer::*,
	reverse_graph::*,
};
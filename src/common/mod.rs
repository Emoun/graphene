//! Contains common graph implementations.

mod adjacency_list;
mod ensured;
mod vertex_map;

pub use self::{adjacency_list::*, ensured::*, vertex_map::*};

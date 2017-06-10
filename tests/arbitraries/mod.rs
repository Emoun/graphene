
use graphene::core::*;
use quickcheck::{Arbitrary,Gen};

#[macro_use]
mod utilities;
mod arbitrary_graph_description;

pub use self::arbitrary_graph_description::*;
pub use self::utilities::*;

pub trait ArbVertex: Arbitrary + Vertex{}
impl<T> ArbVertex for T where T: Arbitrary + Vertex{}

pub trait ArbWeight: Arbitrary + Weight{}
impl<T> ArbWeight for T where T: Arbitrary + Weight{}
#![allow(non_snake_case)]
#![allow(unused_imports)]
//#![feature(trace_macros)] //trace_macros!(true);

#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[macro_use]
pub mod mock_graphs;
mod common;
pub mod core;




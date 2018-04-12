#![allow(non_snake_case)]
#![allow(unused_imports)]
//#![feature(trace_macros)] //trace_macros!(true);
#[macro_use]
extern crate graphene;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;
#[macro_use]
extern crate dmutil;


#[macro_use]
mod arbitraries;
mod common;
mod core;

use graphene::core::*;
use graphene::core::constraint::*;
use graphene::common::*;

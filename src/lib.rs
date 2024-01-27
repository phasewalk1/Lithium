#![feature(stmt_expr_attributes)]
#![forbid(unused_allocation, unused_variables, unused_assignments)]
#![deny(dead_code)]

pub mod environment;
pub mod grunt;
pub mod interface;
pub mod primitive;

#[macro_use]
pub mod macros;

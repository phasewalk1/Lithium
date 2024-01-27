#![feature(stmt_expr_attributes)]
#![forbid(unused_allocation, unused_variables, unused_assignments)]
#![deny(dead_code)]

pub mod builtins;
pub mod clause;
pub mod eval;
pub mod identifiers;
pub mod namespace;
pub mod parser;
pub mod primitive;
pub mod reducers;
pub mod token;

#[macro_use]
pub mod macros;

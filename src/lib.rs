#![feature(stmt_expr_attributes)]
pub mod builtins;
pub mod eval;
#[macro_use]
pub mod macros;
pub mod namespace;
pub mod parser;
pub mod prim;
pub mod reducers;
pub mod token;

use lazy_static::lazy_static;
#[rustfmt::skip] lazy_static! {
   pub static ref OPERATORS: namespace::OperatorTable = 
        namespace::OperatorTable::init();
}

#![allow(dead_code)]
#![feature(stmt_expr_attributes)]
pub mod eval;
pub mod token;
pub mod parser;
pub mod prim;
pub mod reducers;
pub mod namespace;


use lazy_static::lazy_static;
#[rustfmt::skip] lazy_static! {
   pub static ref OPERATORS: namespace::OperatorTable = 
        namespace::OperatorTable::init();
}

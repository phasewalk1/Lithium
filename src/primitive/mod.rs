pub mod atom;
pub use atom::Atom;

mod structure;

pub use structure::cell::Cell;
pub use structure::clause as branches;

pub mod function;
pub use function::Function;

pub mod keyword;
pub use keyword::Keyword;

pub mod operator;
pub use operator::{Operator, OperatorFunc};

pub mod symbol;

use std::rc::Rc;

#[derive(PartialEq)]
pub enum Value {
    Nil,
    T,
    Atom(Atom),
    Function(Rc<function::Function>),
    Operator(Rc<operator::Operator>),
    Symbol(crate::environment::identifiers::SymbolId),
    Cell(Cell),
    Keyword(Rc<keyword::Keyword>),
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::T => write!(f, "t"),
            Value::Atom(a) => write!(f, "{:?}", a),
            Value::Function(_) => write!(f, "<function>"),
            Value::Symbol(s) => write!(f, "<sym> {:?}", s),
            Value::Cell(c) => write!(f, "<cell> ({:?} . {:?})", c.car, c.cdr),
            Value::Operator(op) => write!(f, "<op> {:?}@{:?}", op.id, op.f),
            Value::Keyword(kw) => write!(f, "<kw> {:?}", kw.id),
        }
    }
}

impl Value {
    pub fn unwrap_atom(&self) -> atom::Atom {
        match self {
            Value::Atom(atom) => atom::Atom(atom.0),
            _ => panic!("Expected atom, got {:?}", self),
        }
    }
}

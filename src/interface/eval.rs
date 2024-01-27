use super::substitution::{CbvSubstitute, *};
use crate::primitive::{Atom, Cell, Value};

use std::rc::Rc;

pub trait Eval
where
    Self: core::fmt::Debug,
{
    fn eval(&self) -> Value;
}

impl Eval for Atom {
    fn eval(&self) -> Value {
        Value::Atom(Atom(self.0))
    }
}

impl Eval for Cell {
    fn eval(&self) -> Value {
        CbvSubstitute(&self).subs()
    }
}

impl Eval for Value {
    fn eval(&self) -> Value {
        #[rustfmt::skip]
        match self {
            Value::Atom(atom)         => atom.eval(),
            Value::Cell(cell)         => cell.eval(),
            Value::Nil                => Value::Nil,
            Value::T                  => Value::T,
            Value::Function(function) => Value::Function(Rc::clone(function)),
            Value::Symbol(_)          => todo!(),
            Value::Operator(op)       => Value::Operator(Rc::clone(op)),
            Value::Keyword(kw)        => Value::Keyword(Rc::clone(kw)),
        }
    }
}

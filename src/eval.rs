use crate::prim::{
    Atom,
    Cell,
    Value,
};

use std::rc::Rc;

pub trait Eval {
    fn eval(&self) -> Value;
}

impl Eval for Atom {
    fn eval(&self) -> Value {
        Value::Atom(Atom(self.0))
    }
}

impl Eval for Cell {
    fn eval(&self) -> Value {
        crate::reducers::BetaReducer::new(&self)
            .reduce_cbv()
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
        }
    }
}

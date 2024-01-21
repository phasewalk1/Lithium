use crate::primitives::{
    Atom,
    Cell,
    Function,
    Nil,
    SymbolicId,
    Value,
};

pub(super) trait Eval {
    fn eval(&self) -> Value;
}

impl Eval for Atom {
    fn eval(&self) -> Value {
        Value::Atom(Atom(self.0))
    }
}

impl Eval for Cell {
    fn eval(&self) -> Value {
        todo!()
    }
}

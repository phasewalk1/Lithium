use crate::prim::{
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
    #[rustfmt::skip]
    fn eval(&self) -> Value {
        crate::reducers::BetaReducer
            ::new(&self).reduce_cbv()
    }
}

impl Eval for Function {
    fn eval(&self) -> Value {
        todo!()
    }
}

impl Eval for Nil {
    fn eval(&self) -> Value {
        Value::Nil
    }
}

impl Eval for Value {
    fn eval(&self) -> Value {
        match self {
            Value::Cell(cell) => {
                // beta-reduction
                todo!()
            }
            Value::Function(function) => {
                // eta-reduction
                todo!()
            }
            Value::Symbol(symbol) => {
                // alpha-reduction
                todo!()
            }
            Value::Atom(atom) => atom.eval(),
            Value::Nil => Value::Nil,
        }
    }
}

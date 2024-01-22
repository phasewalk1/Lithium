use crate::eval::Eval;
use crate::prim::{Cell, Value};

use std::rc::Rc;

#[rustfmt::skip]
pub(super) struct BetaReducer<'a>
    (pub &'a Cell);

impl<'a> BetaReducer<'a> {
    pub(super) fn new(cell: &'a Cell) -> Self {
        Self(cell)
    }

    pub(super) fn cell(&self) -> &'a Cell {
        self.0
    }

    // Call-by-value
    pub(super) fn cbv(&self) -> Value {
        let cell = self.cell();
        let (car, cdr) = cell.disassemble();

        match car.eval() {
            Value::Function(func) => {
                let args = cdr
                    .iter()
                    .map(|v| Rc::new(v.eval()))
                    .collect::<Vec<Rc<Value>>>();
                func.apply(&args)
            }
            Value::Operator(op) => {
                let args = cdr
                    .iter()
                    .map(|v| Rc::new(v.eval()))
                    .collect::<Vec<Rc<Value>>>();
                log::debug!("Applying operator {:?} to {:?}", op, args);
                op.apply(&args)
            }
            Value::Nil => Value::Nil,
            Value::Atom(atom) => Value::Atom(atom),
            Value::Symbol(symbol) => Value::Symbol(symbol),
            _ => panic!("Cannot beta-reduce {:?} value", car.eval()),
        }
    }
}

use crate::eval::Eval;
use crate::primitive::{Cell, Value};

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
                assert!(
                    args.len() == 2,
                    "Operator {:?} expects 2 arguments, got {}",
                    op,
                    args.len()
                );
                let a = args[0].clone().unwrap_atom();
                let b = args[1].clone().unwrap_atom();
                op.apply(a, b)
            }
            Value::Nil => Value::Nil,
            Value::T => Value::T,
            Value::Atom(atom) => Value::Atom(atom),
            Value::Symbol(symbol) => Value::Symbol(symbol),
            Value::Keyword(kw) => {
                if kw.id == "if" {
                    if crate::clause::is_conditional(&cell) {
                        log::debug!("Found cond body {:?} as conditional", cell);
                        let cond = crate::clause::downcast_conditional(&cell);
                        log::debug!("Downcasted cell to cond {:?}", cond);
                        return cond.eval();
                    } else {
                        log::debug!("Cannot evaluate cond body {:?} as conditional", cell);
                    }
                } else {
                    log::debug!("Cannot evaluate keyword {:?} as value", kw.id);
                }
                Value::Keyword(kw)
            }
            _ => panic!("Cannot beta-reduce {:?} value", car.eval()),
        }
    }
}

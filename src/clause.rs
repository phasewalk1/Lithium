use std::rc::Rc;

use crate::{
    eval::Eval,
    primitive::{Atom, Cell, Value},
};

pub fn is_conditional(cell: &Cell) -> bool {
    match cell.car.eval() {
        Value::Keyword(kw) => {
            if kw.id == "if" {
                log::debug!("Found if keyword");
                match &cell.cdr as &Value {
                    Value::Cell(cdr) => true,
                    _ => {
                        log::warn!("Cannot evaluate {:?} as conditional", cell);
                        false
                    }
                }
            } else {
                log::warn!("kw.id is not 'if': {:?}", kw.id);
                false
            }
        }
        Value::Atom(x) => x.cond(),
        _ => false,
    }
}

pub fn downcast_conditional(cell: &Cell) -> IfClause {
    assert!(is_conditional(&cell));
    let (_, rest) = cell.disassemble();
    let cond = Rc::clone(&rest[0]);
    let then = Rc::clone(&rest[1]);
    let else_ = rest.get(2).map(|v| Rc::clone(v));
    IfClause { cond, then, else_ }
}

pub trait Conditional {
    fn cond(&self) -> bool;
}

impl Conditional for Value {
    fn cond(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::T => true,
            _ => panic!("Cannot evaluate {:?} as condition", self),
        }
    }
}

impl Conditional for Atom {
    fn cond(&self) -> bool {
        match self {
            Atom(0) => false,
            Atom(_) => true,
        }
    }
}

#[derive(Debug)]
pub struct IfClause {
    pub cond: Rc<Value>,
    pub then: Rc<Value>,
    pub else_: Option<Rc<Value>>,
}

impl Eval for IfClause {
    fn eval(&self) -> Value {
        let pred = self.cond.eval().cond();
        log::debug!("Evaluated conditional {:?} to {:?}", self.cond, pred);

        match pred {
            true => self.then.eval(),
            _ => self
                .else_
                .as_ref()
                .map_or(Value::Nil, |else_clause| else_clause.eval()),
        }
    }
}

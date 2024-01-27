use super::eval::Eval;
use crate::primitive::{branches, Cell, Function, Keyword, Operator, Value};

use std::rc::Rc;

pub trait SubstitutionStrategy {
    fn subs(&self) -> Value;
}

#[rustfmt::skip]
pub(super) struct CbvSubstitute<'a>
    (pub &'a Cell);

impl CbvSubstitute<'_> {
    fn handle_func(func: &Rc<Function>, cdr: Vec<Rc<Value>>) -> Value {
        func.apply(&cdr)
    }

    fn handle_operator(op: &Rc<Operator>, cdr: Vec<Rc<Value>>) -> Value {
        let args = Self::args_eager(cdr);
        assert_guards::operator::good_args(op, &args);

        let a = args[0].clone().unwrap_atom();
        let b = args[1].clone().unwrap_atom();
        op.apply(a, b)
    }

    fn args_eager(cdr: Vec<Rc<Value>>) -> Vec<Rc<Value>> {
        cdr.iter().map(|v| Rc::new(v.eval())).collect()
    }

    fn handle_keyword(kw: &Rc<Keyword>, cell: &Cell) -> Value {
        if kw.id == "if" {
            if branches::is_conditional(&cell) {
                log::debug!("Found cond body {:?} as conditional", cell);
                let cond = branches::downcast_conditional(&cell);
                log::debug!("Downcasted cell to cond {:?}", cond);
                return cond.eval();
            } else {
                log::warn!("Cannot evaluate cond body {:?} as conditional", cell);
            }
        } else {
            log::warn!("Cannot evaluate keyword {:?} as value", kw.id);
        }
        Value::Keyword(Rc::clone(kw))
    }
}

impl<'a> SubstitutionStrategy for CbvSubstitute<'a> {
    fn subs(&self) -> Value {
        let cell = self.0;
        let (car, cdr) = cell.disassemble();

        #[rustfmt::skip]
        match car.eval() {
            Value::Nil            => Value::Nil,
            Value::T              => Value::T,
            Value::Atom(atom)     => Value::Atom(atom),
            Value::Symbol(symbol) => Value::Symbol(symbol),
            Value::Function(func) => Self::handle_func(&func, cdr),
            Value::Operator(op)   => Self::handle_operator(&op, cdr),
            Value::Keyword(kw)    => Self::handle_keyword(&kw, cell),

            _ => panic!("Cannot beta-reduce {:?} value", car.eval()),
        }
    }
}

pub mod assert_guards {
    pub mod operator {
        use crate::primitive::Operator;

        use super::super::{Rc, Value};

        pub fn good_args(op: &Rc<Operator>, args: &[Rc<Value>]) -> () {
            assert!(
                args.len() == 2,
                "Operator {:?} expects 2 arguments, got {}",
                op,
                args.len()
            );
        }
    }
}

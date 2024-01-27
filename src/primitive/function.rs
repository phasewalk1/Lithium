use crate::environment::identifiers::SymbolId;
use crate::primitive::Value;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Function {
    id: SymbolId,
    f: fn(&[Rc<Value>]) -> Value,
}

impl Function {
    #[allow(dead_code)]
    pub(super) fn new(id: SymbolId, f: fn(&[Rc<Value>]) -> Value) -> Self {
        Self { id, f }
    }

    pub(crate) fn apply(&self, args: &[std::rc::Rc<Value>]) -> Value {
        (self.f)(args)
    }
}

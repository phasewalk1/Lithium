use crate::primitive::{atom::Atom, Value};

#[derive(Debug, Clone)]
pub struct OperatorFunc(pub fn(Atom, Atom) -> Value);

impl From<fn(Atom, Atom) -> Value> for OperatorFunc {
    fn from(f: fn(Atom, Atom) -> Value) -> Self {
        Self(f)
    }
}

#[derive(Clone, Debug)]
pub struct Operator {
    pub(crate) id: u8,
    pub(crate) f: OperatorFunc,
}

impl std::cmp::PartialEq for Operator {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Operator {
    pub fn new(id: u8, f: fn(Atom, Atom) -> Value) -> Self {
        Operator { id, f: f.into() }
    }

    pub(crate) fn apply(&self, a: Atom, b: Atom) -> Value {
        (self.f.0)(a, b)
    }
}

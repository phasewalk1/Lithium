use crate::primitive::{Atom, Value};

pub(crate) fn add(a: Atom, b: Atom) -> Value {
    Value::Atom(Atom(a.0 + b.0))
}

pub(crate) fn sub(a: Atom, b: Atom) -> Value {
    Value::Atom(Atom(a.0 - b.0))
}

pub(crate) fn mul(a: Atom, b: Atom) -> Value {
    Value::Atom(Atom(a.0 * b.0))
}

pub(crate) fn div(a: Atom, b: Atom) -> Value {
    Value::Atom(Atom(a.0 / b.0))
}

pub(crate) fn eq(a: Atom, b: Atom) -> Value {
    if a == b {
        Value::T
    } else {
        Value::Nil
    }
}

pub(crate) fn ge(a: Atom, b: Atom) -> Value {
    if a >= b {
        Value::T
    } else {
        Value::Nil
    }
}

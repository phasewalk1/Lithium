use crate::primitive::{Atom, Value};
use std::{collections::HashMap, rc::Rc};

impl Default for crate::namespace::Environment {
    fn default() -> Self {
        let mut operators = HashMap::new();
        let mut keywords = HashMap::new();

        crate::load_builtin_ops!(operators,
            b'+' => crate::builtins::add,
            b'-' => crate::builtins::sub,
            b'*' => crate::builtins::mul,
            b'/' => crate::builtins::div,
            b'=' => crate::builtins::eq,
            b'>' => crate::builtins::ge,
        );
        crate::load_builtin_keywords!(keywords, String::from("if") => ());

        Self {
            operators,
            keywords,
        }
    }
}

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

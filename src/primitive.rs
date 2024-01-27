use crate::identifiers::SymbolId;
use std::rc::Rc;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Atom(pub i32);

impl Atom {
    pub fn wrap(&self) -> Value {
        Value::Atom(Atom(self.0))
    }
}

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

    pub(super) fn apply(&self, args: &[std::rc::Rc<Value>]) -> Value {
        (self.f)(args)
    }
}

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

    pub(super) fn apply(&self, a: Atom, b: Atom) -> Value {
        (self.f.0)(a, b)
    }
}

#[derive(Debug, PartialEq)]
pub(super) struct Symbol {
    id: SymbolId,
    value: Value,
}

#[derive(Debug, PartialEq)]
pub struct Cell {
    pub(super) car: Rc<Value>,
    pub(super) cdr: Rc<Value>,
}

impl Cell {
    #[allow(dead_code)]
    pub(super) fn list(car: Value, cdr: Value) -> Self {
        let tail = Value::Nil;
        Self {
            car: Rc::new(car),
            cdr: Rc::new(Value::Cell(Cell {
                car: Rc::new(cdr),
                cdr: Rc::new(tail),
            })),
        }
    }

    pub(super) fn cons(car: Value, cdr: Value) -> Self {
        Self {
            car: Rc::new(car),
            cdr: Rc::new(cdr),
        }
    }

    pub(super) fn from_vec(elements: Vec<Rc<Value>>) -> Self {
        let mut tail = Value::Nil;

        for elem in elements.into_iter().rev() {
            tail = Value::Cell(Cell::cons(Rc::try_unwrap(elem).unwrap(), tail));
        }

        if let Value::Cell(cell) = tail {
            cell
        } else {
            unreachable!()
        }
    }

    pub(super) fn disassemble(&self) -> (Rc<Value>, Vec<Rc<Value>>) {
        let first = Rc::clone(&self.car);
        let mut rest = vec![];
        let mut curr = self;

        while let Value::Cell(ref cell) = *curr.cdr {
            rest.push(Rc::clone(&cell.car));
            curr = &cell;
        }

        (first, rest)
    }
}

#[derive(Debug)]
pub struct Keyword {
    pub id: String,
}

impl std::cmp::PartialEq for Keyword {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(PartialEq)]
pub enum Value {
    Nil,
    T,
    Atom(Atom),
    Function(Rc<Function>),
    Operator(Rc<Operator>),
    Symbol(SymbolId),
    Cell(Cell),
    Keyword(Rc<Keyword>),
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::T => write!(f, "t"),
            Value::Atom(a) => write!(f, "{:?}", a),
            Value::Function(_) => write!(f, "<function>"),
            Value::Symbol(s) => write!(f, "<sym> {:?}", s),
            Value::Cell(c) => write!(f, "<cell> ({:?} . {:?})", c.car, c.cdr),
            Value::Operator(op) => write!(f, "<op> {:?}@{:?}", op.id, op.f),
            Value::Keyword(kw) => write!(f, "<kw> {:?}", kw.id),
        }
    }
}

impl Value {
    pub fn unwrap_atom(&self) -> Atom {
        match self {
            Value::Atom(atom) => Atom(atom.0),
            _ => panic!("Expected atom, got {:?}", self),
        }
    }
}

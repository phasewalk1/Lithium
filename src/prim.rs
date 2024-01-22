use crate::eval::Eval;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Atom(pub i32);

pub trait Identifier {
    fn id(&self) -> Vec<u8>;
}

#[derive(Debug, PartialEq)]
pub struct SymbolId([u8; 32]);

impl TryFrom<&str> for SymbolId {
    type Error = Box<dyn std::error::Error>;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.len() {
            0..=32 => {
                let mut id = [0; 32];
                id[..s.len()].copy_from_slice(s.as_bytes());
                Ok(Self(id))
            }
            _ => Err("SymbolId must be 32 bytes or less".into()),
        }
    }
}

impl Identifier for SymbolId {
    fn id(&self) -> Vec<u8> {
        self.0.to_vec()
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

impl Identifier for u8 {
    fn id(&self) -> Vec<u8> {
        vec![*self]
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Operator {
    pub(crate) id: u8,
    pub(crate) f: fn(&[Rc<Value>]) -> Value,
}

impl Operator {
    pub fn new(id: u8, f: fn(&[Rc<Value>]) -> Value) -> Self {
        Operator { id, f }
    }

    pub(super) fn apply(&self, args: &[std::rc::Rc<Value>]) -> Value {
        (self.f)(args)
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

#[allow(dead_code)]
impl Cell {
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

    pub(super) fn reduce_args(&self) -> Vec<Rc<Value>> {
        let mut args = vec![];
        let mut curr = self;

        while let Value::Cell(ref cell) = *curr.cdr {
            args.push(Rc::new(cell.car.eval()));
            curr = &cell;
        }

        args
    }

    pub(super) fn car(&self) -> &Value {
        &self.car
    }

    pub(super) fn cdr(&self) -> &Value {
        &self.cdr
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

#[derive(PartialEq)]
pub enum Value {
    Nil,
    T,
    Atom(Atom),
    Function(Rc<Function>),
    Operator(Rc<Operator>),
    Symbol(SymbolId),
    Cell(Cell),
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
        }
    }
}

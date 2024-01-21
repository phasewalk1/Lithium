use crate::eval::Eval;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub(super) struct Atom(pub i32);

#[derive(PartialEq)]
pub(super) struct Nil(());

#[derive(Debug, PartialEq)]
pub(super) struct SymbolicId([u8; 32]);

impl SymbolicId {
    pub(super) fn make_symbolic_id(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        match s.len() {
            0..=32 => {
                let mut id = [0; 32];
                id[..s.len()].copy_from_slice(s.as_bytes());
                Ok(Self(id))
            }
            _ => Err("SymbolicId must be 32 bytes or less".into()),
        }
    }

    pub(super) fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<&str> for SymbolicId {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() > 32 {
            Err("SymbolicId must be 32 bytes or less")
        } else {
            let mut id = [0; 32];
            id[..s.len()].copy_from_slice(s.as_bytes());
            Ok(Self(id))
        }
    }
}

#[derive(Debug, PartialEq)]
pub(super) struct Function {
    sid: Option<SymbolicId>,
    oid: Option<u8>,
    f: fn(&[Rc<Value>]) -> Value,
}

impl Function {
    pub(super) fn new(id: SymbolicId, f: fn(&[Rc<Value>]) -> Value) -> Self {
        Self {
            sid: Some(id),
            oid: None,
            f,
        }
    }

    pub(super) fn new_operator(id: u8, f: fn(&[Rc<Value>]) -> Value) -> Self {
        Self {
            sid: None,
            oid: Some(id),
            f,
        }
    }

    pub(super) fn apply(&self, args: &[std::rc::Rc<Value>]) -> Value {
        (self.f)(args)
    }
}

#[derive(Debug, PartialEq)]
pub(super) struct Symbol {
    id: SymbolicId,
    value: Value,
}

#[derive(Debug, PartialEq)]
pub(super) struct Cell {
    pub(super) car: Rc<Value>,
    pub(super) cdr: Rc<Value>,
}

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
            tail = Value::Cell(Cell::cons(
                Rc::try_unwrap(elem).unwrap(), tail
            ));
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
pub(super) enum Value {
    Nil,
    Atom(Atom),
    Function(Function),
    Symbol(SymbolicId),
    Cell(Cell),
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Atom(a) => write!(f, "{:?}", a),
            Value::Function(_) => write!(f, "<function>"),
            Value::Symbol(s) => write!(f, "<sym> {:?}", s),
            Value::Cell(c) => write!(f, "<cell> ({:?} . {:?})", c.car, c.cdr),
        }
    }
}
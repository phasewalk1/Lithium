use crate::primitive::Value;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Cell {
    pub(crate) car: Rc<Value>,
    pub(crate) cdr: Rc<Value>,
}

impl Cell {
    pub(crate) fn from_vec(elements: Vec<Rc<Value>>) -> Self {
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

    pub(crate) fn disassemble(&self) -> (Rc<Value>, Vec<Rc<Value>>) {
        let first = Rc::clone(&self.car);
        let mut rest = vec![];
        let mut curr = self;

        while let Value::Cell(ref cell) = *curr.cdr {
            rest.push(Rc::clone(&cell.car));
            curr = &cell;
        }

        (first, rest)
    }

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
}

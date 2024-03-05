#[derive(Debug, PartialEq, Clone)]
pub struct Cell {
    car: Box<Expr>,
    cdr: Option<Box<Expr>>,
}

impl Cell {
    pub fn new(car: Expr, cdr: Option<Box<Expr>>) -> Cell {
        Cell {
            car: Box::new(car),
            cdr,
        }
    }

    fn car(&self) -> Expr {
        *(self.car.clone())
    }

    fn cdr(&self) -> Option<Expr> {
        self.cdr.clone().map(|e| *e)
    }
}

pub(crate) fn vec2cell(exprs: Vec<Expr>) -> Option<Box<Expr>> {
    let mut iter = exprs.into_iter().rev();
    let first = iter.next().map(Box::new);

    iter.fold(first, |acc, expr| {
        Some(Box::new(Expr::List(Cell::new(expr, acc))))
    })
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Atom(u32),
    Symbol(String),
    List(Cell),
}

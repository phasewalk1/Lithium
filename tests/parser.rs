#![cfg(test)]

use lithium::core::*;
use lithium::parser::*;
use nom::IResult;

fn assert_ok(result: IResult<&str, Expr>, expected: Expr) {
    assert_eq!(result, Ok(("", expected)));
}

#[test]
fn test_lists() {
    assert_ok(
        cell("(1 2 3)"),
        Expr::List(Cell::new(
            Expr::Atom(1),
            Some(Box::new(Expr::List(Cell::new(
                Expr::Atom(2),
                Some(Box::new(Expr::Atom(3))),
        )))),
    )));

    assert_ok(
        cell("(+ 2 3)"),
        Expr::List(Cell::new(
            Expr::Symbol("+".to_string()),
            Some(Box::new(Expr::List(Cell::new(
                Expr::Atom(2),
                Some(Box::new(Expr::Atom(3))),
        )))),
    )));
}

#[test]
fn test_atom() {
    assert_eq!(number("123"), Ok(("", Expr::Atom(123))));
}

#[test]
fn test_symbol() {
    assert_eq!(symbol("abc"), Ok(("", Expr::Symbol("abc".to_string()))));
    assert_eq!(symbol("+"), Ok(("", Expr::Symbol("+".to_string()))));
}


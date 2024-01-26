use crate::prim::{Atom, Cell, Value};
use crate::token::Token;
use crate::OPERATORS;
use std::iter::Peekable;
use std::rc::Rc;
use std::slice::Iter;

#[derive(Debug)]
pub struct Parser;

impl Parser {
    pub fn parse(tokens: &[Token]) -> Result<Value, String> {
        let mut iter = tokens.iter().peekable();
        let parsed = Self::parse_expression(&mut iter)?;

        if iter.peek().is_some() {
            Err("Unexpected tokens after parsing".into())
        } else {
            Ok(parsed)
        }
    }

    fn parse_expression(iter: &mut Peekable<Iter<Token>>) -> Result<Value, String> {
        match iter.next() {
            Some(Token::Atom(n)) => Ok(Value::Atom(Atom(*n))),
            Some(Token::Laren) => Self::parse_list(iter),
            Some(Token::Raren) => Err("Unexpected ')'".into()),
            Some(Token::Operator(op)) => Self::parse_operator(op),
            Some(Token::Keyword(_)) => unimplemented!(),
            Some(Token::Symbol(_)) => unimplemented!(),
            None => Err("Unexpected end of input".into()),
        }
    }

    fn parse_list(iter: &mut Peekable<Iter<Token>>) -> Result<Value, String> {
        let mut elements = Vec::new();

        while iter.peek().map_or(false, |token| **token != Token::Raren) {
            elements.push(Rc::new(Self::parse_expression(iter)?));
        }

        if iter.next().is_none() {
            return Err("Missing ')'".into());
        }

        Ok(Value::Cell(Cell::from_vec(elements)))
    }

    fn parse_operator(id: &u8) -> Result<Value, String> {
        match OPERATORS.get(id) {
            #[rustfmt::skip]
            Some(operator) => {
                Ok(Value::Operator(
                        operator.clone().into()))
            },
            None => Err("Unknown operator".into()),
        }
    }
}

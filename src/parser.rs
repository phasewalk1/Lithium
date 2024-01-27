use crate::clause as branch;
use crate::namespace::Environment;
use crate::primitive::{Atom, Cell, Value};
use crate::token::Token;
use std::iter::Peekable;
use std::rc::Rc;
use std::slice::Iter;

#[derive(Debug)]
pub struct Parser {
    pub namespace: Environment,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            namespace: Environment::default(),
        }
    }
}

impl Parser {
    pub fn parse(&self, tokens: &[Token]) -> Result<Value, String> {
        let mut iter = tokens.iter().peekable();
        let parsed = self.parse_expression(&mut iter)?;

        if let Some(c) = iter.peek() {
            if c == &&Token::Newline {
                return Ok(parsed);
            }
            Err("Unexpected tokens after parsing".into())
        } else {
            Ok(parsed)
        }
    }

    fn parse_expression(&self, iter: &mut Peekable<Iter<Token>>) -> Result<Value, String> {
        match iter.next() {
            Some(Token::Atom(n)) => Ok(Value::Atom(Atom(*n))),
            Some(Token::Laren) => self.parse_list(iter),
            Some(Token::Raren) => Err("Unexpected ')'".into()),
            Some(Token::Operator(op)) => self.parse_operator(op),
            Some(Token::Keyword(kw)) => self.parse_keyword(kw),
            Some(Token::Newline) => self.parse_expression(iter),
            Some(Token::Symbol(_)) => unimplemented!(),
            None => Err("Unexpected end of input".into()),
        }
    }

    fn parse_list(&self, iter: &mut Peekable<Iter<Token>>) -> Result<Value, String> {
        let mut elements = Vec::new();

        while iter.peek().map_or(false, |token| **token != Token::Raren) {
            elements.push(Rc::new(self.parse_expression(iter)?));
        }

        if iter.next().is_none() {
            return Err("Missing ')'".into());
        }

        Ok(Value::Cell(Cell::from_vec(elements)))
    }

    fn parse_operator(&self, id: &u8) -> Result<Value, String> {
        match self.namespace.operators.get(id) {
            #[rustfmt::skip]
            Some(operator) => {
                Ok(Value::Operator(
                        operator.clone().into()))
            },
            None => Err("Unknown operator".into()),
        }
    }

    fn parse_keyword(&self, id: &str) -> Result<Value, String> {
        match self.namespace.keywords.get(id) {
            #[rustfmt::skip]
            Some(keyword) => {
                Ok(Value::Keyword(
                        keyword.clone().into()))
            },
            None => Err("Unknown keyword".into()),
        }
    }
}

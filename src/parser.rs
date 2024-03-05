use crate::core::{Expr, vec2cell};
use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{char, multispace1},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

pub const OPERATOR_RUNES: [char; 4] = ['+', '-', '*', '/'];

pub fn is_symbol_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || OPERATOR_RUNES.contains(&c)
}

pub fn number(input: &str) -> IResult<&str, Expr> {
    let (input, num) = take_while1(|c: char| c.is_ascii_digit())(input)?;
    Ok((input, Expr::Atom(num.parse().unwrap())))
}

pub fn symbol(input: &str) -> IResult<&str, Expr> {
    let (input, sym) = take_while1(is_symbol_char)(input)?;
    Ok((input, Expr::Symbol(sym.to_string())))
}

pub fn cell(input: &str) -> IResult<&str, Expr> {
    let parser = separated_list1(multispace1, expr);
    let (input, exprs) = delimited(char('('), parser, char(')'))(input)?;

    match vec2cell(exprs) {
        Some(cell) => Ok((input, *cell)),
        None => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        ))),
    }
}

pub fn expr(input: &str) -> IResult<&str, Expr> {
    alt((number, symbol, cell))(input)
}

use crate::OPERATORS;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Atom(i32),
    Operator(u8),
}

#[derive(Default)]
pub struct Tokenizer;

impl Tokenizer {
    pub fn tokenize(input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut atom_buffer = None;

        for c in input.chars() {
            match c {
                '0'..='9' => Self::push_atom_digit_front(c, &mut atom_buffer),
                '(' => tokens.push(Token::LParen),
                ')' => {
                    if let Some(n) = atom_buffer.take() {
                        tokens.push(Token::Atom(n));
                    }
                    tokens.push(Token::RParen);
                }
                c if Self::is_valid_operator(c as u8) => {
                    if let Some(operator) = OPERATORS.get(&(c as u8)) {
                        tokens.push(Token::Operator(operator.id))
                    } else {
                        log::warn!("<tokenizer> unknown operator: {}", c);
                    }
                }
                _ => {
                    if let Some(n) = atom_buffer.take() {
                        tokens.push(Token::Atom(n));
                    }
                }
            }
        }

        tokens
    }

    fn push_atom_digit_front(c: char, current_number: &mut Option<i32>) {
        let digit = c.to_digit(10).unwrap() as i32;
        *current_number = Some(current_number.unwrap_or(0) * 10 + digit);
    }

    fn is_valid_operator(op: u8) -> bool {
        if OPERATOR_RUNES.contains(&op) {
            return true;
        }
        false
    }
}

pub const OPERATOR_RUNES: [u8; 12] = [
    b'+', b'-', b'*', b'/', b'%', b'^', b'&', b'|', b'=', b'!', b'>', b'<',
];

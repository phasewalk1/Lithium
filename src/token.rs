#[derive(Debug, Clone, PartialEq)]
pub(super) enum Token {
    LParen,
    RParen,
    Atom(i32),
    Operator(u8),
}

#[derive(Default)]
pub(super) struct Tokenizer;

impl Tokenizer {
  pub(super) fn tokenize(&self, input: &str) -> Vec<Token> {
    input.chars().filter_map(|c| {
    #[rustfmt::skip] match c {
      '('                         => Some(Token::LParen),
      ')'                         => Some(Token::RParen),
      '+' | '-' | '*' | '/' | 'λ' => Some(Token::Operator(c as u8)),
      '0'..='9'                   => c.to_digit(10).map(|n| Token::Atom(n as i32)),
      _                           => None,
      }}).collect()
    }
}

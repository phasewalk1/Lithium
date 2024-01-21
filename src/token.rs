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
        let mut tokens = Vec::new();
        let mut atom_buffer = None;

        for c in input.chars() {
            #[rustfmt::skip] match c {
                '0'..='9' => Self::push_atom_digit_front(c, &mut atom_buffer),
                _         => Self::push_non_atom(&c, &mut atom_buffer, &mut tokens),
            }
        }

        if let Some(num) = atom_buffer {
            tokens.push(Token::Atom(num));
        }

        tokens
    }

    fn push_atom_digit_front(c: char, current_number: &mut Option<i32>) {
        let digit = c.to_digit(10).unwrap() as i32;
        *current_number = Some(current_number.unwrap_or(0) * 10 + digit);
    }

    #[rustfmt::skip]
    fn push_non_atom(c: &char, atom_buffer: &mut Option<i32>, tokens: &mut Vec<Token>) {
        if let Some(num) = atom_buffer.take() {
            tokens.push(Token::Atom(num));
        } match c {
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '+' | '-' | '*' | '/' | 'λ' => tokens.push(Token::Operator(*c as u8)),
            _ => {}
        }
    }
}

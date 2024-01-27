use crate::primitive::Atom;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Laren,
    Raren,
    Atom(i32),
    Operator(u8),
    Keyword(String),
    Symbol(String),
    Newline,
}

#[rustfmt::skip]
pub const OPERATOR_RUNES: [
    u8; 12
] = [b'+', b'-', b'*', b'/', b'%', b'^', b'&', b'|', b'=', b'!', b'>', b'<',];

#[rustfmt::skip]
pub const KEYWORDS: [
    &str; 2
] = ["nil", "if"];

pub fn operatorp(r: u8) -> bool {
    OPERATOR_RUNES.contains(&r)
}

pub fn keywordp(s: &str) -> bool {
    KEYWORDS.contains(&s)
}

pub fn tokenize(src: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut buf = Buffers::default();

    let src = src
        .lines()
        .filter(|line| !line.starts_with(";;"))
        .collect::<Vec<_>>()
        .join("\n");

    if src.is_empty() {
        return tokens;
    }

    for c in src.chars() {
        #[rustfmt::skip]
        match c {
            '0'..='9'               => buf.atomize(c),
            '('                     => buf.push_take(Token::Laren, &mut tokens),
            ')'                     => buf.push_take(Token::Raren, &mut tokens),
            c if operatorp(c as u8) => tokens.push(Token::Operator(c as u8)),
            ' '                     => buf.take_buffers(&mut tokens),
            '\n'                    => { tokens.push(Token::Newline); buf.take_buffers(&mut tokens); },
            _                       => buf.keyword.push(c),
        }
    }
    buf.take_buffers(&mut tokens);
    tokens
}

#[derive(Debug, Default)]
struct Buffers {
    pub atom: Option<i32>,
    pub keyword: String,
}

impl Buffers {
    pub fn atomize(&mut self, c: char) {
        let digit = c.to_digit(10).unwrap() as i32;
        self.atom = Some(self.atom.unwrap_or(0) * 10 + digit);
    }
    pub fn take_buffers(&mut self, tokens: &mut Vec<Token>) {
        if let Some(atom) = self.atom.take() {
            tokens.push(Token::Atom(atom));
        }
        if keywordp(&self.keyword) {
            tokens.push(Token::Keyword(self.keyword.to_owned()));
            self.keyword.clear();
        }
    }
    pub fn push_take(&mut self, token: Token, tokens: &mut Vec<Token>) {
        self.take_buffers(tokens);
        tokens.push(token);
    }
}

impl Token {
    pub fn is_atom(&self) -> bool {
        match self {
            Token::Atom(_) => true,
            _ => false,
        }
    }
    pub fn unwrap_atom(&self) -> Atom {
        match self {
            Token::Atom(a) => Atom(*a),
            _ => panic!("unwrap_atom called on non-atom token"),
        }
    }
}

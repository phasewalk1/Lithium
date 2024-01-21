#![feature(stmt_expr_attributes)]

mod eval;
mod token;
use token::Tokenizer;
mod primitives;
#[allow(unused_imports)]
use primitives::{Atom, Function, Nil, SymbolicId};

use rustyline::error::ReadlineError;
fn main() -> Result<(), ReadlineError> {
    let tokenizer = Tokenizer::default();
    let mut rl = rustyline::DefaultEditor::new()?;

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let tokens = tokenizer.tokenize(&line);
                println!("{:?}", tokens);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

use lilith::{
    token::Tokenizer,
    parser::Parser,
    eval::Eval,
    namespace,
};
use rustyline::error::ReadlineError;

use lazy_static::lazy_static;
#[rustfmt::skip] lazy_static! {
   pub static ref OPERATORS: namespace::OperatorTable = 
        namespace::OperatorTable::init();
}

fn main() -> Result<(), ReadlineError> {
    pretty_env_logger::init();
    let mut rl = rustyline::DefaultEditor::new()?;

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let tokens = Tokenizer::tokenize(&line);
                log::info!("Got tokens: {:?}", tokens);

                let expr = match Parser::parse(&tokens) {
                    Ok(value) => { log::info!("Built Abstract-Syntax-Tree: {:?}", value); value }
                    Err(err)  => { panic!("Failed to parse expr error: {:?}", err); },
                };

                let evaluated = expr.eval();

                log::info!("Beta-Normal Form ---> {:?}", evaluated);
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

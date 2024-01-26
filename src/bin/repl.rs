use lithium::{eval::Eval, namespace, parser::Parser, token::tokenize};
use rustyline::error::ReadlineError;

fn with_logs() {
    std::env::set_var("RUST_LOG", "none,lithium=debug");
    pretty_env_logger::init_custom_env("RUST_LOG");
}

fn main() -> Result<(), ReadlineError> {
    #[rustfmt::skip]
    #[cfg(debug_assertions)] with_logs();
    let mut rl = rustyline::DefaultEditor::new()?;

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let tokens = tokenize(&line);
                log::info!("Got tokens: {:?}", tokens);

                let expr = match Parser::parse(&tokens) {
                    Ok(value) => {
                        log::info!("Built Abstract-Syntax-Tree: {:?}", value);
                        value
                    }
                    Err(err) => {
                        panic!("Failed to parse expr error: {:?}", err);
                    }
                };

                let evaluated = expr.eval();

                log::info!("Value ---> {:?}", evaluated);
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

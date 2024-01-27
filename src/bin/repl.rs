use lithium::{eval::Eval, parser::Parser, token::tokenize};
use rustyline::error::ReadlineError;

fn with_logs(level: &str) {
    let env_str = format!("none,lithium={}", level);
    std::env::set_var("RUST_LOG", &env_str);
    pretty_env_logger::init_custom_env("RUST_LOG");
}

fn load_script(path: &str) {
    let contents = std::fs::read_to_string(path).unwrap();
    let tokens = tokenize(&contents);
    let parser = Parser::default();
    let expr = parser.parse(&tokens).unwrap();
    let evaluated = expr.eval();
    log::info!("{:?}", evaluated);
}

fn load_check() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() > 1 && args[1] == "load" {
        let path = &args[2];
        load_script(path);
    }
    Ok(())
}

fn main() -> Result<(), ReadlineError> {
    #[cfg(debug_assertions)]
    with_logs("debug");

    if let Ok(_) = load_check() {
        return Ok(());
    }

    let mut rl = rustyline::DefaultEditor::new()?;
    let parser = Parser::default();

    log::debug!("Starting REPL");
    log::debug!("Lithium: {:?}", parser.namespace);

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let tokens = tokenize(&line);
                log::info!("Got tokens: {:?}", tokens);

                let expr = match parser.parse(&tokens) {
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

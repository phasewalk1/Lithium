use rustyline::error::ReadlineError;

fn with_logs(level: &str) {
    let env_str = format!("none,lithium={}", level);
    std::env::set_var("RUST_LOG", &env_str);
    pretty_env_logger::init_custom_env("RUST_LOG");
}

fn main() -> Result<(), ReadlineError> {
    #[cfg(debug_assertions)]
    with_logs("debug");

    let mut rl = rustyline::DefaultEditor::new()?;

    log::debug!("Starting REPL");

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                log::debug!("Line: {}", line);
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

use rustyline;

mod command;
mod lexer;
mod parser;
mod redirect;
mod typesystem;

// Built-in commands
mod cd;
mod ls;
mod pwd;

fn main() -> rustyline::Result<()> {
    loop {
        let mut rl = rustyline::DefaultEditor::new()?;
        let readline = rl.readline(&format!(
            "{} >> ",
            std::env::current_dir()
                .unwrap()
                .to_string_lossy()
                .replace(std::env::var("HOME").unwrap().as_str(), "~")
        ));
        let tokens = match readline {
            Ok(line) => lexer::lex(&line),
            Err(e) => match e {
                rustyline::error::ReadlineError::Eof => {
                    std::process::exit(0);
                }
                rustyline::error::ReadlineError::Interrupted => {
                    continue;
                }
                e => {
                    eprintln!("Error: {:?}", e);
                    continue;
                }
            },
        };

        let command = match tokens {
            Ok(tokens) => parser::Parser::new(tokens.iter().peekable()).parse(),
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        let output = command.run();

        match output {
            typesystem::Type::Null => (),
            _ => println!("{}", output),
        }
    }
}

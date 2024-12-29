use rustyline;

mod command;
mod parser;
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
        let command = match readline {
            Ok(line) => parser::parse(line),
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

        let output = match command.r#type {
            command::CommandType::BuiltIn(builtin) => command::builtins::run(builtin, command.args),
            command::CommandType::External => command::external::run(command),
            command::CommandType::None => continue,
        };

        match output {
            typesystem::Type::Null => (),
            _ => println!("{}", output),
        }
    }
}

use rustyline;

mod command;
mod parser;

fn main() -> rustyline::Result<()> {
    loop {
        let mut rl = rustyline::DefaultEditor::new()?;
        // TODO: Add the directory
        let readline = rl.readline("(dir) >> ");
        let command = match readline {
            Ok(line) => parser::parse(line),
            Err(_) => continue,
        };
        println!("{:?}", command);
    }

    Ok(())
}

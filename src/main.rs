use rustyline;

mod parser;

fn main() -> rustyline::Result<()> {
    loop {
        let mut rl = rustyline::DefaultEditor::new()?;
        // TODO: Add the directory
        let readline = rl.readline("(dir) >> ");
        match readline {
            Ok(line) => parser::parse(line),
            Err(_) => continue,
        }
    }

    Ok(())
}

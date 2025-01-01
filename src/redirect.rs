use std::io::Write;

use crate::command::{Command, CommandKind};
use crate::typesystem::Type;

impl Command {
    pub fn run_redirect(&mut self) -> Type {
        match &mut self.kind {
            CommandKind::Redirect {
                source,
                destination,
            } => {
                let source_output = source.run();
                let mut file = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(destination.run().to_undecorated_string())
                    .unwrap();
                file.write_all(source_output.to_colorless_string().as_bytes())
                    .unwrap();
            }
            _ => unreachable!(),
        }

        Type::Null
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let output = Command::new(CommandKind::Redirect {
            source: Box::new(Command::new(CommandKind::String("Hello, world!".into()))),
            destination: Box::new(Command::new(CommandKind::String("test.txt".into()))),
        })
        .run_redirect();

        assert_eq!(output, Type::Null);
        let contents = std::fs::read_to_string("test.txt").unwrap();
        assert_eq!(contents, "\"Hello, world!\"");
        std::fs::remove_file("test.txt").unwrap();
    }
}

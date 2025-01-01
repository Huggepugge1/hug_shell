use std::io::Write;

use crate::command::{Command, CommandKind};
use crate::typesystem::Type;

pub fn run(command: &mut Command) -> Type {
    match &mut command.kind {
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
            let colorless = source_output.to_colorless_string();
            let len = colorless.len();
            // Remove the first and last character, which is a quote
            file.write_all(colorless[1..len - 1].as_bytes()).unwrap();
        }
        _ => unreachable!(),
    }

    Type::Null
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let mut command = Command {
            kind: CommandKind::Redirect {
                source: Box::new(Command {
                    kind: CommandKind::String("Hello, world!".into()),
                    stdin: None,
                }),
                destination: Box::new(Command {
                    kind: CommandKind::String("test.txt".into()),
                    stdin: None,
                }),
            },
            stdin: None,
        };
        let output = run(&mut command);
        assert_eq!(output, Type::Null);
        let contents = std::fs::read_to_string("test.txt").unwrap();
        assert_eq!(contents, "Hello, world!");
        std::fs::remove_file("test.txt").unwrap();
    }
}

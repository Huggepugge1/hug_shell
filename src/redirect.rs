use std::io::Write;

use crate::command::Command;
use crate::typesystem::Type;

pub fn run(command: &Command) -> Type {
    match command {
        Command::Redirect {
            source,
            destination,
        } => {
            let source_output = source.run();
            match **destination {
                Command::String(ref s) => {
                    let mut file = std::fs::OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open(s)
                        .unwrap();
                    let colorless = source_output.to_colorless_string();
                    let len = colorless.len();
                    // Remove the first and last character, which is a quote
                    file.write_all(colorless[1..len - 1].as_bytes()).unwrap();
                }
                _ => eprintln!("Destination must be a string"),
            }
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
        let output = run(&Command::Redirect {
            source: Box::new(Command::String("Hello, world!".to_string())),
            destination: Box::new(Command::String("test.txt".to_string())),
        });
        assert_eq!(output, Type::Null);
        let contents = std::fs::read_to_string("test.txt").unwrap();
        assert_eq!(contents, "Hello, world!");
        std::fs::remove_file("test.txt").unwrap();
    }
}

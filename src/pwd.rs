use crate::command::builtins::BuiltinExitCode;

use crate::command::builtins::handle_builtin_error;
use crate::lexer::Token;
use crate::typesystem::Type;

pub fn run(args: Vec<Token>) -> Type {
    if !args.is_empty() {
        return Type::Error {
            message: "Too many arguments".into(),
            code: BuiltinExitCode::TooManyArguments as i32,
        };
    }
    match std::env::current_dir() {
        Ok(path) => Type::File(std::fs::File::open(&path).unwrap(), path.into()),
        Err(e) => handle_builtin_error(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        // Set the current directory to the root of the project
        std::env::set_current_dir(
            std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .parent()
                .unwrap(),
        )
        .unwrap();
        let output = run(Vec::new());
        match output {
            Type::File(_, path) => {
                assert_eq!(path, std::env::current_dir().unwrap());
            }
            _ => panic!("Expected Type::File"),
        }
    }
}

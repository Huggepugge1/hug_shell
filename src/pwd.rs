use crate::builtin::{handle_builtin_error, BuiltinExitCode};
use crate::command::Command;
use crate::typesystem::Type;

impl Command {
    pub fn run_pwd(&self) -> Type {
        let args = self.get_args();
        if !args.is_empty() {
            return Type::Error {
                message: "Too many arguments".into(),
                code: BuiltinExitCode::TooManyArguments as i32,
            };
        }
        match std::env::current_dir() {
            Ok(path) => Type::File {
                path: path.into(),
                full_path: true,
            },
            Err(e) => handle_builtin_error(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::builtin::Builtin;
    use crate::command::{Command, CommandKind};
    use crate::typesystem::Type;

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
        let output = Command {
            kind: CommandKind::Builtin {
                builtin: Builtin::Pwd,
                args: vec![],
            },
            stdin: None,
        }
        .run();
        match output {
            Type::File { path, .. } => {
                assert_eq!(path, std::env::current_dir().unwrap());
            }
            _ => panic!("Expected Type::File"),
        }
    }
}

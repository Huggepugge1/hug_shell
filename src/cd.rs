use homedir::my_home;

use std::path::PathBuf;

use crate::builtin::{handle_builtin_error, BuiltinExitCode};
use crate::command::Command;
use crate::typesystem::Type;

enum CdExitCode {
    HomeDirNotFound = 25,
}

impl Command {
    fn set_dir(&self, path: &PathBuf) -> Type {
        match std::env::set_current_dir(path) {
            Ok(_) => Type::Null,
            Err(e) => handle_builtin_error(e),
        }
    }

    fn set_home_dir(&self) -> Type {
        self.set_dir(match my_home() {
            Ok(Some(ref path)) => &path,
            Ok(None) => {
                return Type::Error {
                    message: "Could not find home directory".into(),
                    code: CdExitCode::HomeDirNotFound as i32,
                }
            }
            Err(e) => {
                return Type::Error {
                    message: e.to_string(),
                    code: BuiltinExitCode::UnknownError as i32,
                }
            }
        })
    }

    pub fn run_cd(&self) -> Type {
        let args = self.get_args();
        match args.len() {
            0 => self.set_home_dir(),
            1 => self.set_dir(&PathBuf::from(args[0].run_as_arg())),
            _ => Type::Error {
                message: "Too many arguments".into(),
                code: BuiltinExitCode::TooManyArguments as i32,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use homedir::my_home;

    use std::path::PathBuf;

    use crate::builtin::BuiltinExitCode;
    use crate::command::{Command, CommandKind};
    use crate::typesystem::Type;

    #[test]
    fn test_run() {
        let output = Command::new(CommandKind::Builtin {
            builtin: crate::builtin::Builtin::Cd,
            args: vec![],
        })
        .run();

        match output {
            Type::Null => {
                assert_eq!(
                    my_home().unwrap().unwrap(),
                    std::env::current_dir().unwrap()
                );
            }
            _ => panic!("Expected Null, got {}", output),
        }
    }

    #[test]
    fn test_run_with_arg() {
        const PROJECT_DIR: &str = "/home/huggepugge/hug_shell";
        const TEST_DIR: &str = "/home/huggepugge/hug_shell/test_dir";

        std::env::set_current_dir(PathBuf::from(PROJECT_DIR)).unwrap();
        let output = Command::new(CommandKind::Builtin {
            builtin: crate::builtin::Builtin::Cd,
            args: vec![Command::new(CommandKind::String(TEST_DIR.to_string()))],
        })
        .run();
        match output {
            Type::Null => {
                assert_eq!(std::env::current_dir().unwrap(), PathBuf::from(TEST_DIR));
            }
            _ => panic!("Expected Null, got {}", output),
        }
    }

    #[test]
    fn test_run_with_invalid_arg() {
        let output = Command::new(CommandKind::Builtin {
            builtin: crate::builtin::Builtin::Cd,
            args: vec![Command::new(CommandKind::String("/invalid".to_string()))],
        })
        .run();
        match output {
            Type::Error { code, .. } => {
                assert_eq!(code, BuiltinExitCode::FileNotFound as i32);
            }
            _ => panic!("Expected Error, got {}", output),
        }
    }

    #[test]
    fn test_run_with_too_many_args() {
        let output = Command::new(CommandKind::Builtin {
            builtin: crate::builtin::Builtin::Cd,
            args: vec![
                Command::new(CommandKind::String("/".to_string())),
                Command::new(CommandKind::String("/".to_string())),
            ],
        })
        .run();
        match output {
            Type::Error { code, .. } => {
                assert_eq!(code, BuiltinExitCode::TooManyArguments as i32);
            }
            _ => panic!("Expected Error, got {}", output),
        }
    }
}

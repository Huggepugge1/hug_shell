use crate::command::{Command, CommandKind};
use crate::typesystem::Type;

#[derive(Debug, PartialEq)]
pub enum Builtin {
    Cd,
    Exit,
    Ls,
    Pwd,
}

#[derive(Debug)]
pub enum BuiltinExitCode {
    TooManyArguments = 1,
    FileNotFound = 50,
    PermissionDenied = 100,
    UnknownError = 200,
}

pub trait BuiltinExt {
    fn is_builtin(&self) -> bool;
    fn get_builtin(&self) -> Builtin;
}

impl BuiltinExt for &str {
    fn is_builtin(&self) -> bool {
        match self {
            &"cd" => true,
            &"exit" => true,
            &"ls" => true,
            &"pwd" => true,
            _ => false,
        }
    }

    fn get_builtin(&self) -> Builtin {
        match self {
            &"cd" => Builtin::Cd,
            &"exit" => Builtin::Exit,
            &"ls" => Builtin::Ls,
            &"pwd" => Builtin::Pwd,
            name => panic!("`{name}` is not a builtin!"),
        }
    }
}

impl BuiltinExt for String {
    fn is_builtin(&self) -> bool {
        self.as_str().is_builtin()
    }

    fn get_builtin(&self) -> Builtin {
        self.as_str().get_builtin()
    }
}

pub fn handle_builtin_error(e: std::io::Error) -> Type {
    Type::Error {
        message: e.to_string(),
        code: match e.kind() {
            std::io::ErrorKind::NotFound => BuiltinExitCode::FileNotFound as i32,
            std::io::ErrorKind::PermissionDenied => BuiltinExitCode::PermissionDenied as i32,
            _ => BuiltinExitCode::UnknownError as i32,
        },
    }
}

impl Command {
    pub fn run_builtin(&self) -> Type {
        match &self.kind {
            CommandKind::Builtin { builtin, .. } => match builtin {
                Builtin::Cd => self.run_cd(),
                Builtin::Exit => std::process::exit(0),
                Builtin::Ls => self.run_ls(),
                Builtin::Pwd => self.run_pwd(),
            },
            _ => Type::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_builtin() {
        assert_eq!("cd".is_builtin(), true);
        assert_eq!("exit".is_builtin(), true);
        assert_eq!("ls".is_builtin(), true);
        assert_eq!("pwd".is_builtin(), true);
        assert_eq!("helloworld".is_builtin(), false);
    }

    #[test]
    fn test_get_builtin() {
        assert_eq!("cd".get_builtin(), Builtin::Cd);
        assert_eq!("exit".get_builtin(), Builtin::Exit);
        assert_eq!("ls".get_builtin(), Builtin::Ls);
        assert_eq!("pwd".get_builtin(), Builtin::Pwd);
    }

    #[test]
    #[should_panic(expected = "`helloworld` is not a builtin!")]
    fn test_get_builtin_panic() {
        "helloworld".get_builtin();
    }
}

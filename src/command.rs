use super::lexer::Token;

use crate::builtin;
use crate::external;
use crate::pipes;
use crate::redirect;

#[derive(Debug)]
pub struct Command {
    pub kind: CommandKind,
    pub stdin: Option<crate::typesystem::Type>,
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

pub const UNKNOWN_ERROR_CODE: i32 = 200;

impl Command {
    pub fn run(&mut self) -> crate::typesystem::Type {
        match &self.kind {
            CommandKind::Builtin { .. } => self.run_builtin(),
            CommandKind::External { .. } => external::run(self),

            CommandKind::String(s) => crate::typesystem::Type::String(s.clone()),
            CommandKind::Boolean(b) => crate::typesystem::Type::Boolean(*b),

            CommandKind::Redirect { .. } => redirect::run(self),
            CommandKind::Pipe { .. } => pipes::run(self),

            CommandKind::None => crate::typesystem::Type::Null,

            CommandKind::Error(e) => crate::typesystem::Type::Error {
                message: e.clone(),
                code: UNKNOWN_ERROR_CODE,
            },
        }
    }

    pub fn get_args(&self) -> Vec<Token> {
        match &self.kind {
            CommandKind::Builtin { args, .. } => args.clone(),
            CommandKind::External { args, .. } => args.clone(),
            _ => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_run_string() {
        let mut command = super::Command {
            kind: super::CommandKind::String("Hello, world!".into()),
            stdin: None,
        };
        let output = command.run();
        match output {
            crate::typesystem::Type::String(s) => {
                assert_eq!(s, "Hello, world!");
            }
            _ => panic!("Expected Type::String"),
        }
    }

    #[test]
    fn test_run_boolean() {
        let mut command = super::Command {
            kind: super::CommandKind::Boolean(true),
            stdin: None,
        };
        let output = command.run();
        match output {
            crate::typesystem::Type::Boolean(b) => {
                assert_eq!(b, true);
            }
            _ => panic!("Expected Type::Boolean"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CommandKind {
    Builtin {
        builtin: builtin::Builtin,
        args: Vec<Token>,
    },
    External {
        name: Token,
        args: Vec<Token>,
    },

    String(String),
    Boolean(bool),

    Redirect {
        source: Box<Command>,
        destination: Box<Command>,
    },
    Pipe {
        source: Box<Command>,
        destination: Box<Command>,
    },

    None,
    Error(String),
}

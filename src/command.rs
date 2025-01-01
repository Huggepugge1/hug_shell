use crate::builtin::Builtin;
use crate::lexer::Token;

#[derive(Debug, Clone)]
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
    pub fn new(kind: CommandKind) -> Self {
        Command { kind, stdin: None }
    }

    pub fn run(&mut self) -> crate::typesystem::Type {
        match &self.kind {
            CommandKind::Builtin { .. } => self.run_builtin(),
            CommandKind::External { .. } => self.run_external(),

            CommandKind::String(s) => crate::typesystem::Type::String(s.clone()),
            CommandKind::Boolean(b) => crate::typesystem::Type::Boolean(*b),
            CommandKind::Integer(i) => crate::typesystem::Type::Integer(*i),
            CommandKind::Float(f) => crate::typesystem::Type::Float(*f),

            CommandKind::Redirect { .. } => self.run_redirect(),
            CommandKind::Pipe { .. } => self.run_pipe(),

            CommandKind::None => crate::typesystem::Type::Null,

            CommandKind::Error(e) => crate::typesystem::Type::Error {
                message: e.clone(),
                code: UNKNOWN_ERROR_CODE,
            },
        }
    }

    pub fn run_as_arg(&self) -> String {
        match &self.kind {
            CommandKind::String(s) => s.clone(),
            CommandKind::Boolean(b) => b.to_string(),
            CommandKind::Integer(i) => i.to_string(),
            CommandKind::Float(f) => f.to_string(),
            _ => unreachable!(),
        }
    }

    pub fn get_args(&self) -> Vec<Command> {
        match &self.kind {
            CommandKind::Builtin { args, .. } => args.clone(),
            CommandKind::External { args, .. } => args.clone(),
            _ => Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CommandKind {
    Builtin {
        builtin: Builtin,
        args: Vec<Command>,
    },
    External {
        name: Token,
        args: Vec<Command>,
    },

    String(String),
    Boolean(bool),
    Integer(i64),
    Float(f64),

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

    #[test]
    fn test_run_integer() {
        let mut command = super::Command {
            kind: super::CommandKind::Integer(42),
            stdin: None,
        };
        let output = command.run();
        match output {
            crate::typesystem::Type::Integer(i) => {
                assert_eq!(i, 42);
            }
            _ => panic!("Expected Type::Integer"),
        }
    }

    #[test]
    fn test_run_float() {
        let mut command = super::Command {
            kind: super::CommandKind::Float(3.14),
            stdin: None,
        };
        let output = command.run();
        match output {
            crate::typesystem::Type::Float(f) => {
                assert_eq!(f, 3.14);
            }
            _ => panic!("Expected Type::Float"),
        }
    }
}

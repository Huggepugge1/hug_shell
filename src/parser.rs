use crate::command::Command;

pub fn parse(line: String) -> Command {
    if line.is_empty() {
        return Command {
            name: "".to_string(),
            args: Vec::new(),
            r#type: crate::command::CommandType::None,
        };
    }

    let mut name = String::new();
    let mut args = Vec::new();
    let mut r#type = crate::command::CommandType::External;

    let mut iter = line.split_whitespace();
    if let Some(token) = iter.next() {
        name = token.to_string();
    }

    for token in iter {
        args.push(token.to_string());
    }

    if crate::command::builtins::is_builtin(&name) {
        r#type = crate::command::CommandType::BuiltIn(crate::command::builtins::get(&name));
    }

    Command { name, args, r#type }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::Builtin;

    #[test]
    fn test_parse_cd() {
        let command = parse("cd".to_string());
        assert_eq!(command.name, "cd");
        assert_eq!(command.args, Vec::<&str>::new());
        assert_eq!(
            command.r#type,
            crate::command::CommandType::BuiltIn(Builtin::Cd)
        );

        let command = parse("cd /home".to_string());
        assert_eq!(command.name, "cd");
        assert_eq!(command.args, vec!["/home"]);
        assert_eq!(
            command.r#type,
            crate::command::CommandType::BuiltIn(Builtin::Cd)
        );

        let command = parse("cd /home /usr".to_string());
        assert_eq!(command.name, "cd");
        assert_eq!(command.args, vec!["/home", "/usr"]);
        assert_eq!(
            command.r#type,
            crate::command::CommandType::BuiltIn(Builtin::Cd)
        );
    }

    #[test]
    fn test_parse_exit() {
        let command = parse("exit".to_string());
        assert_eq!(command.name, "exit");
        assert_eq!(command.args, Vec::<&str>::new());
        assert_eq!(
            command.r#type,
            crate::command::CommandType::BuiltIn(Builtin::Exit)
        );

        let command = parse("exit 0".to_string());
        assert_eq!(command.name, "exit");
        assert_eq!(command.args, vec!["0"]);
        assert_eq!(
            command.r#type,
            crate::command::CommandType::BuiltIn(Builtin::Exit)
        );

        let command = parse("exit 0 1".to_string());
        assert_eq!(command.name, "exit");
        assert_eq!(command.args, vec!["0", "1"]);
        assert_eq!(
            command.r#type,
            crate::command::CommandType::BuiltIn(Builtin::Exit)
        );
    }

    #[test]
    fn test_parse_ls() {
        let command = parse("ls".to_string());
        assert_eq!(command.name, "ls");
        assert_eq!(command.args, Vec::<&str>::new());
        assert_eq!(
            command.r#type,
            crate::command::CommandType::BuiltIn(Builtin::Ls)
        );

        let command = parse("ls -l".to_string());
        assert_eq!(command.name, "ls");
        assert_eq!(command.args, vec!["-l"]);
        assert_eq!(
            command.r#type,
            crate::command::CommandType::BuiltIn(Builtin::Ls)
        );

        let command = parse("ls -l /home".to_string());
        assert_eq!(command.name, "ls");
        assert_eq!(command.args, vec!["-l", "/home"]);
        assert_eq!(
            command.r#type,
            crate::command::CommandType::BuiltIn(Builtin::Ls)
        );
    }

    #[test]
    fn test_parse_external() {
        let command = parse("helloworld".to_string());
        assert_eq!(command.name, "helloworld");
        assert_eq!(command.args, Vec::<&str>::new());
        assert_eq!(command.r#type, crate::command::CommandType::External);

        let command = parse("helloworld arg1".to_string());
        assert_eq!(command.name, "helloworld");
        assert_eq!(command.args, vec!["arg1"]);
        assert_eq!(command.r#type, crate::command::CommandType::External);

        let command = parse("helloworld arg1 arg2".to_string());
        assert_eq!(command.name, "helloworld");
        assert_eq!(command.args, vec!["arg1", "arg2"]);
        assert_eq!(command.r#type, crate::command::CommandType::External);
    }

    #[test]
    fn test_parse_empty() {
        let command = parse("".to_string());
        assert_eq!(command.name, "");
        assert_eq!(command.args, Vec::<&str>::new());
        assert_eq!(command.r#type, crate::command::CommandType::None);
    }
}

use crate::command::builtins::handle_builtin_error;
use crate::typesystem::Type;

pub fn run() -> Type {
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
        let output = run();
        match output {
            Type::File(_, path) => {
                assert_eq!(path, std::env::current_dir().unwrap());
            }
            _ => panic!("Expected Type::File"),
        }
    }
}

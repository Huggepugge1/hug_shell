use std::os::unix::process::ExitStatusExt;
use std::process::Output;

use crate::command::builtins::{handle_builtin_error, BuiltinExitCode};

fn list_dir(path: &str) -> Output {
    let mut output = Output {
        status: std::process::ExitStatus::from_raw(0),
        stdout: Vec::new(),
        stderr: Vec::new(),
    };

    match std::fs::read_dir(path) {
        Ok(entries) => {
            list_dir_files(entries, &mut output);
        }
        Err(e) => handle_builtin_error(&mut output, e),
    }

    output
}

fn list_dir_files(entries: std::fs::ReadDir, output: &mut Output) {
    for entry in entries {
        match entry {
            Ok(entry) => {
                output
                    .stdout
                    .append(&mut entry.file_name().as_encoded_bytes().to_vec());
                output.stdout.append(&mut "\n".as_bytes().to_vec());
            }
            Err(e) => {
                handle_builtin_error(output, e);
            }
        }
    }
}

pub fn run(args: Vec<String>) -> Output {
    if args.len() > 1 {
        Output {
            status: std::process::ExitStatus::from_raw(BuiltinExitCode::TooManyArguments as i32),
            stdout: Vec::new(),
            stderr: "Too many arguments".as_bytes().to_vec(),
        }
    } else if args.is_empty() {
        list_dir(".")
    } else {
        list_dir(&args[0])
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
        assert_eq!(output.status.code(), Some(0));
        assert_eq!(
            String::from_utf8(output.stdout).unwrap(),
            "src\n\
            LICENSE\n\
            Cargo.lock\n\
            Cargo.toml\n\
            target\n\
            test_dir\n\
            .git\n\
            README.md\n\
            .gitignore\n"
        );
    }

    #[test]
    fn test_run_with_args() {
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
        let output = run(vec!["test_dir".to_string()]);
        assert_eq!(output.status.success(), true);
        assert_eq!(
            String::from_utf8(output.stdout).unwrap(),
            "1.txt\n\
            2.txt\n"
        );
    }

    #[test]
    fn test_run_with_invalid_args() {
        let output = run(vec!["invalid".to_string()]);
        assert_eq!(
            output.status.signal(),
            Some(BuiltinExitCode::FileNotFound as i32)
        );
        assert_eq!(
            String::from_utf8(output.stderr).unwrap(),
            "No such file or directory (os error 2)"
        );
    }

    #[test]
    fn test_run_with_too_many_args() {
        let output = run(vec![
            "too".to_string(),
            "many".to_string(),
            "args".to_string(),
        ]);
        assert_eq!(
            output.status.signal(),
            Some(BuiltinExitCode::TooManyArguments as i32)
        );
        assert_eq!(
            String::from_utf8(output.stderr).unwrap(),
            "Too many arguments"
        );
    }
}

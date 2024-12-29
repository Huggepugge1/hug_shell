use homedir::my_home;

use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::ExitStatus;
use std::process::Output;

use crate::command::builtins::BuiltinExitCode;

enum CdExitCode {
    HomeDirNotFound = 25,
}

fn set_dir(path: PathBuf, output: &mut Output) {
    match std::env::set_current_dir(path) {
        Ok(_) => (),
        Err(e) => {
            output.stderr.append(&mut e.to_string().into());
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    output.status = ExitStatus::from_raw(BuiltinExitCode::FileNotFound as i32);
                }
                std::io::ErrorKind::PermissionDenied => {
                    output.status = ExitStatus::from_raw(BuiltinExitCode::PermissionDenied as i32);
                }
                _ => {
                    output.status = ExitStatus::from_raw(BuiltinExitCode::UnknownError as i32);
                }
            }
        }
    }
}

fn set_home_dir(output: &mut Output) {
    set_dir(
        match my_home() {
            Ok(Some(path)) => path,
            Ok(None) => {
                output
                    .stderr
                    .append(&mut "Could not find home directory".into());
                output.status = ExitStatus::from_raw(CdExitCode::HomeDirNotFound as i32);
                std::env::current_dir().unwrap()
            }
            Err(e) => {
                output.stderr.append(&mut e.to_string().into());
                output.status = ExitStatus::from_raw(BuiltinExitCode::UnknownError as i32);
                std::env::current_dir().unwrap()
            }
        },
        output,
    );
}

pub fn run(args: Vec<String>) -> Output {
    if args.is_empty() {
        let mut output = Output {
            status: ExitStatus::from_raw(0),
            stdout: Vec::new(),
            stderr: Vec::new(),
        };
        set_home_dir(&mut output);
        output
    } else if args.len() > 1 {
        Output {
            status: ExitStatus::from_raw(BuiltinExitCode::TooManyArguments as i32),
            stdout: Vec::new(),
            stderr: "Too many arguments".as_bytes().to_vec(),
        }
    } else {
        let mut output = Output {
            status: ExitStatus::from_raw(0),
            stdout: Vec::new(),
            stderr: Vec::new(),
        };
        set_dir(args[0].clone().into(), &mut output);
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use homedir::my_home;

    #[test]
    fn test_run() {
        let output = run(Vec::new());
        assert_eq!(output.status.success(), true);
        assert_eq!(
            my_home().unwrap().unwrap().to_string_lossy(),
            std::env::current_dir().unwrap().to_string_lossy()
        );
    }

    #[test]
    fn test_run_with_arg() {
        let output = run(vec!["/".to_string()]);
        assert_eq!(output.status.success(), true);
        assert_eq!(
            std::env::current_dir().unwrap().to_string_lossy(),
            "/".to_string()
        );
    }

    #[test]
    fn test_run_with_invalid_arg() {
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

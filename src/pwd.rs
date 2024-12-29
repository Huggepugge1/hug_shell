use std::os::unix::process::ExitStatusExt;
use std::process::ExitStatus;
use std::process::Output;

use crate::command::builtins::BuiltinExitCode;

pub fn run() -> Output {
    match std::env::current_dir() {
        Ok(path) => Output {
            status: ExitStatus::from_raw(0),
            stdout: path.to_string_lossy().as_bytes().to_vec(),
            stderr: Vec::new(),
        },
        Err(e) => Output {
            status: ExitStatus::from_raw(BuiltinExitCode::UnknownError as i32),
            stdout: Vec::new(),
            stderr: e.to_string().into(),
        },
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
        assert_eq!(output.status.success(), true,);
        assert_eq!(
            String::from_utf8(output.stdout).unwrap(),
            std::env::current_dir().unwrap().to_string_lossy()
        );
    }
}

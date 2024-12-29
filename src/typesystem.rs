use colored::Colorize;

use std::fs::File;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

pub enum Type {
    Output(std::process::Output),

    File(File, PathBuf),

    String(String),
    Array(Vec<Type>),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,

    Error { message: String, code: i32 },
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Output(o) => match o.status.success() {
                true => write!(f, "{}\n", String::from_utf8_lossy(&o.stdout)),
                false => write!(f, "{}\n", String::from_utf8_lossy(&o.stderr)),
            },

            Type::File(file, path) => color_file(file, path, f),

            Type::String(s) => write!(f, "{}\n", format!("\"{s}\"").green().to_string()),
            Type::Array(a) => write!(f, "{}\n", array_to_string(a).to_string()),
            Type::Integer(i) => write!(f, "{}\n", i.to_string().white().to_string()),
            Type::Float(fl) => write!(f, "{}\n", fl.to_string().white().to_string()),
            Type::Boolean(b) => write!(f, "{}\n", b.to_string().bright_magenta().to_string()),
            Type::Null => write!(f, "{}\n", "null".to_string().yellow().to_string()),

            Type::Error { message, code } => write!(
                f,
                "{}{}\nExited With status {}\n",
                "Error: ".red().to_string(),
                message,
                code
            ),
        }
    }
}

fn array_to_string(array: &Vec<Type>) -> String {
    let mut s = String::new();
    s.push_str("[\n");
    for (i, item) in array.iter().enumerate() {
        s.push_str("  ");
        s.push_str(&item.to_string());
        if i < array.len() - 1 {
            s.push_str(",\n");
        }
    }
    s.push_str("\n]");
    s
}

fn color_file(
    file: &File,
    path: &PathBuf,
    f: &mut std::fmt::Formatter<'_>,
) -> Result<(), std::fmt::Error> {
    let path_name = path.file_name().unwrap().to_str().unwrap();
    match file.metadata() {
        Ok(metadata) => {
            if metadata.is_dir() {
                if path_name.starts_with(".") {
                    // Faded blue
                    write!(f, "{}", path_name.bright_blue().to_string())
                } else {
                    write!(f, "{}", path_name.blue().to_string())
                }
            } else if metadata.is_file() {
                if path_name.starts_with(".") {
                    write!(f, "{}", path_name.bright_green().to_string())
                } else if metadata.permissions().mode() & 0o111 != 0 {
                    write!(f, "{}", path_name.yellow().to_string())
                } else {
                    write!(f, "{}", path_name.green().to_string())
                }
            } else {
                write!(f, "{:?}", path_name)
            }
        }
        Err(e) => {
            write!(f, "{:?}", e)
        }
    }
}

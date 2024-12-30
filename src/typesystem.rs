use colored::Colorize;

use std::fs::File;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Type {
    Output(std::process::Output),

    File { file: File, path: PathBuf },

    String(String),
    Array(Vec<Type>),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,

    Error { message: String, code: i32 },
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Type::Output(o) => match other {
                Type::Output(o2) => o == o2,
                _ => false,
            },
            Type::File { path, .. } => match other {
                Type::File { path: path2, .. } => path == path2,
                _ => false,
            },
            Type::String(s) => match other {
                Type::String(s2) => s == s2,
                _ => false,
            },
            Type::Array(a) => match other {
                Type::Array(a2) => a == a2,
                _ => false,
            },
            Type::Integer(i) => match other {
                Type::Integer(i2) => i == i2,
                _ => false,
            },
            Type::Float(fl) => match other {
                Type::Float(fl2) => fl == fl2,
                _ => false,
            },
            Type::Boolean(b) => match other {
                Type::Boolean(b2) => b == b2,
                _ => false,
            },
            Type::Null => match other {
                Type::Null => true,
                _ => false,
            },
            Type::Error { message, code } => match other {
                Type::Error {
                    message: m2,
                    code: c2,
                } => message == m2 && code == c2,
                _ => false,
            },
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Output(o) => match o.status.success() {
                true => write!(f, "{}", String::from_utf8_lossy(&o.stdout)),
                false => write!(f, "{}", String::from_utf8_lossy(&o.stderr)),
            },

            Type::File { file, path } => color_file(file, path, f),

            Type::String(s) => write!(f, "{}\n", format!("\"{s}\"").green().to_string()),
            Type::Array(a) => write!(f, "{}\n", array_to_string(a, true).to_string()),
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

impl Type {
    pub fn to_colorless_string(&self) -> String {
        match self {
            Type::Output(o) => match o.status.success() {
                true => String::from_utf8_lossy(&o.stdout).to_string(),
                false => String::from_utf8_lossy(&o.stderr).to_string(),
            },

            Type::File { path, .. } => path.file_name().unwrap().to_str().unwrap().to_string(),

            Type::String(s) => format!("\"{s}\""),
            Type::Array(a) => array_to_string(a, false),
            Type::Integer(i) => i.to_string(),
            Type::Float(fl) => fl.to_string(),
            Type::Boolean(b) => b.to_string(),
            Type::Null => "null".to_string(),

            Type::Error { message, code } => {
                format!("Error: {}\nExited With status {}", message, code)
            }
        }
    }
}

fn array_to_string(array: &Vec<Type>, colored: bool) -> String {
    let mut s = String::new();
    s.push_str("[\n");
    for (i, item) in array.iter().enumerate() {
        s.push_str("  ");
        if colored {
            s.push_str(&item.to_string());
        } else {
            s.push_str(&item.to_colorless_string());
        }
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

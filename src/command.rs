#[derive(Debug, PartialEq)]
pub enum CommandType {
    BuiltIn(Builtin),
    External,
}

#[derive(Debug, PartialEq)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
    pub r#type: CommandType,
}

#[derive(Debug, PartialEq)]
pub enum Builtin {
    Cd,
    Exit,
    Ls,
}

pub mod builtins {
    use super::Builtin;
    use homedir::my_home;

    pub fn is_builtin(name: &str) -> bool {
        match name {
            "cd" => true,
            "exit" => true,
            _ => false,
        }
    }

    pub fn get_builtin(name: &str) -> Builtin {
        match name {
            "cd" => Builtin::Cd,
            "exit" => Builtin::Exit,
            name => panic!("`{name}` is not a builtin!"),
        }
    }

    pub fn run_builtin(builtin: Builtin, args: Vec<String>) {
        match builtin {
            Builtin::Cd => {
                if args.len() == 0 {
                    std::env::set_current_dir(match my_home() {
                        Ok(Some(path)) => path,
                        Ok(None) => {
                            eprintln!("Could not find home directory");
                            std::env::current_dir().unwrap()
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                            std::env::current_dir().unwrap()
                        }
                    })
                    .unwrap();
                } else {
                    std::env::set_current_dir(&args[0]).unwrap();
                }
            }
            Builtin::Exit => std::process::exit(0),
            Builtin::Ls => {
                let dir = std::fs::read_dir(".").unwrap();
                for entry in dir {
                    let entry = entry.unwrap();
                    println!("{}", entry.file_name().to_string_lossy());
                }
            }
        }
    }
}

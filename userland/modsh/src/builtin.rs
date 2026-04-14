//! Shell built-in commands

pub enum Builtin {
    Cd,
    Echo,
    Exit,
    Ls,
    Cat,
}

impl Builtin {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "cd" => Some(Self::Cd),
            "echo" => Some(Self::Echo),
            "exit" => Some(Self::Exit),
            "ls" => Some(Self::Ls),
            "cat" => Some(Self::Cat),
            _ => None,
        }
    }

    pub fn execute(&self, _args: &[&str]) -> i32 {
        match self {
            Builtin::Cd => {
                // TODO: Change directory via filesystem IPC
                0
            }
            Builtin::Echo => {
                // TODO: Print arguments
                0
            }
            Builtin::Exit => {
                // TODO: Exit with code
                0
            }
            Builtin::Ls => {
                // TODO: List directory via filesystem IPC
                0
            }
            Builtin::Cat => {
                // TODO: Read file via filesystem IPC
                0
            }
        }
    }
}

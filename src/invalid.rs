use crate::extensions::Extensions;

pub enum Invalid {
    Args(String),
    NoArg(i32),
    NoFileFound,
    CannotOpenFile,
    FileSystemError
}

impl Invalid {
    pub fn alert_user_and_exit(&self) {
        match self {
            Invalid::Args(invalid_arg) => println!("Given argument is invalid: {}, first should be directory or file, second time in seconds", invalid_arg),
            Invalid::NoFileFound => println!("Given directory contains no files with extensions: {}", Extensions),
            Invalid::CannotOpenFile => println!("Couldn't open given file, it lacks permissions to write or read"),
            Invalid::FileSystemError => println!("File system error, cannot work with files and directories"),
            Invalid::NoArg(i) => println!("Program requires arguments! It lacks argument {}", i)
        }
    }
}
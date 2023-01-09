use std::env::current_dir;
use std::io::Error;
use std::path::PathBuf;

pub fn current_directory() -> PathBuf {
    let current_directory: Result<PathBuf, Error> = current_dir();

    let current_directory: PathBuf = match current_directory {
        Ok(cwd) => cwd,
        Err(error) => panic!("Could not get current directory {}", error),
    };

    current_directory
}

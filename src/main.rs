use std::path::Path;

mod util;

use crate::util::{env::current_directory, file_system::get_file_content};

fn main() {
    let directory_file_path = Path::new(&current_directory())
        .join(".config")
        .join("directory.txt");

    if !Path::new(&directory_file_path).exists() {
        panic!(
            "Missing directory path(s) in {}",
            directory_file_path.display()
        );
    }

    for file_path in get_file_content(&directory_file_path) {
        println!("{}", file_path);
    }
}

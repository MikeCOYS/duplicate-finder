use std::path::Path;

mod util;

use util::env::current_directory;
use util::file_system::get_file_paths;

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

    get_file_paths(&directory_file_path);
}

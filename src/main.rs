use std::fs::canonicalize;
use std::path::Path;

mod util;

use crate::util::file_system::{get_file_content, get_file_paths, group_files_by_size};
use crate::util::hash::hash_duplicate_files;
use util::env::current_directory;

fn main() {
    let search_directory_file_path = Path::new(&current_directory())
        .join(".config")
        .join("directory.txt");

    if !Path::new(&search_directory_file_path).exists() {
        panic!(
            "Missing directory path(s) in {}",
            search_directory_file_path.display()
        );
    }

    let directory_paths = match get_file_content(&search_directory_file_path) {
        Ok(f) => f,
        Err(e) => panic!("Could not get the list of file directories: {}", e),
    };

    let directory_paths = match directory_paths.iter().map(canonicalize).collect() {
        Ok(f) => f,
        Err(e) => panic!("Could not canonicalize file paths: {}", e),
    };

    // TODO: get_file_paths should additionally accept a ignore-list.txt file to ignore folders/files (use get_file_content("<root>/.config/ignore_list.txt"))
    let file_paths = match get_file_paths(directory_paths) {
        Ok(f) => f,
        Err(e) => panic!("Could not get file paths: {}", e),
    };

    let grouped_file_paths = group_files_by_size(file_paths);
    // TODO: Dump this in a file or a storage for the React UI to review
    let hashed_files = hash_duplicate_files(grouped_file_paths.unwrap());
}

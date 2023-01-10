use std::{
    collections::BTreeSet,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use walkdir::WalkDir;

pub fn get_file_content(file_path: &PathBuf) -> Vec<String> {
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!(
            "Failed to open file: {} - Error: {}",
            &file_path.display(),
            e
        ),
    };
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.expect("Could not load line"))
        .collect()
}

pub fn get_file_paths(directory_file_path: &PathBuf) -> BTreeSet<PathBuf> {
    get_file_content(directory_file_path)
        .into_iter()
        .flat_map(WalkDir::new)
        .flat_map(|f| f.ok())
        .filter_map(|x| {
            if x.metadata().unwrap().is_file() {
                return Some(x.path().to_owned());
            }

            None
        })
        .into_iter()
        .collect()
}

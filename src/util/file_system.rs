use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

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

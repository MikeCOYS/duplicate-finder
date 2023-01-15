use std::{
    collections::{HashMap, HashSet},
    fs::{metadata, read_dir, File},
    io::{BufRead, BufReader, Error},
    path::PathBuf,
};

pub fn get_file_content(file_path: &PathBuf) -> Result<Vec<String>, Error> {
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!(
            "Failed to open file: {} - Error: {}",
            &file_path.display(),
            e
        ),
    };

    BufReader::new(file).lines().collect()
}

fn list_files_recursively(dir: &PathBuf) -> Result<HashSet<PathBuf>, Error> {
    let read_dir = read_dir(dir)?;
    let mut files = HashSet::new();

    for entry in read_dir {
        let path = entry?.path();

        if path.is_dir() {
            files.extend(list_files_recursively(&path)?);
        } else {
            files.insert(path);
        }
    }

    Ok(files.drain().collect())
}

pub fn get_file_paths(directory_paths: Vec<PathBuf>) -> Result<HashSet<PathBuf>, Error> {
    let mut files = HashSet::new();

    for directory_path in directory_paths {
        for file_path in list_files_recursively(&directory_path).unwrap() {
            files.insert(file_path);
        }
    }

    Ok(files.drain().collect())
}

pub fn group_files_by_size(
    file_paths: HashSet<PathBuf>,
) -> Result<HashMap<u64, Vec<PathBuf>>, Error> {
    let mut files_by_size = HashMap::new();

    for file in file_paths {
        files_by_size
            .entry(metadata(file.clone())?.len())
            .or_insert_with(Vec::new)
            .push(file);
    }

    Ok(files_by_size
        .into_iter()
        .filter(|(_, v)| v.len() > 1)
        .collect::<HashMap<_, _>>())
}

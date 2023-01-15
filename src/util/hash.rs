use std::collections::HashMap;
use std::fs::metadata;
use std::io::{BufReader, Error, Read};
use std::{fs::File, path::PathBuf};

use xxhash_rust::xxh3::Xxh3;

const BUFFER_SIZE: usize = 1_000_000; // 1mb - decrease/incease buffer size for performance based on your system

pub fn hash_file(file_path: &PathBuf) -> Result<u64, Error> {
    let stat = metadata(file_path)?;
    let file_size = stat.len();
    let file = File::open(file_path)?;

    if file_size < (10 << 20) {
        // small file (10mb) - bit shift left 20 times: 10 * 2^20 = 10485760
        let mut buffer = vec![];

        BufReader::new(file).read_to_end(&mut buffer)?;
        let mut xxh3 = Xxh3::with_seed(0);
        xxh3.update(&buffer);

        Ok(xxh3.digest())
    } else {
        // large file
        let mut buf_reader = BufReader::with_capacity(BUFFER_SIZE, file);
        let mut buffer = [0u8; BUFFER_SIZE];
        let mut xxh3 = Xxh3::with_seed(0);

        loop {
            let n = buf_reader.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            xxh3.update(&buffer[..n]);
        }

        Ok(xxh3.digest())
    }
}

pub fn hash_duplicate_files(
    grouped_file_paths: HashMap<u64, Vec<PathBuf>>,
) -> Result<HashMap<u64, Vec<(PathBuf, u64)>>, Error> {
    let mut duplicate_files = HashMap::new();

    for (_size, file_paths) in grouped_file_paths {
        let mut hashed_files = Vec::new();
        for file_path in file_paths {
            let hash = hash_file(&file_path)?;
            hashed_files.push((file_path, hash));
        }
        duplicate_files.insert(_size, hashed_files);
    }

    Ok(duplicate_files)
}

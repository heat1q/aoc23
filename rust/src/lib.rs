use std::fs;
use std::path::Path;

pub fn read_lines(path: impl AsRef<Path>) -> Vec<String> {
    fs::read_to_string(path)
        .expect("read input file")
        .lines()
        .map(Into::into)
        .collect()
}

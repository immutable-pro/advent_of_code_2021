use std::fs;

pub fn read_file_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

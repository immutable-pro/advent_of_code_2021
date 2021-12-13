use std::fs;
use std::fmt::Display;

pub fn read_file_lines(filename: &str) ->Vec<String> {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
}

pub fn print_matrix<T:Display>(matrix: &[Vec<T>]) {
    let rows = matrix.len();
    let cols = matrix[0].len();

    println!();
    for row in matrix.iter().take(rows) {
        for val in row.iter().take(cols) {
            print!("{} ", val);
        }
        println!();
    }
}
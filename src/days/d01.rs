use crate::utils::read_file_lines;
use std::collections::HashSet;

pub fn part1() {
    let data = read_file_lines("input/01.txt");
    let mut entries = HashSet::new();

    for line in &data {
        let number: i32 = line.parse().unwrap();
        let complement = 2020 - number;
        entries.insert(number);

        if entries.contains(&complement) {
            println!("Day 01 > Part 1: {}", number * complement);
            break;
        }
    }
}

pub fn part2() {
    let data = read_file_lines("input/01.txt");
    let mut numbers = HashSet::new();

    for line in &data {
        let number: i32 = line.parse().unwrap();
        numbers.insert(number);

        for n in &numbers {
            let complement = 2020 - (n + number);
            if complement > 0 && numbers.contains(&complement) {
                println!("Day 01 > Part 2: {}", n * number * complement);
                break;
            }
        }
    }
}

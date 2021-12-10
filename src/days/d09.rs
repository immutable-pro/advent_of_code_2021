use crate::utils::read_file_lines;
use std::collections::HashSet;

/*
*/

const NEIGHBORS: [(i16, i16); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn get_neighbors((x, y): (i16, i16), map: &[Vec<i16>]) -> Vec<(i16, i16)> {
    let mut neighbors = Vec::<(i16, i16)>::new();
    for (nx, ny) in &NEIGHBORS {
        let new_x = nx + x;
        let new_y = ny + y;
        if new_x >= 0
            && new_y >= 0
            && (new_x as usize) < map.len()
            && (new_y as usize) < map[0].len()
        {
            neighbors.push((new_x, new_y));
        }
    }
    neighbors
}

pub fn part1() {
    let map = read_file_lines("input/09.txt")
        .map(|row| {
            row.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect::<Vec<i16>>()
        })
        .collect::<Vec<Vec<i16>>>();

    let mut stack = Vec::<(i16, i16)>::from([(0_i16, 0_i16)]);
    let mut visited = HashSet::<(i16, i16)>::new();
    let mut risk = 0_i16;

    while let Some(pos) = stack.pop() {
        if visited.contains(&pos) {
            continue;
        }
        let neighbors = get_neighbors(pos, &map);
        let is_lowest = neighbors
            .iter()
            .all(|(x, y)| map[*x as usize][*y as usize] > map[pos.0 as usize][pos.1 as usize]);

        if is_lowest {
            risk += map[pos.0 as usize][pos.1 as usize] + 1;
        }
        visited.insert(pos);
        neighbors.iter().for_each(|n| stack.push(*n));
    }

    println!("Day 09 > Part 1: {}", risk);
}

/*
*/

fn get_neighbors_but_9((x, y): (i16, i16), map: &[Vec<i16>]) -> Vec<(i16, i16)> {
    let mut neighbors = Vec::<(i16, i16)>::new();
    for (nx, ny) in &NEIGHBORS {
        let new_x = nx + x;
        let new_y = ny + y;
        if new_x >= 0
            && new_y >= 0
            && (new_x as usize) < map.len()
            && (new_y as usize) < map[0].len()
            && map[new_x as usize][new_y as usize] != 9
        {
            neighbors.push((new_x, new_y));
        }
    }
    neighbors
}

pub fn part2() {
    let map = read_file_lines("input/09.txt")
        .map(|row| {
            row.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect::<Vec<i16>>()
        })
        .collect::<Vec<Vec<i16>>>();

    let mut visited = HashSet::<(i16, i16)>::new();
    let mut sizes = Vec::<i16>::new();

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let idx = (i as i16, j as i16);
            if map[i][j] == 9 || visited.contains(&idx) {
                continue;
            }
            let mut stack = Vec::<(i16, i16)>::from([idx]);
            let mut size = 0_i16;
            while let Some(pos) = stack.pop() {
                if visited.contains(&pos) {
                    continue;
                }
                size += 1;
                let neighbors = get_neighbors_but_9(pos, &map);
                visited.insert(pos);
                neighbors.iter().for_each(|n| stack.push(*n));
            }
            sizes.push(size);
        }
    }

    sizes.sort_unstable();
    let result: i32 = sizes.iter().rev().take(3).fold(1, |acc, v| acc * (*v as i32));

    println!("Day 09 > Part 2: {}", result);
}

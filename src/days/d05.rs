use crate::utils::read_file_lines;
use std::collections::HashMap;

/*
You come across a field of hydrothermal vents on the ocean floor! These vents constantly produce large, opaque clouds, so it would be best to avoid them if possible.

They tend to form in lines; the submarine helpfully produces a list of nearby lines of vents (your puzzle input) for you to review. For example:

0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2

Each line of vents is given as a line segment in the format x1,y1 -> x2,y2 where x1,y1 are the coordinates of one end the line segment and x2,y2 are the coordinates of the other end. These line segments include the points at both ends. In other words:

    An entry like 1,1 -> 1,3 covers points 1,1, 1,2, and 1,3.
    An entry like 9,7 -> 7,7 covers points 9,7, 8,7, and 7,7.

For now, only consider horizontal and vertical lines: lines where either x1 = x2 or y1 = y2.

So, the horizontal and vertical lines from the above list would produce the following diagram:

.......1..
..1....1..
..1....1..
.......1..
.112111211
..........
..........
..........
..........
222111....

In this diagram, the top left corner is 0,0 and the bottom right corner is 9,9. Each position is shown as the number of lines which cover that point or . if no line covers that point. The top-left pair of 1s, for example, comes from 2,2 -> 2,1; the very bottom row is formed by the overlapping lines 0,9 -> 5,9 and 0,9 -> 2,9.

To avoid the most dangerous areas, you need to determine the number of points where at least two lines overlap. In the above example, this is anywhere in the diagram with a 2 or larger - a total of 5 points.

Consider only horizontal and vertical lines. At how many points do at least two lines overlap?
*/
fn add_point(points: &mut HashMap<(i16, i16), i16>, (x, y): (i16, i16)) {
    let point = points.get_mut(&(x, y));
    match point {
        Some(count) => *count += 1,
        None => {
            points.insert((x, y), 1);
        }
    }
}

pub fn part1() {
    let mut points: HashMap<(i16, i16), i16> = HashMap::new();
    read_file_lines("input/05.txt")
        .map(|line| {
            let mut parts = line.split(" -> ");
            let coords_1: Vec<i16> = parts
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect();
            let coords_2: Vec<i16> = parts
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect();
            (coords_1[0], coords_1[1], coords_2[0], coords_2[1])
        })
        .filter(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2)
        .for_each(|(x1, y1, x2, y2)| {
            if x1 == x2 {
                let range = if y1 < y2 { y1..(y2 + 1) } else { y2..(y1 + 1) };
                for i in range {
                    add_point(&mut points, (x1, i));
                }
            } else if y1 == y2 {
                let range = if x1 < x2 { x1..(x2 + 1) } else { x2..(x1 + 1) };
                for i in range {
                    add_point(&mut points, (i, y1));
                }
            }
        });

    let result = points
        .values()
        .fold(0, |acc, x| if *x >= 2 { acc + 1 } else { acc });
    println!("Day 05 > Part 1: {}", result);
}

/*
Unfortunately, considering only horizontal and vertical lines doesn't give you the full picture; you need to also consider diagonal lines.

Because of the limits of the hydrothermal vent mapping system, the lines in your list will only ever be horizontal, vertical, or a diagonal line at exactly 45 degrees. In other words:

    An entry like 1,1 -> 3,3 covers points 1,1, 2,2, and 3,3.
    An entry like 9,7 -> 7,9 covers points 9,7, 8,8, and 7,9.

Considering all lines from the above example would now produce the following diagram:

1.1....11.
.111...2..
..2.1.111.
...1.2.2..
.112313211
...1.2....
..1...1...
.1.....1..
1.......1.
222111....

You still need to determine the number of points where at least two lines overlap. In the above example, this is still anywhere in the diagram with a 2 or larger - now a total of 12 points.

Consider all of the lines. At how many points do at least two lines overlap?
*/

pub fn part2() {
    let mut points: HashMap<(i16, i16), i16> = HashMap::new();
    read_file_lines("input/05.txt")
        .map(|line| {
            let mut parts = line.split(" -> ");
            let coords_1: Vec<i16> = parts
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect();
            let coords_2: Vec<i16> = parts
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect();
            (coords_1[0], coords_1[1], coords_2[0], coords_2[1])
        })
        .for_each(|(x1, y1, x2, y2)| {
            if x1 == x2 {
                let range = if y1 < y2 { y1..(y2 + 1) } else { y2..(y1 + 1) };
                for i in range {
                    add_point(&mut points, (x1, i));
                }
            } else if y1 == y2 {
                let range = if x1 < x2 { x1..(x2 + 1) } else { x2..(x1 + 1) };
                for i in range {
                    add_point(&mut points, (i, y1));
                }
            } else {
                add_point(&mut points, (x1, y1));
                let step_x = if x1 < x2 { 1 } else { -1 };
                let step_y = if y1 < y2 { 1 } else { -1 };
                let mut x = x1;
                let mut y = y1;
                while x != x2 && y != y2 {
                    x += step_x;
                    y += step_y;
                    add_point(&mut points, (x, y));
                }
            }
        });

    let result = points
        .values()
        .fold(0, |acc, x| if *x >= 2 { acc + 1 } else { acc });
    println!("Day 05 > Part 2: {}", result);
}

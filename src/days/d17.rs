use crate::utils::read_file_lines;
use regex::Regex;
use std::collections::HashMap;

/*
You finally decode the Elves' message. HI, the message says. You continue searching for the sleigh keys.

Ahead of you is what appears to be a large ocean trench. Could the keys have fallen into it? You'd better send a probe to investigate.

The probe launcher on your submarine can fire the probe with any integer velocity in the x (forward) and y (upward, or downward if negative) directions. For example, an initial x,y velocity like 0,10 would fire the probe straight up, while an initial velocity like 10,-1 would fire the probe forward at a slight downward angle.

The probe's x,y position starts at 0,0. Then, it will follow some trajectory by moving in steps. On each step, these changes occur in the following order:

    The probe's x position increases by its x velocity.
    The probe's y position increases by its y velocity.
    Due to drag, the probe's x velocity changes by 1 toward the value 0; that is, it decreases by 1 if it is greater than 0, increases by 1 if it is less than 0, or does not change if it is already 0.
    Due to gravity, the probe's y velocity decreases by 1.

For the probe to successfully make it into the trench, the probe must be on some trajectory that causes it to be within a target area after any step. The submarine computer has already calculated this target area (your puzzle input). For example:

target area: x=20..30, y=-10..-5

This target area means that you need to find initial x,y velocity values such that after any step, the probe's x position is at least 20 and at most 30, and the probe's y position is at least -10 and at most -5.

Given this target area, one initial velocity that causes the probe to be within the target area after any step is 7,2:

.............#....#............
.......#..............#........
...............................
S........................#.....
...............................
...............................
...........................#...
...............................
....................TTTTTTTTTTT
....................TTTTTTTTTTT
....................TTTTTTTT#TT
....................TTTTTTTTTTT
....................TTTTTTTTTTT
....................TTTTTTTTTTT

In this diagram, S is the probe's initial position, 0,0. The x coordinate increases to the right, and the y coordinate increases upward. In the bottom right, positions that are within the target area are shown as T. After each step (until the target area is reached), the position of the probe is marked with #. (The bottom-right # is both a position the probe reaches and a position in the target area.)

Another initial velocity that causes the probe to be within the target area after any step is 6,3:

...............#..#............
...........#........#..........
...............................
......#..............#.........
...............................
...............................
S....................#.........
...............................
...............................
...............................
.....................#.........
....................TTTTTTTTTTT
....................TTTTTTTTTTT
....................TTTTTTTTTTT
....................TTTTTTTTTTT
....................T#TTTTTTTTT
....................TTTTTTTTTTT

Another one is 9,0:

S........#.....................
.................#.............
...............................
........................#......
...............................
....................TTTTTTTTTTT
....................TTTTTTTTTT#
....................TTTTTTTTTTT
....................TTTTTTTTTTT
....................TTTTTTTTTTT
....................TTTTTTTTTTT

One initial velocity that doesn't cause the probe to be within the target area after any step is 17,-4:

S..............................................................
...............................................................
...............................................................
...............................................................
.................#.............................................
....................TTTTTTTTTTT................................
....................TTTTTTTTTTT................................
....................TTTTTTTTTTT................................
....................TTTTTTTTTTT................................
....................TTTTTTTTTTT..#.............................
....................TTTTTTTTTTT................................
...............................................................
...............................................................
...............................................................
...............................................................
................................................#..............
...............................................................
...............................................................
...............................................................
...............................................................
...............................................................
...............................................................
..............................................................#

The probe appears to pass through the target area, but is never within it after any step. Instead, it continues down and to the right - only the first few steps are shown.

If you're going to fire a highly scientific probe out of a super cool probe launcher, you might as well do it with style. How high can you make the probe go while still reaching the target area?

In the above example, using an initial velocity of 6,9 is the best you can do, causing the probe to reach a maximum y position of 45. (Any higher initial y velocity causes the probe to overshoot the target area entirely.)

Find the initial velocity that causes the probe to reach the highest y position and still eventually be within the target area after any step. What is the highest y position it reaches on this trajectory?
*/

pub fn part1() {
    let target_area_input = &read_file_lines("input/17.txt")[0];
    // target area: x=20..30, y=-10..-5
    let re = Regex::new(r"^target area: x=(\d+)..(\d+), y=(-?\d+)..(-?\d+)$").unwrap();
    let mut x1: i16 = 0;
    let mut x2: i16 = 0;
    let mut y1: i16 = 0;
    let mut y2: i16 = 0;
    for cap in re.captures_iter(target_area_input) {
        x1 = cap[1].parse().unwrap();
        x2 = cap[2].parse().unwrap();
        y1 = cap[3].parse().unwrap();
        y2 = cap[4].parse().unwrap();
    }

    // let mut valid_v_x = HashMap::<i16, usize>::new();
    // let min_x_speed = 1 + ((-1 + ((1 + 8 * x1) as f32).sqrt() as i16) / 2);
    // let valid_end_range = x1..=x2;
    // let mut max_x_steps_count = i16::MIN;
    // let mut max_steps_x_vel = -1;
    // for v in (min_x_speed..x1).rev() {
    //     let mut reach = v;
    //     for i in (1..v).rev() {
    //         reach += i;
    //         if valid_end_range.contains(&reach) {
    //             valid_v_x.insert(v, (v - i + 1) as usize);
    //             if (v - i + 1) > max_x_steps_count {
    //                 max_x_steps_count = v - i + 1;
    //                 max_steps_x_vel = v;
    //             }
    //             break;
    //         }
    //         if reach > x2 {
    //             break;
    //         }
    //     }
    // }

    let n = -y1 - 1;
    let max_height = n * (n + 1) / 2;
    println!("Day 17 > Part 1: {}", max_height);
}

pub fn part2() {}

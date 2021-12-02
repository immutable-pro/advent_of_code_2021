use crate::utils::read_file_lines;

/*
Now, you need to figure out how to pilot this thing.

It seems like the submarine can take a series of commands like forward 1, down 2, or up 3:

    forward X increases the horizontal position by X units.
    down X increases the depth by X units.
    up X decreases the depth by X units.

Note that since you're on a submarine, down and up affect your depth, and so they have the opposite result of what you might expect.

The submarine seems to already have a planned course (your puzzle input). You should probably figure out where it's going. For example:

forward 5
down 5
forward 8
up 3
down 8
forward 2

Your horizontal position and depth both start at 0. The steps above would then modify them as follows:

    forward 5 adds 5 to your horizontal position, a total of 5.
    down 5 adds 5 to your depth, resulting in a value of 5.
    forward 8 adds 8 to your horizontal position, a total of 13.
    up 3 decreases your depth by 3, resulting in a value of 2.
    down 8 adds 8 to your depth, resulting in a value of 10.
    forward 2 adds 2 to your horizontal position, a total of 15.

After following these instructions, you would have a horizontal position of 15 and a depth of 10. (Multiplying these together produces 150.)

Calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?
*/

pub fn part1() {
    let mut hor_pos: i32 = 0;
    let mut depth_pos: i32 = 0;

    read_file_lines("input/02.txt")
        .into_iter()
        .map(|instruction| {
            instruction
                .split_whitespace()
                .map(|x| x.to_string())
                .collect()
        })
        .for_each(|chunks: Vec<String>| {
            let instruction = &chunks[0];
            let amount: i32 = chunks[1].parse().unwrap();
            match instruction.as_str() {
                "forward" => { hor_pos += amount },
                "down" => { depth_pos += amount },
                "up" => { depth_pos -= amount },
                _ => { println!("Weird instruction found") }
            }
        });

        println!("Day 02 > Part 1: {}", hor_pos * depth_pos);
    }

/*
*/

pub fn part2() {}

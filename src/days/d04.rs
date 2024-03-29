use crate::utils::read_file_lines;
use std::collections::HashMap;
use std::collections::HashSet;

/*
You're already almost 1.5km (almost a mile) below the surface of the ocean, already so deep that you can't see any sunlight. What you can see, however, is a giant squid that has attached itself to the outside of your submarine.

Maybe it wants to play bingo?

Bingo is played on a set of boards each consisting of a 5x5 grid of numbers. Numbers are chosen at random, and the chosen number is marked on all boards on which it appears. (Numbers may not appear on all boards.) If all numbers in any row or any column of a board are marked, that board wins. (Diagonals don't count.)

The submarine has a bingo subsystem to help passengers (currently, you and the giant squid) pass the time. It automatically generates a random order in which to draw numbers and a random set of boards (your puzzle input). For example:

7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7

After the first five numbers are drawn (7, 4, 9, 5, and 11), there are no winners, but the boards are marked as follows (shown here adjacent to each other to save space):

22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
 8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
 6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
 1 12 20 15 19        14 21 16 12  6         2  0 12  3  7

After the next six numbers are drawn (17, 23, 2, 0, 14, and 21), there are still no winners:

22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
 8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
 6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
 1 12 20 15 19        14 21 16 12  6         2  0 12  3  7

Finally, 24 is drawn:

22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
 8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
 6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
 1 12 20 15 19        14 21 16 12  6         2  0 12  3  7

At this point, the third board wins because it has at least one complete row or column of marked numbers (in this case, the entire top row is marked: 14 21 17 24 4).

The score of the winning board can now be calculated. Start by finding the sum of all unmarked numbers on that board; in this case, the sum is 188. Then, multiply that sum by the number that was just called when the board won, 24, to get the final score, 188 * 24 = 4512.

To guarantee victory against the giant squid, figure out which board will win first. What will your final score be if you choose that board?
*/

const SIZE: usize = 5;

struct Board {
    hits: Vec<i32>,
    unmarked: HashSet<i32>,
    numbers: HashMap<i32, (usize, usize)>,
}

fn line_to_numbers(line: &str) -> Vec<i32> {
    if line.is_empty() {
        Vec::new()
    } else {
        line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    }
}

fn check_for_lines(board: &Board, number: &i32) -> bool {
    let (i, j) = board.numbers[number];
    let mut same_row_found = 0;
    let mut same_column_found = 0;
    for number in &board.hits {
        let (hit_i, hit_j) = board.numbers[number];
        if hit_i == i {
            same_row_found += 1;
        }
        if hit_j == j {
            same_column_found += 1;
        }
    }
    same_row_found == SIZE || same_column_found == SIZE
}

fn calculate_result(board: &Board) -> i32 {
    board.unmarked.iter().sum()
}

fn parse_bingo() -> (Vec<i32>, Vec<Board>) {
    let mut bingo = read_file_lines("input/04.txt").into_iter();
    let random_numbers: Vec<i32> = bingo
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let mut boards: Vec<Board> = Vec::new();

    let mut i: usize = 0;
    for line in bingo {
        let numbers = line_to_numbers(&line);
        if numbers.is_empty() {
            boards.push(Board {
                hits: Vec::new(),
                unmarked: HashSet::new(),
                numbers: HashMap::new(),
            });
            i = 0;
        } else {
            numbers.into_iter().enumerate().for_each(|(j, number)| {
                boards.last_mut().unwrap().numbers.insert(number, (i, j));
                boards.last_mut().unwrap().unmarked.insert(number);
            });
            i += 1;
        }
    }
    (random_numbers, boards)
}

pub fn part1() {
    let (random_numbers, mut boards) = parse_bingo();

    let mut result: i32 = -1;
    let mut done = false;
    for number in &random_numbers {
        if done {
            break;
        }

        for board in &mut boards {
            if board.numbers.contains_key(number) {
                board.hits.push(*number);
                board.unmarked.remove(number);

                if board.hits.len() >= 5 && check_for_lines(board, number) {
                    result = calculate_result(board) * number;
                    done = true;
                    break;
                }
            }
        }
    }
    println!("Day 04 > Part 1: {}", result);
}

/*
On the other hand, it might be wise to try a different strategy: let the giant squid win.

You aren't sure how many bingo boards a giant squid could play at once, so rather than waste time counting its arms, the safe thing to do is to figure out which board will win last and choose that one. That way, no matter which boards it picks, it will win for sure.

In the above example, the second board is the last to win, which happens after 13 is eventually called and its middle column is completely marked. If you were to keep playing until this point, the second board would have a sum of unmarked numbers equal to 148 for a final score of 148 * 13 = 1924.

Figure out which board will win last. Once it wins, what would its final score be?
*/
pub fn part2() {
    let (random_numbers, mut boards) = parse_bingo();

    let mut winning_boards: HashSet<usize> = HashSet::new();
    let mut last_won: usize = usize::MAX;
    let mut last_won_number: i32 = -1;
    for number in &random_numbers {
        for (idx, board) in boards.iter_mut().enumerate() {
            if !winning_boards.contains(&idx) && board.numbers.contains_key(number) {
                board.hits.push(*number);
                board.unmarked.remove(number);

                if board.hits.len() >= 5 && check_for_lines(board, number) {
                    last_won = idx;
                    last_won_number = *number;
                    winning_boards.insert(idx);
                }
            }
        }
    }

    println!(
        "Day 04 > Part 2: {}",
        calculate_result(&boards[last_won]) * last_won_number
    );
}

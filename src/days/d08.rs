use crate::utils::read_file_lines;
use std::collections::HashMap;
use std::collections::HashSet;
/*
You barely reach the safety of the cave when the whale smashes into the cave mouth, collapsing it. Sensors indicate another exit to this cave at a much greater depth, so you have no choice but to press on.

As your submarine slowly makes its way through the cave system, you notice that the four-digit seven-segment displays in your submarine are malfunctioning; they must have been damaged during the escape. You'll be in a lot of trouble without them, so you'd better figure out what's wrong.

Each digit of a seven-segment display is rendered by turning on or off any of seven segments named a through g:

  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg

So, to render a 1, only segments c and f would be turned on; the rest would be off. To render a 7, only segments a, c, and f would be turned on.

The problem is that the signals which control the segments have been mixed up on each display. The submarine is still trying to display numbers by producing output on signal wires a through g, but those wires are connected to segments randomly. Worse, the wire/segment connections are mixed up separately for each four-digit display! (All of the digits within a display use the same connections, though.)

So, you might know that only signal wires b and g are turned on, but that doesn't mean segments b and g are turned on: the only digit that uses two segments is 1, so it must mean segments c and f are meant to be on. With just that information, you still can't tell which wire (b/g) goes to which segment (c/f). For that, you'll need to collect more information.

For each display, you watch the changing signals for a while, make a note of all ten unique signal patterns you see, and then write down a single four digit output value (your puzzle input). Using the signal patterns, you should be able to work out which pattern corresponds to which digit.

For example, here is what you might see in a single entry in your notes:

acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
cdfeb fcadb cdfeb cdbaf

(The entry is wrapped here to two lines so it fits; in your notes, it will all be on a single line.)

Each entry consists of ten unique signal patterns, a | delimiter, and finally the four digit output value. Within an entry, the same wire/segment connections are used (but you don't know what the connections actually are). The unique signal patterns correspond to the ten different ways the submarine tries to render a digit using the current wire/segment connections. Because 7 is the only digit that uses three segments, dab in the above example means that to render a 7, signal lines d, a, and b are on. Because 4 is the only digit that uses four segments, eafb means that to render a 4, signal lines e, a, f, and b are on.

Using this information, you should be able to work out which combination of signal wires corresponds to each of the ten digits. Then, you can decode the four digit output value. Unfortunately, in the above example, all of the digits in the output value (cdfeb fcadb cdfeb cdbaf) use five segments and are more difficult to deduce.

For now, focus on the easy digits. Consider this larger example:

be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
fgae cfgab fg bagce

Because the digits 1, 4, 7, and 8 each use a unique number of segments, you should be able to tell which combinations of signals correspond to those digits. Counting only digits in the output values (the part after | on each line), in the above example, there are 26 instances of digits that use a unique number of segments (highlighted above).

In the output values, how many times do digits 1, 4, 7, or 8 appear?
*/
pub fn part1() {
    let result: i32 = read_file_lines("input/08.txt")
        .map(|line| {
            let mut parts = line.split(" | ");
            parts.next();
            parts
                .next()
                .unwrap()
                .split_whitespace()
                .fold(0, |acc, digit| {
                    acc + if digit.len() == 2
                        || digit.len() == 4
                        || digit.len() == 3
                        || digit.len() == 7
                    {
                        1
                    } else {
                        0
                    }
                })
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum();
    println!("Day 08 > Part 1: {}", result);
}

/*
Through a little deduction, you should now be able to determine the remaining digits. Consider again the first example above:

acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
cdfeb fcadb cdfeb cdbaf

After some careful analysis, the mapping between signal wires and segments only make sense in the following configuration:

 dddd
e    a
e    a
 ffff
g    b
g    b
 cccc

So, the unique signal patterns would correspond to the following digits:

    acedgfb: 8
    cdfbe: 5
    gcdfa: 2
    fbcad: 3
    dab: 7
    cefabd: 9
    cdfgeb: 6
    eafb: 4
    cagedb: 0
    ab: 1

Then, the four digits of the output value can be decoded:

    cdfeb: 5
    fcadb: 3
    cdfeb: 5
    cdbaf: 3

Therefore, the output value for this entry is 5353.

Following this same process for each entry in the second, larger example above, the output value of each entry can be determined:

    fdgacbe cefdb cefbgd gcbe: 8394
    fcgedb cgb dgebacf gc: 9781
    cg cg fdcagb cbg: 1197
    efabcd cedba gadfec cb: 9361
    gecf egdcabf bgf bfgea: 4873
    gebdcfa ecba ca fadegcb: 8418
    cefg dcbef fcge gbcadfe: 4548
    ed bcgafe cdgba cbgef: 1625
    gbdfcae bgc cg cgb: 8717
    fgae cfgab fg bagce: 4315

Adding all of the output values in this larger example produces 61229.

For each entry, determine all of the wire/segment connections and decode the four-digit output values. What do you get if you add up all of the output values?
*/

fn decode(digits: &mut Vec<String>) -> HashMap<char, usize> {
    let all_chars = HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']);

    digits.sort_unstable_by_key(|a| a.len());
    let one = digits[0].chars().collect::<HashSet<char>>();
    let seven = digits[1].chars().collect::<HashSet<char>>();
    let four = &digits[2].chars().collect::<HashSet<char>>();
    let two_three_five = [
        digits[3].chars().collect::<HashSet<char>>(),
        digits[4].chars().collect::<HashSet<char>>(),
        digits[5].chars().collect::<HashSet<char>>(),
    ];
    let zero_six_nine = [
        digits[6].chars().collect::<HashSet<char>>(),
        digits[7].chars().collect::<HashSet<char>>(),
        digits[8].chars().collect::<HashSet<char>>(),
    ];

    let mut found = HashSet::<char>::new();
    let mut segments = vec![HashSet::<char>::new(); 7];
    one.iter().for_each(|c| {
        segments[2].insert(*c);
        segments[5].insert(*c);
        found.insert(*c);
    });

    segments[0].insert(*seven.symmetric_difference(&one).next().unwrap());
    found.insert(*segments[0].iter().next().unwrap());

    four.symmetric_difference(&found)
        .cloned()
        .collect::<Vec<char>>()
        .iter()
        .for_each(|c| {
            if c == segments[0].iter().next().unwrap() {
                return;
            }
            segments[1].insert(*c);
            segments[3].insert(*c);
            found.insert(*c);
        });

    let mut diff: char = '?';
    let _nine = zero_six_nine
        .iter()
        .find(|digit| {
            let diffs = digit
                .symmetric_difference(&found)
                .cloned()
                .collect::<Vec<char>>();
            if diffs.len() == 1 {
                diff = diffs[0];
                true
            } else {
                false
            }
        })
        .unwrap();

    segments[6].insert(diff);
    found.insert(diff);
    let segment4 = *found
        .symmetric_difference(&all_chars)
        .cloned()
        .collect::<Vec<char>>()
        .get(0)
        .unwrap();
    segments[4].insert(segment4);

    let two = two_three_five
        .iter()
        .find(|digit| digit.contains(&segment4))
        .unwrap();

    let segment2 = *two
        .intersection(&one)
        .cloned()
        .collect::<Vec<char>>()
        .get(0)
        .unwrap();
    segments[2] = HashSet::from([segment2]);
    segments[5].remove(&segment2);

    let segment3 = *two
        .intersection(&segments[3])
        .cloned()
        .collect::<Vec<char>>()
        .get(0)
        .unwrap();
    segments[3] = HashSet::from([segment3]);
    segments[1].remove(&segment3);

    segments
        .iter()
        .enumerate()
        .map(|(i, segment)| (*segment.iter().next().unwrap(), i))
        .collect::<HashMap<char, usize>>()
}

pub fn part2() {
    // Binary representation of 7 segments
    /*
         0000
        1    2
        1    2
         3333
        4    5
        4    5
         6666
    */
    let numbers = HashMap::from([
        (119, 0),
        (18, 1),
        (93, 2),
        (91, 3),
        (58, 4),
        (107, 5),
        (111, 6),
        (82, 7),
        (127, 8),
        (123, 9),
    ]);
    let lines = read_file_lines("input/08.txt").map(|line| {
        let mut parts = line.split(" | ");
        (
            parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|str| str.to_string())
                .collect::<Vec<String>>(),
            parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|str| str.to_string())
                .collect::<Vec<String>>(),
        )
    });

    let result = lines.fold(0, |sum, (mut digits, output)| {
        let decoded = decode(&mut digits);
        sum + output.iter().enumerate().rev().fold(0, |acc, (i, digit)| {
            let mut lit = 0;
            digit.chars().for_each(|c| {
                lit += 1 << (6 - decoded.get(&c).unwrap());
            });
            let number = numbers.get(&lit).unwrap();
            acc + 10_i32.pow(3 - i as u32) * number
        })
    });

    println!("Day 08 > Part 2: {}", result);
}

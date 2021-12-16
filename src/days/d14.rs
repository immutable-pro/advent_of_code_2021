use crate::utils::read_file_lines;
use std::collections::HashMap;

/*
The incredible pressures at this depth are starting to put a strain on your submarine. The submarine has polymerization equipment that would produce suitable materials to reinforce the submarine, and the nearby volcanically-active caves should even have the necessary input elements in sufficient quantities.

The submarine manual contains instructions for finding the optimal polymer formula; specifically, it offers a polymer template and a list of pair insertion rules (your puzzle input). You just need to work out what polymer would result after repeating the pair insertion process a few times.

For example:

NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C

The first line is the polymer template - this is the starting point of the process.

The following section defines the pair insertion rules. A rule like AB -> C means that when elements A and B are immediately adjacent, element C should be inserted between them. These insertions all happen simultaneously.

So, starting with the polymer template NNCB, the first step simultaneously considers all three pairs:

    The first pair (NN) matches the rule NN -> C, so element C is inserted between the first N and the second N.
    The second pair (NC) matches the rule NC -> B, so element B is inserted between the N and the C.
    The third pair (CB) matches the rule CB -> H, so element H is inserted between the C and the B.

Note that these pairs overlap: the second element of one pair is the first element of the next pair. Also, because all pairs are considered simultaneously, inserted elements are not considered to be part of a pair until the next step.

After the first step of this process, the polymer becomes NCNBCHB.

Here are the results of a few steps using the above rules:

Template:     NNCB
After step 1: NCNBCHB
After step 2: NBCCNBBBCBHCB
After step 3: NBBBCNCCNBBNBNBBCHBHHBCHB
After step 4: NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB

This polymer grows quickly. After step 5, it has length 97; After step 10, it has length 3073. After step 10, B occurs 1749 times, C occurs 298 times, H occurs 161 times, and N occurs 865 times; taking the quantity of the most common element (B, 1749) and subtracting the quantity of the least common element (H, 161) produces 1749 - 161 = 1588.

Apply 10 steps of pair insertion to the polymer template and find the most and least common elements in the result. What do you get if you take the quantity of the most common element and subtract the quantity of the least common element?
*/

type Pair = (char, char);
type Count = HashMap<char, i64>;
type Rules = HashMap<Pair, char>;
type Memo = HashMap<(Pair, i8), Count>;

fn step(pair: &Pair, remaining_steps: i8, count: &mut Count, rules: &Rules, memo: &mut Memo) {
    if let Some(found) = memo.get(&(*pair, remaining_steps)) {
        for (key, value) in found.iter() {
            *count.entry(*key).or_insert(0) += value;
        }
        return;
    }
    if remaining_steps == 0 {
        return;
    }
    let expansion = rules[pair];
    *count.entry(expansion).or_insert(0) += 1;

    let left_pair = (pair.0, expansion);
    let right_pair = (expansion, pair.1);

    let mut left_count = Count::new();
    step(
        &left_pair,
        remaining_steps - 1,
        &mut left_count,
        rules,
        memo,
    );
    memo.insert((left_pair, remaining_steps - 1), left_count.clone());

    let mut right_count = Count::new();
    step(
        &right_pair,
        remaining_steps - 1,
        &mut right_count,
        rules,
        memo,
    );
    memo.insert((right_pair, remaining_steps - 1), right_count.clone());

    for (key, value) in left_count.iter() {
        *count.entry(*key).or_insert(0) += value;
    }
    for (key, value) in right_count.iter() {
        *count.entry(*key).or_insert(0) += value;
    }
}

fn find_min_max(total: &Count) -> (i64, i64) {
    let mut min = i64::MAX;
    let mut max = i64::MIN;
    for (_, value) in total.iter() {
        if *value < min {
            min = *value;
        }
        if *value > max {
            max = *value;
        }
    }
    (min, max)
}

fn parse() -> (String, Rules) {
    let lines = read_file_lines("input/14.txt");
    let mut lines_iter = lines.iter();
    let start = lines_iter.next().unwrap();
    lines_iter.next();
    let mut rules = Rules::new();
    for line in lines_iter {
        let chunks = line.split(" -> ").into_iter().collect::<Vec<&str>>();
        let chars = chunks[0].to_string().chars().collect::<Vec<char>>();
        let value = chunks[1].parse::<char>().unwrap();
        rules.insert((chars[0], chars[1]), value);
    }
    (start.to_string(), rules)
}

fn solve(steps: i8) -> i64 {
    let (start, rules) = parse();
    let mut total = Count::new();
    let mut first: char = '?';
    let mut second: char = '?';
    let mut pairs = Vec::<Pair>::new();
    start.chars().for_each(|c| {
        *total.entry(c).or_insert(0) += 1;
        if first == '?' {
            first = c;
            return;
        }
        second = c;
        pairs.push((first, second));
        first = second;
    });
    let mut memo = Memo::new();
    pairs.iter().for_each(|pair| {
        let mut count = Count::new();
        step(pair, steps, &mut count, &rules, &mut memo);
        for (key, value) in count.iter() {
            *total.entry(*key).or_insert(0) += value;
        }
    });

    let (min, max) = find_min_max(&total);
    max - min
}

pub fn part1() {
    println!("Day 14 > Part 1: {}", solve(10));
}

/*
The resulting polymer isn't nearly strong enough to reinforce the submarine. You'll need to run more steps of the pair insertion process; a total of 40 steps should do it.

In the above example, the most common element is B (occurring 2192039569602 times) and the least common element is H (occurring 3849876073 times); subtracting these produces 2188189693529.

Apply 40 steps of pair insertion to the polymer template and find the most and least common elements in the result. What do you get if you take the quantity of the most common element and subtract the quantity of the least common element?
*/
pub fn part2() {
    println!("Day 14 > Part 2: {}", solve(40));
}

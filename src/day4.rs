use std::{
    ops::Range,
};

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Range<usize> {
    let r: Vec<usize> = input
        .split("-")
        .map(|x| {
            x.to_string()
                .parse::<usize>()
                .unwrap()
        })
        .collect();
    Range { start: r[0], end: r[1] }
}

fn always_increasing(s: &str) -> bool {
    let mut last_max = 0;
    for x in s.chars().map(|x| x.to_digit(10).unwrap()) {
        if last_max > x {
            return false;
        } else {
            last_max = x;
        }
    }
    return true
}

fn has_tuplets(s: &str) -> bool {
    let mut last_seen: char = ' ';
    for c in s.chars() {
        if c == last_seen {
            return true;
        } else {
            last_seen = c;
        }
    }
    return false;
}

fn valid_password(s: &str) -> bool {
    always_increasing(s) && has_tuplets(s)
}

#[aoc(day4, part1)]
fn solve_p1(range: &Range<usize>) -> usize {
    let mut total: usize = 0;
    for n in (range.start)..(range.end) {
        if valid_password(&n.to_string()) {
            total += 1;
        }
    }
    total
}

fn has_twins(s: &str) -> bool {
    let mut counts: Vec<u64> = vec![0; 10];

    for c in s.chars() {
        counts[c.to_digit(10).unwrap() as usize] += 1;
    }

    for c in counts {
        if c == 2 {
            return true;
        }
    }

    false
}

fn valid_strict_password(n: &str) -> bool {
    always_increasing(n) && has_twins(n)
}

#[aoc(day4, part2)]
fn solve_p2(range: &Range<usize>) -> usize {
    let mut total: usize = 0;
    for n in (range.start)..(range.end) {
        if valid_strict_password(&n.to_string()) {
            total += 1;
        }
    }
    total
}

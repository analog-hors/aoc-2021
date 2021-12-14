use std::io::BufRead;
use std::collections::HashMap;

fn parse_template(input: impl BufRead) -> (Vec<char>, HashMap<(char, char), char>) {
    let mut input = input.lines().map(Result::unwrap);
    let init = input.next().unwrap().chars().collect();
    input.next();
    let rules = input
        .map(|line| {
            let (from, to) = line
                .split_once(" -> ")
                .unwrap();
            let mut from = from.chars();
            let mut to = to.chars();
            ((from.next().unwrap(), from.next().unwrap()), to.next().unwrap())
        })
        .collect();
    (init, rules)
}

type LetterCounts = HashMap<char, u64>;

fn add_counts(counts: &mut LetterCounts, other: &LetterCounts) {
    for (&c, &n) in other {
        *counts.entry(c).or_default() += n;
    }
}

fn letter_counts(
    rules: &HashMap<(char, char), char>,
    cache: &mut HashMap<((char, char), u8), LetterCounts>,
    (left, right): (char, char),
    depth: u8
) -> LetterCounts {
    let cache_key = ((left, right), depth);
    let middle = rules.get(&(left, right));
    if depth == 0 || middle.is_none() {
        [(left, 1)].into_iter().collect()
    } else if let Some(counts) = cache.get(&cache_key) {
        counts.clone()
    } else {
        let middle = *middle.unwrap();
        let mut total = letter_counts(rules, cache, (left, middle), depth - 1);
        let right = letter_counts(rules, cache, (middle, right), depth - 1);
        add_counts(&mut total, &right);
        cache.insert(cache_key, total.clone());
        total
    }
}

fn solve(polymer: &[char], rules: &HashMap<(char, char), char>, steps: u8) -> u64 {
    let mut cache = HashMap::new();
    let mut counts = LetterCounts::new();
    for window in polymer.windows(2) {
        let result = letter_counts(
            &rules,
            &mut cache,
            (window[0], window[1]),
            steps
        );
        add_counts(&mut counts, &result);
    }
    *counts.entry(*polymer.last().unwrap()).or_default() += 1;
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

fn part_1(input: impl BufRead) -> u64 {
    let (polymer, rules) = parse_template(input);
    solve(&polymer, &rules, 10)
}

fn part_2(input: impl BufRead) -> u64 {
    let (polymer, rules) = parse_template(input);
    solve(&polymer, &rules, 40)
}

aoc::main!();

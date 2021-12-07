use std::io::BufRead;

fn parse_crabs(input: impl BufRead) -> Vec<i32> {
    let fishes = input.lines().next().unwrap().unwrap();
    fishes.split(',').map(|n| n.parse().unwrap()).collect()
}

fn min_fuel_required(crabs: &[i32], mut fuel_required: impl FnMut(i32, i32) -> i32) -> i32 {
    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();
    (min..=max)
        .map(|p| crabs.iter().map(|&c| fuel_required(c, p)).sum())
        .min()
        .unwrap()
}

fn part_1(input: impl BufRead) -> i32 {
    let crabs = parse_crabs(input);
    min_fuel_required(&crabs, |c, p| (c - p).abs())
}

fn triangular_number(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn part_2(input: impl BufRead) -> i32 {
    let crabs = parse_crabs(input);
    min_fuel_required(&crabs, |c, p| triangular_number((c - p).abs()))
}

aoc::main!();

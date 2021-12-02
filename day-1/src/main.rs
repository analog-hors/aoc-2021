use std::io::BufRead;

fn parse_inputs(input: impl BufRead) -> Vec<i32> {
    input
        .lines()
        .map(|n| n.unwrap().parse().unwrap())
        .collect()
}

fn part_1(input: impl BufRead) -> u32 {
    let input = parse_inputs(input);
    input
        .windows(2)
        .filter(|d| d[0] < d[1])
        .count() as u32
}

fn part_2(input: impl BufRead) -> u32 {
    let input = parse_inputs(input);
    let summed = input
        .windows(3)
        .map(|d| d.iter().sum::<i32>())
        .collect::<Vec<_>>();
    summed
        .windows(2)
        .filter(|d| d[0] < d[1])
        .count() as u32
}

aoc::main!();

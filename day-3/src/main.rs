use std::io::BufRead;
use std::cmp::Ordering;

const RECORD_BITS: usize = 12;

fn parse_inputs(input: impl BufRead) -> impl Iterator<Item=i32> {
    input.lines().map(|n| i32::from_str_radix(&n.unwrap(), 2).unwrap())
}

fn bitset_contains(set: i32, i: usize) -> bool {
    (set & (1 << i)) != 0
}

fn part_1(input: impl BufRead) -> i32 {
    let input = parse_inputs(input);
    let mut occurrences = [0; RECORD_BITS];
    for record in input {
        for i in 0..RECORD_BITS {
            if bitset_contains(record, i) {
                occurrences[i] += 1;
            } else {
                occurrences[i] -= 1;
            }
        }
    }
    let mut gamma = 0;
    for (i, &n) in occurrences.iter().enumerate() {
        if n > 0 {
            gamma |= 1 << i;
        }
    }
    let epsilon = gamma ^ ((1 << RECORD_BITS) - 1);
    gamma * epsilon
}

fn compare_ones_to_zeroes(records: &[i32], i: usize) -> Ordering {
    records
        .iter()
        .map(|&n| if bitset_contains(n, i) { 1 } else { -1 })
        .sum::<i32>()
        .cmp(&0)
}

fn find_value_by(mut records: Vec<i32>, f: impl Fn(&[i32], usize) -> bool) -> i32 {
    for i in (0..RECORD_BITS).rev() {
        let required_bit = f(&records, i);
        records.retain(|&n| bitset_contains(n, i) == required_bit);
        if records.len() == 1 {
            return records[0];
        }
    }
    panic!()
}

fn part_2(input: impl BufRead) -> i32 {
    let input = parse_inputs(input).collect::<Vec<_>>();
    let oxygen = find_value_by(input.clone(), |records, i| {
        match compare_ones_to_zeroes(records, i) {
            Ordering::Greater | Ordering::Equal => true,
            Ordering::Less => false
        }
    });
    let co2 = find_value_by(input.clone(), |records, i| {
        match compare_ones_to_zeroes(&records, i) {
            Ordering::Less => true,
            Ordering::Greater | Ordering::Equal => false
        }
    });
    oxygen * co2
}

aoc::main!();

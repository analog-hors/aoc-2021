use std::io::BufRead;
use std::collections::{HashSet, HashMap};

type Segments = HashSet<char>;

fn parse_segment_sequence(segments: &str) -> Vec<Segments> {
    segments.split_ascii_whitespace().map(|s| s.chars().collect()).collect()
}

fn parse_records(input: impl BufRead) -> impl Iterator<Item=(Vec<Segments>, Vec<Segments>)> {
    input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (attempted, output) = line.split_once(" | ").unwrap();
            (parse_segment_sequence(attempted), parse_segment_sequence(output))
        })
}

fn into_key(s: &Segments) -> Vec<char> {
    let mut s: Vec<char> = s.iter().copied().collect();
    s.sort_unstable();
    s
}

fn part_1(input: impl BufRead) -> usize {
    let records = parse_records(input);
    let mut ones_fours_sevens_eights = 0;
    for (attempted, output) in records {
        let mut decoded = HashMap::new();
        for segments in &attempted {
            decoded.insert(into_key(segments), match segments.len() {
                2 => 1,
                4 => 4,
                3 => 7,
                7 => 8,
                _ => continue
            });
        }
        ones_fours_sevens_eights += output
            .iter()
            .filter_map(|d| decoded.get(&into_key(d)))
            .count();
    }
    ones_fours_sevens_eights
}

fn find_and_remove(segments: &mut Vec<Segments>, f: impl FnMut(&Segments) -> bool) -> Segments {
    let position = segments.iter()
        .position(f)
        .unwrap();
    segments.swap_remove(position)
}

fn part_2(input: impl BufRead) -> i32 {
    let records = parse_records(input);
    let mut total = 0;
    for (mut attempted, output) in records {
        let one = find_and_remove(
            &mut attempted,
            |s| s.len() == 2
        );
        let four = find_and_remove(
            &mut attempted,
            |s| s.len() == 4
        );
        let seven = find_and_remove(
            &mut attempted,
            |s| s.len() == 3
        );
        let eight = find_and_remove(
            &mut attempted,
            |s| s.len() == 7
        );
        let nine = find_and_remove(
            &mut attempted,
            |s| s.len() == 6 && s.is_superset(&four)
        );
        let zero = find_and_remove(
            &mut attempted,
            |s| s.len() == 6 && s.is_superset(&one)
        );
        let six = find_and_remove(
            &mut attempted,
            |s| s.len() == 6
        );
        let five = find_and_remove(
            &mut attempted,
            |s| s.is_subset(&six)
        );
        let three = find_and_remove(
            &mut attempted,
            |s| s.is_superset(&one)
        );
        let two = attempted.pop().unwrap();
        let decoded = [
                zero, one, two, three, four,
                five, six, seven, eight, nine
            ].into_iter()
            .enumerate()
            .map(|(i, s)| (into_key(&s), i as i32))
            .collect::<HashMap<_, _>>();
        total += output
            .iter()
            .rev()
            .enumerate()
            .map(|(i, d)| decoded.get(&into_key(d)).unwrap() * 10i32.pow(i as u32))
            .sum::<i32>();
    }
    total
}

aoc::main!();

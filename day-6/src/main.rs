use std::io::BufRead;

fn parse_ages(input: impl BufRead) -> Vec<u64> {
    let fishes = input.lines().next().unwrap().unwrap();
    fishes.split(',').map(|n| n.parse().unwrap()).collect()
}

fn simulate_fishes(init_fishes: &[u64], days: u32) -> u64 {
    let mut fishes = [0; 9];
    for &fish in init_fishes {
        fishes[fish as usize] += 1;
    }
    for _ in 0..days {
        let new_fishes = fishes[0];
        fishes[0] = 0;
        fishes.rotate_left(1);
        fishes[8] += new_fishes;
        fishes[6] += new_fishes;
    }
    fishes.iter().sum()
}

fn part_1(input: impl BufRead) -> u64 {
    let input = parse_ages(input);
    simulate_fishes(&input, 80)
}

fn part_2(input: impl BufRead) -> u64 {
    let input = parse_ages(input);
    simulate_fishes(&input, 256)
}

aoc::main!();

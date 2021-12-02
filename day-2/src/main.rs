use std::io::BufRead;

enum Command {
    Forward(i32),
    Up(i32),
    Down(i32)
}

fn parse_inputs(input: impl BufRead) -> impl Iterator<Item=Command> {
    input
        .lines()
        .map(|command| {
            let command = command.unwrap();
            let (kind, n) = command.split_once(' ').unwrap();
            let n = n.parse().unwrap();
            match kind {
                "forward" => Command::Forward(n),
                "up"      => Command::Up(n),
                "down"    => Command::Down(n),
                _ => panic!()
            }
        })
}

fn part_1(input: impl BufRead) -> i32 {
    let input = parse_inputs(input);
    let mut h_pos = 0;
    let mut depth = 0;
    for command in input {
        match command {
            Command::Forward(n) => h_pos += n,
            Command::Up(n) =>      depth -= n,
            Command::Down(n) =>    depth += n
        }
    }
    h_pos * depth
}

fn part_2(input: impl BufRead) -> i32 {
    let input = parse_inputs(input);
    let mut aim = 0;
    let mut h_pos = 0;
    let mut depth = 0;
    for command in input {
        match command {
            Command::Forward(n) => {
                h_pos += n;
                depth += aim * n;
            },
            Command::Up(n) => aim -= n,
            Command::Down(n) => aim += n
        }
    }
    println!("{}", depth);

    h_pos * depth
}

aoc::main!();

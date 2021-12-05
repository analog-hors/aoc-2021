use std::io::BufRead;

struct BingoGame {
    numbers: Vec<i32>,
    boards: Vec<MarkedBoard>
}

#[derive(Default)]
struct MarkedBoard([[(i32, bool); 5]; 5]);

impl MarkedBoard {
    fn mark(&mut self, number: i32) {
        let matching = self.0
            .iter_mut()
            .flatten()
            .filter(|(n, _)| *n == number);
        for (_, marked) in matching {
            *marked = true;
        }
    }

    fn won(&self) -> bool {
        self.0.iter().any(|&r| r.iter().all(|(_, m)| *m))
            || (0..5).any(|c| self.0.iter().all(|r| r[c].1))
    }

    fn score(&self) -> i32 {
        self.0.iter()
            .flatten()
            .filter_map(|&(n, m)| if !m { Some(n) } else { None })
            .sum()
    }
}

fn parse_inputs(input: impl BufRead) -> BingoGame {
    let mut lines = input.lines().map(Result::unwrap);
    let numbers: Vec<i32> = lines.next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let mut boards = Vec::new();
    while lines.next().is_some() {
        let mut board = MarkedBoard::default();
        for (row, line) in board.0.iter_mut().zip(&mut lines) {
            let numbers = line
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap());
            for (cell, number) in row.iter_mut().zip(numbers) {
                *cell = (number, false);
            }
        }
        boards.push(board);
    }
    BingoGame {
        numbers,
        boards
    }
}

fn part_1(input: impl BufRead) -> i32 {
    let mut input = parse_inputs(input);
    for &number in &input.numbers {
        for board in &mut input.boards {
            board.mark(number);
            if board.won() {
                return board.score() * number;
            }
        }
    }
    panic!()
}

fn part_2(input: impl BufRead) -> i32 {
    let mut input = parse_inputs(input);
    for &number in &input.numbers {
        for board in &mut input.boards {
            board.mark(number);
        }
        if input.boards.len() > 1 {
            input.boards.retain(|b| !b.won());
        } else if input.boards[0].won() {
            return input.boards[0].score() * number;
        }
    }
    panic!()
}

aoc::main!();

use std::io::BufRead;
use std::collections::HashMap;

type Cell = (i32, i32);

fn parse_grid(input: impl BufRead) -> HashMap<Cell, u32> {
    let mut grid = HashMap::new();
    for (y, row) in input.lines().enumerate() {
        let row = row.unwrap();
        for (x, c) in row.chars().enumerate() {
            grid.insert((x as i32, y as i32), c.to_digit(10).unwrap());
        }
    }
    grid
}

fn neighbours((x, y): Cell) -> [Cell; 8] {
    [
        (x + 1, y),
        (x - 1, y),
        (x, y + 1),
        (x, y - 1),
        (x + 1, y + 1),
        (x + 1, y - 1),
        (x - 1, y + 1),
        (x - 1, y - 1)
    ]
}

fn step_grid(grid: &mut HashMap<Cell, u32>) -> usize {
    let mut flashes = 0;
    for octopus in grid.values_mut() {
        *octopus += 1;
    }
    loop {
        let mut flashed = Vec::new();
        for (&cell, octopus) in grid.iter_mut() {
            if *octopus > 9 {
                *octopus = 0;
                flashed.push(cell);
            }
        }
        if flashed.is_empty() {
            break;
        }
        flashes += flashed.len();
        for cell in flashed {
            for neighbour in neighbours(cell) {
                if let Some(neighbour) = grid.get_mut(&neighbour) {
                    if *neighbour != 0 {
                        *neighbour += 1;
                    }
                }
            }
        }
    }
    flashes
}

fn part_1(input: impl BufRead) -> usize {
    let mut grid = parse_grid(input);
    (0..100).map(|_| step_grid(&mut grid)).sum()
}

fn part_2(input: impl BufRead) -> usize {
    let mut grid = parse_grid(input);
    (1..).find(|_| step_grid(&mut grid) == grid.len()).unwrap()
}

aoc::main!();

use std::io::BufRead;
use std::collections::{HashMap, HashSet};

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

fn neighbours((x, y): Cell) -> [Cell; 4] {
    [
        (x + 1, y),
        (x - 1, y),
        (x, y + 1),
        (x, y - 1)
    ]
}

fn low_points(grid: &HashMap<Cell, u32>) -> impl Iterator<Item=(Cell, u32)> + '_ {
    grid.iter()
        .map(|(&c, &h)| (c, h))
        .filter(|&(cell, height)| {
            neighbours(cell)
                .iter()
                .flat_map(|c| grid.get(c))
                .all(|&h| h > height)
        })
}

fn part_1(input: impl BufRead) -> u32 {
    let grid = parse_grid(input);
    low_points(&grid).map(|(_, h)| h + 1).sum()
}

fn flood_fill(grid: &HashMap<Cell, u32>, cell: Cell) -> HashSet<Cell> {
    fn visit(grid: &HashMap<Cell, u32>, visited: &mut HashSet<Cell>, cell: Cell) {
        if visited.contains(&cell) || matches!(grid.get(&cell), None | Some(9)) {
            return;
        }
        visited.insert(cell);
        for neighbour in neighbours(cell) {
            visit(grid, visited, neighbour);
        }
    }
    let mut visited = HashSet::new();
    visit(grid, &mut visited, cell);
    visited
}

fn part_2(input: impl BufRead) -> usize {
    let grid = parse_grid(input);
    let mut basins = low_points(&grid)
        .map(|(c, _)| flood_fill(&grid, c))
        .collect::<Vec<_>>();
    basins.sort_unstable_by_key(|b| b.len());
    basins.iter().rev().take(3).map(|b| b.len()).product()
}

aoc::main!();

use std::io::BufRead;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::{Ord, PartialOrd, Ordering};

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

#[derive(Clone, Copy, PartialEq, Eq)]
struct Node {
    target: Cell,
    cell: Cell,
    cost: u32
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        fn dist((x, y): Cell, (tx, ty): Cell) -> u32 {
            ((tx - x) + (ty - y)) as u32
        }
        let self_cost = self.cost + dist(self.cell, self.target);
        let other_cost = other.cost + dist(other.cell, self.target);
        (self_cost).cmp(&other_cost).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn cost_to_target(grid: &HashMap<Cell, u32>, start: Cell, end: Cell) -> Option<u32> {
    let mut visited = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(Node {
        target: end,
        cell: start,
        cost: 0
    });
    while let Some(node) = queue.pop() {
        if node.cell == end {
            return Some(node.cost);
        }
        visited.insert(node.cell, node.cost);
        for neighbour in neighbours(node.cell) {
            if visited.get(&neighbour).is_some() {
                continue;
            }
            if let Some(&cost) = grid.get(&neighbour) {
                queue.push(Node {
                    target: end,
                    cell: neighbour,
                    cost: node.cost + cost
                });
            }
        }
    }
    None
}

fn part_1(input: impl BufRead) -> u32 {
    let grid = parse_grid(input);
    let end_x = grid.keys().map(|c| c.0).max().unwrap();
    let end_y = grid.keys().map(|c| c.1).max().unwrap();
    cost_to_target(&grid, (0, 0), (end_x, end_y)).unwrap()
}

fn part_2(input: impl BufRead) -> u32 {
    let init_grid = parse_grid(input);
    let init_w = init_grid.keys().map(|c| c.0).max().unwrap() + 1;
    let init_h = init_grid.keys().map(|c| c.1).max().unwrap() + 1;
    let mut grid = HashMap::new();
    let w = init_w * 5;
    let h = init_h * 5;
    for y in 0..h {
        for x in 0..w {
            let base = *init_grid.get(&(x % init_w, y % init_h)).unwrap();
            let increase = (x / init_w + y / init_h) as u32;
            grid.insert((x, y), (base + increase - 1) % 9 + 1);
        }
    }
    cost_to_target(&grid, (0, 0), (w - 1, h - 1)).unwrap()
}

aoc::main!();

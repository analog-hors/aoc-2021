use std::io::BufRead;
use std::collections::HashSet;
use std::ops::RangeInclusive;

type Cell = (i32, i32);

struct Image {
    background_is_white: bool,
    x_bounds: RangeInclusive<i32>,
    y_bounds: RangeInclusive<i32>,
    cells: HashSet<Cell>
}

impl Image {
    fn new(cells: HashSet<Cell>) -> Self {
        let mut this = Self {
            background_is_white: false,
            x_bounds: 0..=0,
            y_bounds: 0..=0,
            cells
        };
        this.update_bounds();
        this
    }

    fn update_bounds(&mut self) {
        let min_x = self.cells.iter().map(|&(x, _)| x).min().unwrap();
        let max_x = self.cells.iter().map(|&(x, _)| x).max().unwrap();
        let min_y = self.cells.iter().map(|&(_, y)| y).min().unwrap();
        let max_y = self.cells.iter().map(|&(_, y)| y).max().unwrap();
        self.x_bounds = min_x..=max_x;
        self.y_bounds = min_y..=max_y;
    }

    fn get(&self, (x, y): Cell) -> bool {
        if self.x_bounds.contains(&x) && self.y_bounds.contains(&y) {
            self.cells.contains(&(x, y))
        } else {
            self.background_is_white
        }
    }

    fn calculate_pixel(&self, map: &[bool; 512], (x, y): Cell) -> bool {
        let neighbors = [
            (x - 1, y - 1), (x    , y - 1), (x + 1, y - 1),
            (x - 1, y    ), (x    , y    ), (x + 1, y    ),
            (x - 1, y + 1), (x    , y + 1), (x + 1, y + 1),
        ];
        let index = neighbors
            .into_iter()
            .zip((0..9).rev())
            .fold(0, |n, (c, i)| n | ((self.get(c) as usize) << i));
        map[index]
    }

    fn enhance(&self, map: &[bool; 512]) -> Image {
        let mut new_image = Self {
            background_is_white: self.background_is_white ^ map[0],
            x_bounds: 0..=0,
            y_bounds: 0..=0,
            cells: HashSet::new()
        };
        let x_positions = (*self.x_bounds.start() - 1)..=(*self.x_bounds.end() + 1);
        let y_positions = (*self.y_bounds.start() - 1)..=(*self.y_bounds.end() + 1);
        for x in x_positions {
            for y in y_positions.clone() {
                if self.calculate_pixel(map, (x, y)) {
                    new_image.cells.insert((x, y));
                }
            }
        }
        new_image.update_bounds();
        new_image
    }
}

fn parse_input(input: impl BufRead) -> ([bool; 512], Image) {
    let mut lines = input.lines().map(Result::unwrap);
    let map = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| c == '#')
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let mut cells = HashSet::new();
    for (y, row) in lines.enumerate() {
        for (x, cell) in row.chars().enumerate() {
            if cell == '#' {
                cells.insert((x as i32, y as i32));
            }
        }
    }
    (map, Image::new(cells))
}

fn part_1(input: impl BufRead) -> usize {
    let (map, image) = parse_input(input);
    image.enhance(&map).enhance(&map).cells.len()
}

fn part_2(input: impl BufRead) -> usize {
    let (map, mut image) = parse_input(input);
    for _ in 0..50 {
        image = image.enhance(&map);
    }
    image.cells.len()
}

aoc::main!();

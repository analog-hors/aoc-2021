use std::io::BufRead;
use std::collections::HashMap;

type Point = (i32, i32);

fn parse_point(point: &str) -> Point {
    let (x, y) = point.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

type Line = (Point, Point);

fn parse_lines(input: impl BufRead) -> impl Iterator<Item=Line> {
    input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (from, to) = line
                .split_once(" -> ")
                .unwrap();
            (parse_point(from), parse_point(to))
        })
}

fn update_points(points: &mut HashMap<Point, u32>, new_points: impl Iterator<Item=Point>) {
    for point in new_points {
        *points.entry(point).or_default() += 1;
    }
}

fn part_1(input: impl BufRead) -> usize {
    let lines = parse_lines(input);
    let mut points = HashMap::new();
    for ((x1, y1), (x2, y2)) in lines {
        if x1 == x2 {
            update_points(&mut points, (y1.min(y2)..=y1.max(y2)).map(|y| (x1, y)));
        } else if y1 == y2 {
            update_points(&mut points, (x1.min(x2)..=x1.max(x2)).map(|x| (x, y1)));
        }
    }
    points.values().filter(|&&n| n > 1).count()
}

fn part_2(input: impl BufRead) -> usize {
    let lines = parse_lines(input);
    let mut points = HashMap::new();
    for ((x1, y1), (x2, y2)) in lines {
        if x1 == x2 {
            update_points(&mut points, (y1.min(y2)..=y1.max(y2)).map(|y| (x1, y)));
        } else if y1 == y2 {
            update_points(&mut points, (x1.min(x2)..=x1.max(x2)).map(|x| (x, y1)));
        } else {
            let x = x1.min(x2)..=x1.max(x2);
            let y = y1.min(y2)..=y1.max(y2);
            if (x1 < x2) == (y1 < y2) {
                update_points(&mut points, x.zip(y));
            } else {
                update_points(&mut points, x.rev().zip(y));
            }
        }
    }
    points.values().filter(|&&n| n > 1).count()
}

aoc::main!();

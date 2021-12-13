use std::io::BufRead;
use std::collections::HashSet;

type Point = (i32, i32);

#[derive(Debug)]
enum Fold {
    X(i32),
    Y(i32)
}

fn parse_input(input: impl BufRead) -> (Vec<Point>, Vec<Fold>) {
    let mut input = input.lines().map(Result::unwrap);
    let points = (&mut input)
        .take_while(|p| !p.is_empty())
        .map(|p| {
            let (x, y) = p.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();
    let folds = input
        .map(|f| {
            let (fold, n) = f.split_once('=').unwrap();
            let n = n.parse().unwrap();
            match fold {
                "fold along x" => Fold::X(n),
                "fold along y" => Fold::Y(n),
                _ => panic!()
            }
        })
        .collect();
    (points, folds)
}

fn part_1(input: impl BufRead) -> usize {
    let (mut points, folds) = parse_input(input);
    for (px, py) in &mut points {
        match *folds.first().unwrap() {
            Fold::X(fx) => if *px > fx { *px = fx - (*px - fx) },
            Fold::Y(fy) => if *py > fy { *py = fy - (*py - fy) },
        }
    }
    points.into_iter().collect::<HashSet<_>>().len()
}

fn part_2(input: impl BufRead) -> String {
    let (mut points, folds) = parse_input(input);
    for fold in folds {
        for (px, py) in &mut points {
            match fold {
                Fold::X(fx) => if *px > fx { *px = fx - (*px - fx) },
                Fold::Y(fy) => if *py > fy { *py = fy - (*py - fy) },
            }
        }
    }
    let points = points.into_iter().collect::<HashSet<_>>();
    let w = *points.iter().map(|(x, _)| x).max().unwrap() + 1;
    let h = *points.iter().map(|(_, y)| y).max().unwrap() + 1;
    let mut output = String::new();
    for y in 0..h {
        for x in 0..w {
            if points.contains(&(x, y)) {
                output.push('#');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }
    output
}

aoc::main!();

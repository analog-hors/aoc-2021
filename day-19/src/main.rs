use std::io::BufRead;
use std::collections::{HashSet, HashMap};

type Point = (i32, i32, i32);

fn all_rotations((x, y, z): Point) -> [Point; 24] {
    [
        (-x, -y,  z), (-x, -z, -y), (-x,  y, -z), (-x,  z,  y),
        (-y, -x, -z), (-y, -z,  x), (-y,  x,  z), (-y,  z, -x),
        (-z, -x,  y), (-z, -y, -x), (-z,  x, -y), (-z,  y,  x),
        ( x, -y, -z), ( x, -z,  y), ( x,  y,  z), ( x,  z, -y),
        ( y, -x,  z), ( y, -z, -x), ( y,  x, -z), ( y,  z,  x),
        ( z, -x, -y), ( z, -y,  x), ( z,  x,  y), ( z,  y, -x),
    ]
}

fn all_scanner_rotations(scanner: &[Point]) -> [Vec<Point>; 48] {
    const VEC: Vec<Point> = Vec::new();
    let mut scanners = [VEC; 48];
    for &p in scanner {
        for (&p, s) in all_rotations(p).iter().zip(&mut scanners) {
            s.push(p);
        }
    }
    scanners
}

fn parse_point(s: &str) -> Point {
    let mut p = s.split(',').map(|n| n.parse().unwrap());
    (p.next().unwrap(), p.next().unwrap(), p.next().unwrap())
}

fn parse_scanners(input: impl BufRead) -> Vec<Vec<Point>> {
    let mut input = input.lines().map(Result::unwrap);
    let mut scanners = Vec::new();
    while input.next().is_some() {
        let beacons = (&mut input)
            .take_while(|s| !s.is_empty())
            .map(|s| parse_point(&s))
            .collect();
        scanners.push(beacons)
    }
    scanners
}

fn try_anchor(anchor: &HashSet<Point>, scanner: &[Point]) -> Option<(Vec<Point>, Point)> {
    for scanner in all_scanner_rotations(scanner) {
        for &(apx, apy, apz) in anchor.iter() {
            for &(spx, spy, spz) in &scanner {                
                let mut normalized_scanner = scanner.clone();
                for (x, y, z) in &mut normalized_scanner {
                    *x -= spx;
                    *y -= spy;
                    *z -= spz;
                    *x += apx;
                    *y += apy;
                    *z += apz;
                }
                let common_points = normalized_scanner
                    .iter()
                    .filter(|&p| anchor.contains(p))
                    .count();
                if common_points >= 12 {
                    return Some((normalized_scanner, (apx - spx, apy - spy, apz - spz)));
                }
            }
        }
    }
    None
}

fn part_1(input: impl BufRead) -> usize {
    let mut scanners = parse_scanners(input);
    let mut anchor = scanners.pop().unwrap().into_iter().collect();
    while !scanners.is_empty() {
        for i in 0..scanners.len() {
            if let Some((scanner, _)) = try_anchor(&mut anchor, &scanners[i]) {
                scanners.swap_remove(i);
                anchor.extend(scanner);
                break;
            }
        }
    }
    anchor.len()
}

fn manhattan_dist((x1, y1, z1): Point, (x2, y2, z2): Point) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()
}

fn part_2(input: impl BufRead) -> i32 {
    let mut scanners = parse_scanners(input);
    let mut anchor = scanners.pop().unwrap().iter().copied().collect();
    let mut scanner_positions = vec![(0, 0 ,0 )];
    while !scanners.is_empty() {
        for i in 0..scanners.len() {
            if let Some((scanner, pos)) = try_anchor(&mut anchor, &scanners[i]) {
                scanners.swap_remove(i);
                anchor.extend(scanner);
                scanner_positions.push(pos);
                break;
            }
        }
    }
    let mut max_dist = 0;
    for &a in &scanner_positions {
        for &b in &scanner_positions {
            max_dist = max_dist.max(manhattan_dist(a, b));
        }
    }
    max_dist
}

aoc::main!();

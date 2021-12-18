use std::io::BufRead;
use std::ops::RangeInclusive;
use std::cmp::Ordering;

type TargetAxis = RangeInclusive<i32>;

fn parse_target(input: impl BufRead) -> (TargetAxis, TargetAxis) {    
    fn parse_target_axis(s: &str) -> RangeInclusive<i32> {
        let (_, range) = s.split_once('=').unwrap();
        let (from, to) = range.split_once("..").unwrap();
        from.parse().unwrap()..=to.parse().unwrap()
    }
    let target = input.lines().next().unwrap().unwrap();
    let (_, target) = target.split_once(": ").unwrap();
    let (x, y) = target.split_once(", ").unwrap();
    (parse_target_axis(x), parse_target_axis(y))
}

fn final_x(vx: i32) -> i32 {
    vx * (vx + 1) / 2
}

fn max_height(mut vx: i32, mut vy: i32, tx: &TargetAxis, ty: &TargetAxis) -> Option<i32> {
    let mut x = 0;
    let mut y = 0;
    let mut max_y = y;
    loop {
        if tx.contains(&x) && ty.contains(&y) {
            return Some(max_y);
        }
        if y < *ty.start() || x > *tx.end() {
            return None;
        }
        x += vx;
        y += vy;
        match vx.cmp(&0) {
            Ordering::Greater => vx -= 1,
            Ordering::Less    => vx += 1,
            _ => {}
        }
        vy -= 1;
        max_y = max_y.max(y);
    }
}

fn part_1(input: impl BufRead) -> i32 {
    let (tx, ty) = parse_target(input);
    let vx = (0..).find(|&vx| tx.contains(&final_x(vx))).unwrap();
    let vy_lbound = ty.start().abs();
    (-vy_lbound..=vy_lbound)
        .rev()
        .flat_map(|vy| max_height(vx, vy, &tx, &ty))
        .next()
        .unwrap()
}

fn part_2(input: impl BufRead) -> i32 {
    let (tx, ty) = parse_target(input);
    let mut valid_velocities = 0;
    let vy_lbound = ty.start().abs();
    for vx in 0..=*tx.end() {
        for vy in -vy_lbound..=vy_lbound {
            if max_height(vx, vy, &tx, &ty).is_some() {
                valid_velocities += 1;
            }
        }
    }
    valid_velocities
}

aoc::main!();

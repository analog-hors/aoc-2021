use std::io::BufRead;
use std::collections::HashSet;
use std::ops::RangeInclusive;

type Cuboid = [RangeInclusive<i32>; 3];

fn parse_instructions(input: impl BufRead) -> impl Iterator<Item=(bool, Cuboid)> {
    input
        .lines()
        .map(|instruction| {
            let instruction = instruction.unwrap();
            let (state, dimensions) = instruction.split_once(' ').unwrap();
            let state = state == "on";
            let mut dimensions = dimensions
                .split(',')
                .map(|range| {
                    let (_, range) = range.split_once('=').unwrap();
                    let (from, to) = range.split_once("..").unwrap();
                    from.parse().unwrap()..=to.parse().unwrap()
                });
            let x = dimensions.next().unwrap();
            let y = dimensions.next().unwrap();
            let z = dimensions.next().unwrap();
            (state, [x, y, z])
        })
}

fn constrain(r: &RangeInclusive<i32>) -> RangeInclusive<i32> {
    (*r.start()).max(-50)..=(*r.end()).min(50)
}

fn part_1(input: impl BufRead) -> usize {
    let instructions = parse_instructions(input);
    let mut cubes = HashSet::new();
    for (state, [x, y, z]) in instructions {
        for x in constrain(&x) {
            for y in constrain(&y) {
                for z in constrain(&z) {
                    let cube = [x, y, z];
                    if state {
                        cubes.insert(cube);
                    } else {
                        cubes.remove(&cube);
                    }
                }
            }
        }
    }
    cubes.len()
}

fn overlap(c1: &Cuboid, c2: &Cuboid) -> Option<Cuboid> {
    let mut overlap = [0..=0, 0..=0, 0..=0];
    let ranges = overlap.iter_mut().zip(c1.iter().zip(c2.iter()));
    for (o, (r1, r2)) in ranges {
        if r2.start() > r1.end() || r1.start() > r2.end() {
            return None;
        }
        *o = *r1.start().max(r2.start())..=*r1.end().min(r2.end());
    }
    Some(overlap)
}

fn axis_length(axis: &RangeInclusive<i32>) -> i32 {
    *axis.end() - *axis.start() + 1
}

fn volume([x, y, z]: &Cuboid) -> u64 {
    axis_length(x) as u64 * axis_length(y) as u64 * axis_length(z)as u64 
}

fn part_2(input: impl BufRead) -> u64 {
    let instructions = parse_instructions(input);
    let mut cuboids = HashSet::new();
    for (is_adding, cuboid) in instructions {
        let mut new_cuboids = HashSet::new();
        let mut new_cuboid_is_encased = false;
        for intersecting in cuboids {
            if let Some(overlap) = overlap(&intersecting, &cuboid) {
                if is_adding && overlap == cuboid {
                    //The new cuboid adds, but it is completely encased.
                    //Adding it is thus useless.
                    new_cuboid_is_encased = true;
                    break;
                }
                if overlap == intersecting {
                    //The new cuboid completely encases the old one, so just ignore it.
                    //This is true regardless of whether the new one adds or subtracts.
                    continue;
                }
                let [ix, iy, iz] = intersecting;
                let [ox, oy, oz] = overlap;
                if ix.start() < ox.start() {
                    let from = *ix.start();
                    let to = *ox.start() - 1;
                    new_cuboids.insert([from..=to, iy.clone(), iz.clone()]);
                }
                if ox.end() < ix.end() {
                    let from = *ox.end() + 1;
                    let to = *ix.end();
                    new_cuboids.insert([from..=to, iy.clone(), iz.clone()]);
                }

                if iy.start() < oy.start() {
                    let from = *iy.start();
                    let to = *oy.start() - 1;
                    new_cuboids.insert([ox.clone(), from..=to, iz.clone()]);
                }
                if oy.end() < iy.end() {
                    let from = *oy.end() + 1;
                    let to = *iy.end();
                    new_cuboids.insert([ox.clone(), from..=to, iz.clone()]);
                }

                if iz.start() < oz.start() {
                    let from = *iz.start();
                    let to = *oz.start() - 1;
                    new_cuboids.insert([ox.clone(), oy.clone(), from..=to]);
                }
                if oz.end() < iz.end() {
                    let from = *oz.end() + 1;
                    let to = *iz.end();
                    new_cuboids.insert([ox.clone(), oy.clone(), from..=to]);
                }
            } else {
                //This doesn't overlap, so just add it unchanged.
                new_cuboids.insert(intersecting);
            }
        }
        if is_adding && !new_cuboid_is_encased {
            new_cuboids.insert(cuboid);
        }
        cuboids = new_cuboids;
    }
    cuboids.iter().map(volume).sum()
}

aoc::main!();

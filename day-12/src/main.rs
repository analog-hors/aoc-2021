use std::io::BufRead;
use std::collections::HashMap;

type CaveSystem = HashMap<String, Vec<String>>;

fn parse_cave_system(input: impl BufRead) -> CaveSystem {
    let mut caves = CaveSystem::new();
    for connection in input.lines().map(Result::unwrap) {
        let (a, b) = connection.split_once('-').unwrap();
        caves.entry(a.to_owned()).or_default().push(b.to_owned());
        caves.entry(b.to_owned()).or_default().push(a.to_owned());
    }
    caves
}

fn unique_paths_to_end(
    caves: &CaveSystem,
    mut can_visit_cave: impl FnMut(&str, &mut Vec<String>) -> bool
) -> u32 {
    fn visit(
        caves: &CaveSystem,
        paths: &mut u32,
        can_visit_cave: &mut impl FnMut(&str, &mut Vec<String>) -> bool,
        path: &mut Vec<String>,
        cave: &str
    ) {
        if !can_visit_cave(cave, path) {
            return;
        }
        if cave == "end" {
            *paths += 1;
            return;
        }
        path.push(cave.to_owned());
        for neighbour in caves.get(cave).unwrap() {
            visit(caves, paths, can_visit_cave, path, neighbour);
        }
        path.pop();
    }
    let mut path = Vec::new();
    let mut paths = 0;
    visit(caves, &mut paths, &mut can_visit_cave, &mut path, "start");
    paths
}

fn cave_is_small(cave: &str) -> bool {
    cave.chars().all(|c| c.is_lowercase())
}

fn part_1(input: impl BufRead) -> u32 {
    let caves = parse_cave_system(input);
    unique_paths_to_end(&caves, |cave, path| {
        !(cave_is_small(cave) && path.iter().any(|c| c.as_str() == cave))
    })
}

fn visit_budget(cave: &str, path: &[String]) -> u32 {
    if cave == "start" {
        return 1;
    }
    for (i, cave_a) in path.iter().enumerate() {
        if !cave_is_small(cave_a) {
            continue;
        }
        for (j, cave_b) in path.iter().enumerate() {
            if i == j {
                continue;
            }
            if cave_a == cave_b {
                return 1;
            }
        }
    }
    2
}

fn part_2(input: impl BufRead) -> u32 {
    let caves = parse_cave_system(input);
    unique_paths_to_end(&caves, |cave, path| {
        if cave_is_small(cave) {
            let visited = path.iter().filter(|c| c.as_str() == cave).count();
            (visited as u32) < visit_budget(cave, path)
        } else {
            true
        }
    })
}

aoc::main!();

use std::io::BufRead;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct PlayerState {
    pawn: u32,
    score: u32
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct GameState {
    p1: PlayerState,
    p2: PlayerState,
    is_p1_turn: bool,
}

impl GameState {
    fn next(&self, die_result: u32) -> Self {
        let mut new = *self;
        let player = if new.is_p1_turn {
            &mut new.p1
        } else {
            &mut new.p2
        };
        player.pawn = ((player.pawn + die_result - 1) % 10) + 1;
        player.score += player.pawn;
        new.is_p1_turn = !new.is_p1_turn;
        new
    }
}

fn parse_game_state(input: impl BufRead) -> GameState {
    let mut input = input
        .lines()
        .map(|s| {
            let s = s.unwrap();
            let (_, p) = s.split_once(": ").unwrap();
            p.parse().unwrap()
        });
    GameState {
        p1: PlayerState {
            pawn: input.next().unwrap(),
            score: 0
        },
        p2: PlayerState {
            pawn: input.next().unwrap(),
            score: 0
        },
        is_p1_turn: true
    }
}

fn part_1(input: impl BufRead) -> u32 {
    let mut game = parse_game_state(input);
    let mut die = (1..=100).cycle();
    let mut die_rolls = 0;
    while game.p1.score.max(game.p2.score) < 1000 {
        game = game.next((&mut die).take(3).sum());
        die_rolls += 3;
    }
    game.p1.score.min(game.p2.score) * die_rolls
}

fn part_2(input: impl BufRead) -> u64 {
    let game_state = parse_game_state(input);
    let mut universes = HashMap::new();
    universes.insert(game_state, 1);
    let mut p1_wins = 0;
    let mut p2_wins = 0;
    while !universes.is_empty() {
        let mut next = HashMap::new();
        for (universe, count) in universes {
            for d1 in 1..=3 {
                for d2 in 1..=3 {
                    for d3 in 1..=3 {
                        let universe = universe.next(d1 + d2 + d3);
                        if universe.p1.score >= 21 {
                            p1_wins += count;
                        } else if universe.p2.score >= 21 {
                            p2_wins += count;
                        } else {
                            *next.entry(universe).or_default() += count;
                        }
                    }
                }
            }
        }
        universes = next;
    }
    p1_wins.max(p2_wins)
}

aoc::main!();

use std::io::BufRead;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Amphipod {
    A,
    B,
    C,
    D
}

impl Amphipod {
    fn energy(self) -> u32 {
        match self {
            Self::A => 1,
            Self::B => 10,
            Self::C => 100,
            Self::D => 1000
        }
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
struct Burrow {
    hallway: [Option<Amphipod>; 7],
    rooms: [Vec<Amphipod>; 4],
    max_room_size: usize
}

const TRUE_SLOT_POSITIONS: [i32; 7] = [0, 1,    3,    5,    7,    9, 10];
const TRUE_ROOM_POSITIONS: [i32; 4] = [      2,    4,    6,    8,      ];

impl Burrow {
    fn room_to_hallway_moves(&self, room: usize) -> impl Iterator<Item=usize> + '_ {
        let room_vec = &self.rooms[room];
        if room_vec.is_empty() {
            return None.into_iter().flatten();
        }
        let amphipod = *room_vec.last().unwrap();
        let correct_room = match amphipod {
            Amphipod::A => 0,
            Amphipod::B => 1,
            Amphipod::C => 2,
            Amphipod::D => 3
        };
        if room == correct_room {
            if room_vec.iter().all(|&a| a == amphipod) {
                return None.into_iter().flatten();
            }
        }

        let mid = 2 + room;
        let left = 0..mid;
        let right = mid..self.hallway.len();
        let left_moves = left
            .rev()
            .take_while(|&s| self.hallway[s].is_none());
        let right_moves = right
            .take_while(|&s| self.hallway[s].is_none());
        Some(left_moves.chain(right_moves)).into_iter().flatten()
    }

    fn room_to_hallway(&self, room: usize, slot: usize) -> (Self, u32) {
        let mut new = self.clone();
        let true_room_pos = TRUE_ROOM_POSITIONS[room];
        let true_slot_pos = TRUE_SLOT_POSITIONS[slot];
        let room_to_slot_dist = (true_room_pos - true_slot_pos).abs() as u32;
        let room_exit_dist = (self.max_room_size - new.rooms[room].len()) as u32 + 1;
        let dist = room_exit_dist + room_to_slot_dist;
        let amphipod = new.rooms[room].pop().unwrap();
        let energy = amphipod.energy() * dist;
        new.hallway[slot] = Some(amphipod);
        (new, energy)
    }

    fn slot_to_room(&self, slot: usize) -> Option<(Self, u32)> {
        let amphipod = self.hallway[slot]?;
        let room = match amphipod {
            Amphipod::A => 0,
            Amphipod::B => 1,
            Amphipod::C => 2,
            Amphipod::D => 3
        };
        if self.rooms[room].iter().any(|&a| a != amphipod) {
            return None;
        }
        if self.rooms[room].len() >= self.max_room_size {
            return None;
        }
        let room_slot = 2 + room;
        let mut slots_in_between = if slot < room_slot {
            (slot + 1)..room_slot
        } else {
            room_slot..slot
        };
        if slots_in_between.any(|s| self.hallway[s].is_some()) {
            return None;
        }
        let mut new = self.clone();
        let true_room_pos = TRUE_ROOM_POSITIONS[room];
        let true_slot_pos = TRUE_SLOT_POSITIONS[slot];
        let slot_to_room_dist = (true_room_pos - true_slot_pos).abs() as u32;
        let room_enter_dist = (self.max_room_size - new.rooms[room].len()) as u32;
        let dist = room_enter_dist + slot_to_room_dist;
        let amphipod = new.hallway[slot].take().unwrap();
        let energy = amphipod.energy() * dist;
        new.rooms[room].push(amphipod);
        assert!(new.rooms[room].len() <= self.max_room_size);
        Some((new, energy))
    }

    fn optimal_energy(&self, cache: &mut HashMap<Self, Option<u32>>) -> Option<u32> {
        let correct_order = self.rooms.iter()
            .zip(&[Amphipod::A, Amphipod::B, Amphipod::C, Amphipod::D])
            .all(|(r, c)| r.iter().all(|a| a == c));
        let hallway_is_empty = self.hallway.iter().all(Option::is_none);
        if hallway_is_empty && correct_order {
            return Some(0);
        }
        if let Some(&energy) = cache.get(self) {
            return energy;
        }
        let mut energy = None;
        for slot in 0..self.hallway.len() {
            if let Some((child, move_energy)) = self.slot_to_room(slot) {
                let child_energy = child.optimal_energy(cache);
                if let Some(child_energy) = child_energy {
                    energy = Some(energy.unwrap_or(u32::MAX).min(move_energy + child_energy));
                }
            }
        }
        for room in 0..self.rooms.len() {
            for slot in self.room_to_hallway_moves(room) {
                let (child, move_energy) = self.room_to_hallway(room, slot);
                let child_energy = child.optimal_energy(cache);
                if let Some(child_energy) = child_energy {
                    energy = Some(energy.unwrap_or(u32::MAX).min(move_energy + child_energy));
                }
            }
        }
        cache.insert(self.clone(), energy);
        energy
    }
}

fn parse_inputs(input: impl BufRead) -> Burrow {
    let mut input = input.lines().map(Result::unwrap);
    input.next();
    input.next();
    let mut burrow = Burrow::default();
    burrow.max_room_size = 2;
    for line in input.take(2) {
        let amphipods = line
            .chars()
            .filter(|c| matches!(c, 'A' | 'B' | 'C' | 'D'))
            .enumerate();
        for (i, amphipod) in amphipods {
            burrow.rooms[i].insert(0, match amphipod {
                'A' => Amphipod::A,
                'B' => Amphipod::B,
                'C' => Amphipod::C,
                'D' => Amphipod::D,
                _ => panic!()
            });
        }
    }
    burrow
}

fn part_1(input: impl BufRead) -> u32 {
    let burrow = parse_inputs(input);
    burrow.optimal_energy( &mut HashMap::new()).unwrap()
}

fn part_2(input: impl BufRead) -> u32 {
    let mut burrow = parse_inputs(input);
    burrow.max_room_size = 4;
    burrow.rooms[0].insert(1, Amphipod::D);
    burrow.rooms[0].insert(1, Amphipod::D);
    burrow.rooms[1].insert(1, Amphipod::C);
    burrow.rooms[1].insert(1, Amphipod::B);
    burrow.rooms[2].insert(1, Amphipod::B);
    burrow.rooms[2].insert(1, Amphipod::A);
    burrow.rooms[3].insert(1, Amphipod::A);
    burrow.rooms[3].insert(1, Amphipod::C);
    burrow.optimal_energy( &mut HashMap::new()).unwrap()
}

aoc::main!();

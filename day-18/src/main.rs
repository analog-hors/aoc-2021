use std::io::BufRead;
use std::iter::Peekable;
use std::fmt::Debug;

#[derive(Clone)]
enum SfNum {
    Num(i32),
    Pair(Box<SfNum>, Box<SfNum>)
}

impl Debug for SfNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Num(n) => write!(f, "{:?}", n),
            Self::Pair(l, r) => write!(f, "[{:?},{:?}]", l, r)
        }
    }
}

impl SfNum {
    fn parse<I: Iterator<Item=char>>(chars: &mut Peekable<I>) -> SfNum {
        if chars.peek() == Some(&'[') {
            chars.next();
            let pair = SfNum::Pair(
                Box::new(Self::parse(chars)),
                Box::new(Self::parse(chars))
            );
            chars.next();
            pair
        } else {
            let num = chars
                .take_while(char::is_ascii_digit)
                .collect::<String>()
                .parse()
                .unwrap();
            SfNum::Num(num)
        }
    }

    fn add(&self, other: &Self) -> Self {
        let mut new = Self::Pair(
            Box::new(self.clone()),
            Box::new(other.clone())
        );
        new.reduce();
        new
    }

    fn reduce(&mut self) {
        while self.try_explode_pair() || self.try_split_pair() {}
    }

    fn modify_leftmost_value(&mut self, mut f: impl FnMut(&mut i32)) {
        match self {
            SfNum::Num(n) => f(n),
            SfNum::Pair(l, _) => l.modify_leftmost_value(f),
        }
    }
    
    fn modify_rightmost_value(&mut self, mut f: impl FnMut(&mut i32)) {
        match self {
            SfNum::Num(n) => f(n),
            SfNum::Pair(_, r) => r.modify_rightmost_value(f),
        }
    }

    fn try_explode_pair(&mut self) -> bool {
        self.inner_try_explode_pair(0).is_some()
    }

    fn inner_try_explode_pair(&mut self, root_dist: u32) -> Option<(i32, i32)> {
        if let Self::Pair(left, right) = self {
            if let (&Self::Num(l), &Self::Num(r)) = (&**left, &**right) {
                if root_dist >= 4 {
                    *self = Self::Num(0);
                    return Some((l, r));
                }
            }
            if let Self::Pair(_, _) = **left {
                if let Some((l, r)) = left.inner_try_explode_pair(root_dist + 1) {
                    right.modify_leftmost_value(|n| *n += r);
                    return Some((l, 0));
                }
            }
            if let Self::Pair(_, _) = **right {
                if let Some((l, r)) = right.inner_try_explode_pair(root_dist + 1) {
                    left.modify_rightmost_value(|n| *n += l);
                    return Some((0, r));
                }
            }
        }
        None
    }

    fn try_split_pair(&mut self) -> bool {
        match self {
            SfNum::Num(n) => if *n >= 10 {
                let l = *n / 2;
                let r = *n - l;
                *self = SfNum::Pair(
                    Box::new(SfNum::Num(l)),
                    Box::new(SfNum::Num(r))
                );
                true
            } else {
                false
            },
            SfNum::Pair(l, r) => {
                if l.try_split_pair() {
                    true
                } else if r.try_split_pair() {
                    true
                } else {
                    false
                }
            }
        }
    }

    fn magnitude(&self) -> i32 {
        match self {
            SfNum::Num(n) => *n,
            SfNum::Pair(l, r) => l.magnitude() * 3 + r.magnitude() * 2
        }
    }
}

fn parse_snailfish_numbers(input: impl BufRead) -> impl Iterator<Item=SfNum> {
    input
        .lines()
        .map(|n| SfNum::parse(&mut n.unwrap().chars().peekable()))
}

fn part_1(input: impl BufRead) -> i32 {
    parse_snailfish_numbers(input)
        .reduce(|a, n| a.add(&n))
        .unwrap()
        .magnitude()
}

fn part_2(input: impl BufRead) -> i32 {
    let numbers = parse_snailfish_numbers(input).collect::<Vec<_>>();
    let mut magnitude = 0;
    for (i, a) in numbers.iter().enumerate() {
        for (j, b) in numbers.iter().enumerate() {
            if i != j {
                magnitude = magnitude.max(a.add(b).magnitude());
            }
        }
    }
    magnitude
}

aoc::main!();

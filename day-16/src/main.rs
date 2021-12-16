use std::io::BufRead;

fn bit_stream(input: impl BufRead) -> impl Iterator<Item=bool> {
    let hex = input.lines().next().unwrap().unwrap();
    let bytes = hex
        .chars()
        .map(|c| char::to_digit(c, 16).unwrap())
        .collect::<Vec<_>>();
    bytes.into_iter().flat_map(|b| (0..4).rev().map(move |i| b & (1 << i) != 0))
}

#[derive(Debug)]
struct Packet {
    version: u64,
    body: PacketBody
}

#[derive(Debug)]
enum PacketBody {
    Literal(u64),
    Operator(OperatorKind, Vec<Packet>)
}

#[derive(Debug)]
enum OperatorKind {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    EqualTo
}

impl Packet {
    fn parse_number(stream: &mut impl Iterator<Item=bool>, bits: usize) -> u64 {
        stream
            .take(bits)
            .zip((0..bits).rev())
            .fold(0, |n, (b, i)| n | ((b as u64) << i))
    }

    fn parse(stream: &mut impl Iterator<Item=bool>) -> Packet {
        let version = Self::parse_number(stream, 3);
        let id = Self::parse_number(stream, 3);
        let body = match id {
            4 => {
                let mut bits = Vec::new();
                loop {
                    let is_last = !stream.next().unwrap();
                    bits.extend(stream.take(4));
                    if is_last {
                        break;
                    }
                }
                let n = Self::parse_number(&mut bits.iter().copied(), bits.len());
                PacketBody::Literal(n)
            }
            _ => {
                let operator_kind = match id {
                    0 => OperatorKind::Sum,
                    1 => OperatorKind::Product,
                    2 => OperatorKind::Min,
                    3 => OperatorKind::Max,
                    5 => OperatorKind::GreaterThan,
                    6 => OperatorKind::LessThan,
                    7 => OperatorKind::EqualTo,
                    _ => panic!()
                };
                let packet_count_mode = stream.next().unwrap();
                if packet_count_mode {
                    let len = Self::parse_number(stream, 11);
                    let packets = (0..len)
                        .map(|_| Self::parse(stream))
                        .collect();
                    PacketBody::Operator(operator_kind, packets)
                } else {
                    let len = Self::parse_number(stream, 15) as usize;
                    let mut bits = stream.take(len).collect::<Vec<_>>();
                    let mut bits = bits.drain(..);
                    let mut packets = Vec::new();
                    while bits.len() > 0 {
                        packets.push(Self::parse(&mut bits));
                    }
                    PacketBody::Operator(operator_kind, packets)
                }
            }
        };
        Packet {
            version,
            body
        }
    }

    fn version_sum(&self) -> u64 {
        let mut sum = self.version;
        if let PacketBody::Operator(_, packets) = &self.body {
            sum += packets.iter().map(Self::version_sum).sum::<u64>();
        }
        sum
    }

    fn evaluate(&self) -> u64 {
        match &self.body {
            PacketBody::Literal(n) => *n,
            PacketBody::Operator(k, p) => {
                let mut p = p.iter().map(Self::evaluate);
                match k {
                    OperatorKind::Sum         => p.sum(),
                    OperatorKind::Product     => p.product(),
                    OperatorKind::Min         => p.min().unwrap(),
                    OperatorKind::Max         => p.max().unwrap(),
                    OperatorKind::GreaterThan => (p.next().unwrap() > p.next().unwrap()) as u64,
                    OperatorKind::LessThan    => (p.next().unwrap() < p.next().unwrap()) as u64,
                    OperatorKind::EqualTo     => (p.next().unwrap() == p.next().unwrap()) as u64,
                }
            },
        }
    }
}

fn part_1(input: impl BufRead) -> u64 {
    Packet::parse(&mut bit_stream(input)).version_sum()
}

fn part_2(input: impl BufRead) -> u64 {
    Packet::parse(&mut bit_stream(input)).evaluate()
}

aoc::main!();

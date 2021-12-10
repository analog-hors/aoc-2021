use std::io::BufRead;
use std::iter::Peekable;

#[derive(Debug)]
enum ParseError {
    UnexpectedEnd,
    InvalidCharacter(usize, char),
    InvalidClosingBracket(usize, char)
}

fn closing_bracket(bracket: char) -> Option<char> {
    Some(match bracket {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => return None
    })
}

fn is_bracket(bracket: char) -> bool {
    closing_bracket(bracket).is_some()
}

fn parse_brackets<I: Iterator<Item=(usize, char)>>(input: &mut Peekable<I>) -> Result<(), ParseError> {
    let (bracket_index, bracket) = input.next()
        .ok_or(ParseError::UnexpectedEnd)?;
    let expected = closing_bracket(bracket)
        .ok_or(ParseError::InvalidCharacter(bracket_index, bracket))?;
    while matches!(input.peek(), Some(&(_, c)) if is_bracket(c)) {
        parse_brackets(input)?;
    }
    let (closing_index, closing) = input.next().ok_or(ParseError::UnexpectedEnd)?;
    if closing != expected {
        return Err(ParseError::InvalidClosingBracket(closing_index, closing));
    }
    Ok(())
}

fn part_1(input: impl BufRead) -> u32 {
    let mut score = 0;
    for line in input.lines().map(Result::unwrap) {
        let mut line = line.chars().enumerate().peekable();
        let parsed = parse_brackets(&mut line);
        if let Err(ParseError::InvalidClosingBracket(_, bracket)) = parsed {
            score += match bracket {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!()
            };
        }
    }
    score
}

fn complete_brackets<I: Iterator<Item=(usize, char)>>(input: &mut Peekable<I>, completion: &mut String) -> Result<(), ParseError> {
    let (bracket_index, bracket) = match input.next() {
        Some(c) => c,
        None => return Ok(())
    };
    let expected = closing_bracket(bracket)
        .ok_or(ParseError::InvalidCharacter(bracket_index, bracket))?;
    while matches!(input.peek(), Some(&(_, c)) if is_bracket(c)) {
        complete_brackets(input, completion)?;
    }
    if let Some((closing_index, closing)) = input.next() {
        if closing != expected {
            return Err(ParseError::InvalidClosingBracket(closing_index, closing));
        }
    } else {
        completion.push(expected);
    }
    Ok(())
}

fn part_2(input: impl BufRead) -> u64 {
    let mut scores = Vec::new();
    for line in input.lines().map(Result::unwrap) {
        let mut line = line.chars().enumerate().peekable();
        let mut completion = String::new();
        if complete_brackets(&mut line, &mut completion).is_ok() {
            let mut score = 0;
            for bracket in completion.chars() {
                score *= 5;
                score += match bracket {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!()
                };
            }
            scores.push(score);
        }
    }
    let middle = scores.len() / 2;
    *scores.select_nth_unstable(middle).1
}

aoc::main!();

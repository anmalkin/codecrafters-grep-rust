use std::env;
use std::io;
use std::process;

enum MatchResult<'a> {
    Success,
    Remaining(&'a str),
    Failure,
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if let Some(pattern) = pattern.strip_prefix('^') {
        if let MatchResult::Success = match_here(input_line, pattern) {
            return true
        }
        return false
    }
    let mut remaining = input_line;
    while !remaining.is_empty() {
        match match_here(remaining, pattern) {
            MatchResult::Success => return true,
            MatchResult::Failure => return false,
            MatchResult::Remaining(str) => {
                remaining = str;
            }
        }
    }
    false
}

fn match_here<'a>(input_line: &'a str, pattern: &str) -> MatchResult<'a> {
    let mut input_chars = input_line.chars();
    let mut pattern_chars = pattern.chars().peekable();
    match pattern_chars.next() {
        Some('\\') => match pattern_chars.next() {
            Some('d') if input_chars.next().is_some_and(|i| i.is_ascii_digit()) => {
                match_here(&input_line[1..], &pattern[2..])
            }
            Some('w') if input_chars.next().is_some_and(|i| i.is_alphanumeric()) => {
                match_here(&input_line[1..], &pattern[2..])
            }
            Some('w' | 'd') => MatchResult::Remaining(&input_line[1..]),
            _ => MatchResult::Failure,
        },
        Some('[') => {
            let mut is_negative = false;
            let mut start = 0;
            let mut end = 1;
            for char in pattern_chars {
                if char == '[' {
                    return MatchResult::Failure;
                }
                if char == '^' {
                    is_negative = true;
                    start += 1;
                    end += 1;
                    continue;
                }
                if char == ']' {
                    break;
                }
                end += 1;
            }

            if end == pattern.len() {
                return MatchResult::Failure;
            }

            let group = &pattern[start..end];

            if is_negative {
                match_neg_group(input_line, group)
            } else {
                match_pos_group(input_line, group)
            }
        }
        Some('$') => {
            if input_chars.next().is_none() {
                return MatchResult::Success
            }
            MatchResult::Failure
        }
        Some(p) if p.is_ascii_alphanumeric() && pattern_chars.peek().is_some_and(|c| *c == '+') => {
            if let MatchResult::Remaining(remaining) = match_sequence(input_line, p) {
               return match_here(remaining, &pattern[1..])
            }
            MatchResult::Remaining(&input_line[1..])
        }
        Some(p) if input_chars.next().is_some_and(|i| i == p) => {
            match_here(&input_line[1..], &pattern[1..])
        }
        Some(_) if input_chars.next().is_none() => MatchResult::Failure,
        Some(_) => MatchResult::Remaining(&input_line[1..]),
        None => MatchResult::Success,
    }
}

fn match_sequence(input_line: &str, char: char) -> MatchResult {
    let mut end = 1;
    let mut chars = input_line.chars().peekable();
    if !chars.peek().is_some_and(|c| *c == char) {
        return MatchResult::Failure
    }
    for c in chars {
        if c != char {
            break;
        }
        end += 1;
    }
    MatchResult::Remaining(&input_line[..end])
}

fn match_pos_group<'a>(input_line: &'a str, group: &str) -> MatchResult<'a> {
    if let Some(c) = input_line.chars().next() {
        for char in group.chars() {
            if c == char {
                return MatchResult::Success;
            }
        }
        return MatchResult::Remaining(&input_line[1..]);
    }
    MatchResult::Remaining(input_line)
}

fn match_neg_group<'a>(input_line: &'a str, group: &str) -> MatchResult<'a> {
    if let Some(c) = input_line.chars().next() {
        for char in group.chars() {
            if c == char {
                return MatchResult::Remaining(&input_line[1..]);
            }
        }
        return MatchResult::Success;
    }
    MatchResult::Remaining(input_line)
}


// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    println!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    }
    process::exit(1)
}

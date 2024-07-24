use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    match pattern {
        r"\d" => {
            input_line.find(|c: char| c.is_ascii_digit()).is_some()
        }
        _char if pattern.chars().count() == 1 => input_line.contains(pattern),
        _ => panic!("Unhandled pattern: {}", pattern)
    }
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


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn digit() {
        let input_line = "abc/123";
        let pattern = r"\d";
        assert!(match_pattern(input_line, pattern));
    }
}

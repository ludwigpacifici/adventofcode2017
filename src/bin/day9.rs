extern crate failure;

use failure::Error;
use std::fs::File;
use std::io::Read;
use std::string::String;

fn main() {
    if let Err(ref err) = run() {
        eprintln!("error: {:?}", err);
        eprintln!("bactrace: {:?}", err.backtrace());
        ::std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let mut file = File::open("input/day9.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    println!("a: {:?}", run_a(&input));
    println!("b: {:?}", run_b(&input));

    Ok(())
}

fn run_a(input: &str) -> u32 {
    let mut score = 0;
    let mut depth = 0;
    let mut discarding = false;

    let mut logic = |character: &char| {
        match *character {
            '<' if !discarding => discarding = true,
            '>' => discarding = false,
            '{' if !discarding => depth += 1,
            '}' if !discarding => {
                score += depth;
                depth -= 1;
            }
            _ => (),
        };
        score
    };

    streamer(input, &mut logic)
}

fn run_b(input: &str) -> u32 {
    let mut score = 0;
    let mut discarding = false;

    let mut logic = |character: &char| {
        match *character {
            '<' if !discarding => discarding = true,
            '>' => discarding = false,
            _ if discarding => score += 1,
            _ => (),
        };
        score
    };

    streamer(input, &mut logic)
}

fn streamer(input: &str, logic: &mut FnMut(&char) -> u32) -> u32 {
    let mut it = input.chars();
    let mut ret = 0;

    while let Some(character) = it.next() {
        match character {
            '!' => {
                it.next();
            }
            _ => ret = logic(&character),
        };
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        assert_eq!(1, run_a("{}"));
        assert_eq!(6, run_a("{{{}}}"));
        assert_eq!(5, run_a("{{},{}}"));
        assert_eq!(16, run_a("{{{},{},{{}}}}"));
        assert_eq!(1, run_a("{<a>,<a>,<a>,<a>}"));
        assert_eq!(9, run_a("{{<ab>},{<ab>},{<ab>},{<ab>}}"));
        assert_eq!(9, run_a("{{<!!>},{<!!>},{<!!>},{<!!>}}"));
        assert_eq!(3, run_a("{{<a!>},{<a!>},{<a!>},{<ab>}}"));
        assert_eq!(0, run_a("<>"));
        assert_eq!(0, run_a("<random characters>"));
        assert_eq!(0, run_a("<<<<>"));
        assert_eq!(0, run_a("<{!>}>"));
        assert_eq!(0, run_a("<!!>"));
        assert_eq!(0, run_a("<!!!>>"));
        assert_eq!(0, run_a("<{o\"i!a,<{i<a>"));
    }

    #[test]
    fn test_run_b() {
        assert_eq!(0, run_b("<>"));
        assert_eq!(17, run_b("<random characters>"));
        assert_eq!(3, run_b("<<<<>"));
        assert_eq!(2, run_b("<{!>}>"));
        assert_eq!(0, run_b("<!!>"));
        assert_eq!(0, run_b("<!!!>>"));
        assert_eq!(10, run_b("<{o\"i!a,<{i<a>"));
    }
}

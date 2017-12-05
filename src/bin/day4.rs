extern crate failure;

use failure::Error;
use std::collections::HashSet;
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
    let mut file = File::open("input/day4.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    println!("a: {:?}", run_a(&input));
    println!("b: {:?}", run_b(&input));

    Ok(())
}

fn run_a(input: &str) -> usize {
    let logic = |line: &str| {
        let mut words = HashSet::new();
        if line.split_whitespace().any(|word| !words.insert(word)) {
            None
        } else {
            Some(())
        }
    };

    passphrase_checker(input, logic)
}

fn run_b(input: &str) -> usize {
    let logic = |line: &str| {
        let mut words = HashSet::new();
        let is_anagram = |word: &str| {
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort();
            !words.insert(chars)
        };
        if line.split_whitespace().any(is_anagram) {
            None
        } else {
            Some(())
        }
    };

    passphrase_checker(input, logic)
}

fn passphrase_checker(input: &str, logic: fn(&str) -> Option<()>) -> usize {
    input.lines().filter_map(logic).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        assert_eq!(1, run_a("aa bb cc dd ee"));
        assert_eq!(0, run_a("aa bb cc dd aa"));
        assert_eq!(1, run_a("aa bb cc dd aaa"));
    }

    #[test]
    fn test_run_b() {
        assert_eq!(1, run_b("abcde fghij"));
        assert_eq!(0, run_b("abcde xyz ecdab"));
        assert_eq!(1, run_b("a ab abc abd abf abj"));
        assert_eq!(1, run_b("iiii oiii ooii oooi oooo"));
        assert_eq!(0, run_b("oiii ioii iioi iiio"));
    }
}

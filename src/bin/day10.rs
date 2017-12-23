extern crate adventofcode2017;
extern crate failure;

use adventofcode2017::{knot_hash, knot_hash_partial};
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
    let mut file = File::open("input/day10.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    input.pop();

    println!("a: {:?}", run_a(&input, 256));
    println!("b: {:?}", run_b(&input));

    Ok(())
}

fn run_a(input: &str, list_size: usize) -> usize {
    let input: Vec<_> = input
        .split(',')
        .filter_map(|n| n.trim().parse().ok())
        .collect();

    knot_hash_partial(&input, list_size, 1)
        .iter()
        .take(2)
        .product()
}

fn run_b(input: &str) -> String {
    knot_hash(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        assert_eq!(0, run_a("", 5));
        assert_eq!(2, run_a("3", 5));
        assert_eq!(12, run_a("3, 4", 5));
        assert_eq!(12, run_a("3, 4, 1", 5));
        assert_eq!(12, run_a("3, 4, 1, 5", 5));
    }

    #[test]
    fn test_run_b() {
        assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", run_b(""));
        assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", run_b("AoC 2017"));
        assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", run_b("1,2,3"));
        assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", run_b("1,2,4"));
    }
}

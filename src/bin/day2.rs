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
    let mut file = File::open("input/day2.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    println!("a: {:?}", run_a(&input));
    println!("b: {:?}", run_b(&input));

    Ok(())
}

fn run_a(input: &str) -> u32 {
    let logic = |vals: Vec<u32>| -> u32 {
        let min = *vals.iter().min().unwrap_or(&0u32);
        let max = *vals.iter().max().unwrap_or(&0u32);
        max - min
    };

    checksum(input, &logic)
}

fn run_b(input: &str) -> u32 {
    let logic = |vals: Vec<u32>| -> u32 {
        for x in 0..vals.len() {
            for y in 0..vals.len() {
                if x != y && vals[y] != 0 && vals[x] % vals[y] == 0 {
                    return vals[x] / vals[y];
                }
            }
        }
        // "the only two numbers in each row where one evenly divides the other"
        unreachable!();
    };

    checksum(input, &logic)
}

fn checksum(input: &str, logic: &Fn(Vec<u32>) -> u32) -> u32 {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|i| i.parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .map(logic)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        let input = "5	1	9	5
7	5	3
2	4	6	8";

        assert_eq!(18, run_a(input));
    }

    #[test]
    fn test_run_b() {
        let input = "5	9	2	8
9	4	7	3
3	8	6	5";

        assert_eq!(9, run_b(input));
    }
}

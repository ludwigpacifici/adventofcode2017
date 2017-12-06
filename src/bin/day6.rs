extern crate failure;

use failure::Error;
use std::collections::HashMap;
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
    let mut file = File::open("input/day6.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    println!("a: {:?}", run_a(&input));
    println!("b: {:?}", run_b(&input));

    Ok(())
}

fn into_vec(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect()
}

fn checksum(banks: &[usize]) -> String {
    banks.iter().map(|n| n.to_string() + ",").collect()
}

fn most_blocks(banks: &[usize]) -> (usize, usize) {
    let (index, &value) = banks
        .iter()
        .rev()
        .enumerate()
        .max_by_key(|&(_, v)| v)
        .unwrap();
    (banks.len() - index - 1, value)
}

fn redistribute(banks: &mut [usize], (index, value): (usize, usize)) {
    let range = (0..banks.len()).cycle().skip(index + 1).take(value);

    banks[index] = 0;
    range.for_each(|index| banks[index] += 1);
}

fn run_a(input: &str) -> usize {
    let logic = |_, count| count;
    looper(input, logic)
}

fn run_b(input: &str) -> usize {
    let logic = |previous_count, count| count - previous_count;
    looper(input, logic)
}

fn looper(input: &str, logic: fn(usize, usize) -> usize) -> usize {
    let mut input = into_vec(input);
    let mut checksums = HashMap::new();
    let mut count = 0;

    while !checksums.contains_key(&checksum(&input)) {
        checksums.insert(checksum(&input), count);
        count += 1;
        let bank = most_blocks(&input);
        redistribute(&mut input, bank);
    }

    logic(checksums[&checksum(&input)], count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        let input = "0	2	7	0";
        assert_eq!(5, run_a(input));
    }

    #[test]
    fn test_run_b() {
        let input = "0	2	7	0";
        assert_eq!(4, run_b(input));
    }
}

#[macro_use]
extern crate failure;

use failure::{err_msg, Error};
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
    let mut file = File::open("input/day1.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    input.pop();

    println!("a: {:?}", run_a(&input));
    println!("b: {:?}", run_b(&input));

    Ok(())
}

fn run_a(input: &str) -> Result<u32, Error> {
    let input = push_first(input)?;
    Ok(captcha_sum_next(&input))
}

fn run_b(input: &str) -> Result<u32, Error> {
    captcha_sum_middle(input)
}

fn push_first(data: &str) -> Result<String, Error> {
    ensure!(!data.is_empty(), "data is empty");

    let first = data.chars()
        .nth(0)
        .ok_or_else(|| err_msg("cannot get first element"))?;
    let mut data = data.to_owned();
    data.push(first);

    ensure!(data.len() >= 2, "could not push first char");
    Ok(data)
}

fn captcha_sum_next(data: &str) -> u32 {
    captcha_sum(data, 1)
}

fn captcha_sum_middle(data: &str) -> Result<u32, Error> {
    ensure!(
        data.len() % 2 == 0,
        "list must have an even number of elements"
    );

    let middle = data.len() / 2;
    let half_sum = captcha_sum(data, middle);
    Ok(2 * half_sum)
}

fn captcha_sum(data: &str, shift: usize) -> u32 {
    data.chars()
        .zip(data[shift..].chars())
        .filter(|pair| pair.0 == pair.1)
        .filter_map(|pair| pair.0.to_digit(10))
        .fold(0, |acc, val| acc + val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        assert_eq!(3, run_a("1122").unwrap()); // As of today, Failure does not implement PartialEq
        assert_eq!(4, run_a("1111").unwrap());
        assert_eq!(0, run_a("1234").unwrap());
        assert_eq!(9, run_a("91212129").unwrap());
    }

    #[test]
    fn test_run_b() {
        assert_eq!(6, run_b("1212").unwrap());
        assert_eq!(0, run_b("1221").unwrap());
        assert_eq!(4, run_b("123425").unwrap());
        assert_eq!(12, run_b("123123").unwrap());
        assert_eq!(4, run_b("12131415").unwrap());
    }
}

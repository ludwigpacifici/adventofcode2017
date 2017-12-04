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
    let mut file = File::open("input/day1.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    println!("a: {:?}", run_a(&input));
    println!("b: {:?}", run_b(&input));

    Ok(())
}

fn run_a(input: &str) -> u32 {
    captcha_sum_next(input)
}

fn run_b(input: &str) -> u32 {
    captcha_sum_middle(input)
}

fn captcha_sum_next(data: &str) -> u32 {
    captcha_sum(data, 1)
}

fn captcha_sum_middle(data: &str) -> u32 {
    let middle = data.len() / 2;
    captcha_sum(data, middle)
}

fn captcha_sum(data: &str, shift: usize) -> u32 {
    data.chars()
        .zip(data.chars().cycle().skip(shift))
        .filter(|&(a, b)| a == b)
        .filter_map(|(a, _)| a.to_digit(10))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        assert_eq!(3, run_a("1122"));
        assert_eq!(4, run_a("1111"));
        assert_eq!(0, run_a("1234"));
        assert_eq!(9, run_a("91212129"));
    }

    #[test]
    fn test_run_b() {
        assert_eq!(6, run_b("1212"));
        assert_eq!(0, run_b("1221"));
        assert_eq!(4, run_b("123425"));
        assert_eq!(12, run_b("123123"));
        assert_eq!(4, run_b("12131415"));
    }
}

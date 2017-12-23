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
    let mut file = File::open("input/day15.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let factor_a = 16_807;
    let factor_b = 48_271;
    let product = 2_147_483_647;
    let pairs_count = 40_000_000;

    println!(
        "a: {:?}",
        run_a(&input, factor_a, factor_b, product, pairs_count)
    );

    let multiple_a = 4;
    let multiple_b = 8;
    let pairs_count = 5_000_000;

    println!(
        "b: {:?}",
        run_b(
            &input,
            factor_a,
            factor_b,
            multiple_a,
            multiple_b,
            product,
            pairs_count
        )
    );

    Ok(())
}

fn run_a(input: &str, factor_a: u64, factor_b: u64, product: u64, pairs_count: u64) -> u64 {
    let (mut a, mut b) = parse_seeds(input);
    let mut count = 0;
    let lowest_16_bits = 0b1111_1111_1111_1111;
    (0..pairs_count).for_each(|_| {
        a = generator_next(a, factor_a, product, 1);
        b = generator_next(b, factor_b, product, 1);

        if generator_match(a, b, lowest_16_bits) {
            count += 1;
        }
    });

    count
}

fn run_b(
    input: &str,
    factor_a: u64,
    factor_b: u64,
    multiple_a: u64,
    multiple_b: u64,
    product: u64,
    pairs_count: u64,
) -> u64 {
    let (mut a, mut b) = parse_seeds(input);
    let mut count = 0;
    let lowest_16_bits = 0b1111_1111_1111_1111;
    (0..pairs_count).for_each(|_| {
        a = generator_next(a, factor_a, product, multiple_a);
        b = generator_next(b, factor_b, product, multiple_b);

        if generator_match(a, b, lowest_16_bits) {
            count += 1;
        }
    });

    count
}

fn generator_next(current: u64, factor: u64, product: u64, multiple: u64) -> u64 {
    let mut candidate = (factor * current) % product;

    while candidate % multiple != 0 {
        candidate = (factor * candidate) % product;
    }

    candidate
}

fn generator_match(a: u64, b: u64, filter: u64) -> bool {
    a & filter == b & filter
}

fn parse_seeds(input: &str) -> (u64, u64) {
    let mut it = input
        .lines()
        .filter_map(|l| l.split_whitespace().last().unwrap().parse::<u64>().ok())
        .take(2);

    (it.next().unwrap(), it.next().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        let factor_a = 16_807;
        let factor_b = 48_271;
        let product = 2_147_483_647;
        let pairs_count = 40_000_000;
        let input = "Generator A starts with 65
Generator B starts with 8921";

        assert_eq!(588, run_a(input, factor_a, factor_b, product, pairs_count));
    }

    #[test]
    fn test_run_b() {
        let factor_a = 16_807;
        let factor_b = 48_271;
        let multiple_a = 4;
        let multiple_b = 8;
        let product = 2_147_483_647;
        let pairs_count = 5_000_000;
        let input = "Generator A starts with 65
Generator B starts with 8921";

        assert_eq!(
            309,
            run_b(
                input,
                factor_a,
                factor_b,
                multiple_a,
                multiple_b,
                product,
                pairs_count
            )
        );
    }
}

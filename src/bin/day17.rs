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
    let mut file = File::open("input/day17.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let input: usize = input
        .lines()
        .filter_map(|n| n.parse().ok())
        .take(1)
        .next()
        .unwrap();

    let steps = 2017 + 1;
    println!("a: {:?}", run_a(input, steps));

    let steps = 50_000_000 + 1;
    println!("a: {:?}", run_b(input, steps));

    Ok(())
}

fn run_a(input: usize, steps: usize) -> usize {
    let mut buffer = vec![0];
    let mut position = 0;

    for i in 1..steps {
        position = (position + 1 + input) % i;
        buffer.insert(position, i);
    }

    buffer[(position + 1) % steps]
}

fn run_b(input: usize, steps: usize) -> usize {
    let mut position = 0;
    let mut position_0 = 0;
    let mut value_after_0 = 0;

    for i in 1..steps {
        position = (position + 1 + input) % i;

        if position == position_0 {
            value_after_0 = i;
        } else if position < position_0 {
            position_0 += 1;
        }
    }

    value_after_0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        let input = 3;
        let steps = 2018;

        assert_eq!(638, run_a(input, steps));
    }
}

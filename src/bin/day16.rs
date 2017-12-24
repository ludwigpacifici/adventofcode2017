#![feature(slice_rotate)]

extern crate failure;

use failure::Error;
use std::fs::File;
use std::io::Read;
use std::string::String;
use std::str;

fn main() {
    if let Err(ref err) = run() {
        eprintln!("error: {:?}", err);
        eprintln!("bactrace: {:?}", err.backtrace());
        ::std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let mut file = File::open("input/day16.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let seed: Vec<_> = (b'a'..b'q').collect();
    println!("a: {:?}", str::from_utf8(&run_a(&input, &seed)));

    println!(
        "b: {:?}",
        str::from_utf8(&run_b(&input, &seed, 1_000_000_000))
    );

    Ok(())
}

fn run_a(input: &str, seed: &Vec<u8>) -> Vec<u8> {
    dance(input, seed)
}

fn run_b(input: &str, seed: &Vec<u8>, iterations: usize) -> Vec<u8> {
    let mut programs = seed.clone();

    let mut cycle_count = 1;
    programs = run_a(input, &mut programs);

    while *seed != programs && cycle_count < iterations {
        programs = run_a(input, &mut programs);
        cycle_count += 1;
    }

    if *seed != programs {
        return programs;
    }

    for _ in 0..(iterations % cycle_count) {
        programs = run_a(input, &mut programs);
    }

    programs
}

fn dance(input: &str, seed: &Vec<u8>) -> Vec<u8> {
    let mut programs = seed.clone();

    for op in input.split(',') {
        match op.as_bytes()[0] {
            b's' => {
                let x = programs.len() - op[1..].parse::<usize>().unwrap();
                programs.rotate(x);
            }
            b'x' => {
                let ab: Vec<usize> = op[1..].split('/').filter_map(|n| n.parse().ok()).collect();
                programs.swap(ab[0], ab[1]);
            }
            b'p' => {
                let a = programs
                    .iter()
                    .position(|&v| v == op.as_bytes()[1])
                    .unwrap();

                let b = programs
                    .iter()
                    .position(|&v| v == op.as_bytes()[3])
                    .unwrap();

                programs.swap(a, b);
            }
            _ => {}
        };
    }

    programs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        let seed: Vec<_> = (b'a'..b'f').collect();
        let input = "s1,x3/4,pe/b";

        assert_eq!(Ok("baedc"), str::from_utf8(&run_a(input, &seed)));
    }

    #[test]
    fn test_run_b() {
        let seed: Vec<_> = (b'a'..b'f').collect();
        let input = "s1,x3/4,pe/b";

        assert_eq!(Ok("ceadb"), str::from_utf8(&run_b(input, &seed, 2)));
    }
}

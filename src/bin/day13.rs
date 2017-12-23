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
    let mut file = File::open("input/day13.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    println!("a: {:?}", run_a(&input));
    println!("b: {:?}", run_b(&input));

    Ok(())
}

fn run_a(input: &str) -> u32 {
    severity(&parse_scanners(input), 0)
}

fn run_b(input: &str) -> u32 {
    let input = parse_scanners(input);
    let mut delay = 0;

    while severity(&input, delay) != 0 {
        delay += 1;
    }

    delay
}

fn severity(input: &[(u32, u32)], delay: u32) -> u32 {
    input
        .iter()
        .map(|&(time, range)| (time + delay, range))
        .filter(|&(time, range)| {
            if range == 1 {
                return true;
            }

            let range = 2 * range - 2;
            time % range == 0
        })
        .fold(0, |acc, (time, range)| acc + time * range)
}

fn parse_scanners(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .filter_map(|l| {
            let mut it = l.split(':')
                .map(|n| n.trim())
                .filter_map(|n| n.parse::<u32>().ok());

            if let Some(time) = it.next() {
                if let Some(range) = it.next() {
                    return Some((time, range));
                }
            }
            None
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        let input = "0: 3
1: 2
4: 4
6: 4
";
        assert_eq!(24, run_a(input));
    }

    #[test]
    fn test_run_b() {
        let input = "0: 3
    1: 2
    4: 4
    6: 4
    ";
        assert_eq!(10, run_b(input));
    }
}

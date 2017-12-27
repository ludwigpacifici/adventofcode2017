#![feature(nll)]
#![feature(match_default_bindings)]

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
    let mut file = File::open("input/day24.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    input.pop();

    println!("a: {:?}", run_a(&input));
    println!("b: {:?}", run_b(&input));

    Ok(())
}

type ScoreLength = (usize, usize);

fn run_a(input: &str) -> usize {
    let mut bridges = make_bridges(input);

    let logic = |a: ScoreLength, b: ScoreLength| -> ScoreLength {
        if a.0 > b.0 {
            a
        } else {
            b
        }
    };

    chain(0, 0, &mut bridges, logic).0
}

fn run_b(input: &str) -> usize {
    let mut bridges = make_bridges(input);

    let logic = |a: ScoreLength, b: ScoreLength| -> ScoreLength {
        if a.1 > b.1 {
            a
        } else if a.1 < b.1 {
            b
        } else if a.0 > b.0 {
            a
        } else {
            b
        }
    };

    chain(0, 0, &mut bridges, logic).0
}

fn chain(
    seed: usize,
    length: usize,
    bridges: &mut Vec<(usize, usize, bool)>,
    logic: fn(ScoreLength, ScoreLength) -> ScoreLength,
) -> ScoreLength {
    let candidates: Vec<_> = bridges
        .iter()
        .enumerate()
        .filter_map(|(i, &(a, b, free))| {
            if free && seed == a {
                Some((i, b))
            } else if free && seed == b {
                Some((i, a))
            } else {
                None
            }
        })
        .collect();

    candidates.iter().fold((0, length), |max, &(i, next_seed)| {
        if let Some((a, b, free)) = bridges.get_mut(i) {
            *free = false;
            let score = chain(next_seed, length + 1, bridges, logic);
            *free = true;
            return logic((*a + *b + score.0, score.1), max);
        }
        (0, length)
    })
}

fn make_bridges(input: &str) -> Vec<(usize, usize, bool)> {
    input
        .lines()
        .map(|l| {
            let mut it = l.split('/').filter_map(|n| n.parse::<usize>().ok());
            (it.next().unwrap(), it.next().unwrap(), true)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        let input = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";

        assert_eq!(31, run_a(&input));
    }

    #[test]
    fn test_run_b() {
        let input = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";

        assert_eq!(19, run_b(&input));
    }
}

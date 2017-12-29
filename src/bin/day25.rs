#![feature(match_default_bindings)]
#![feature(test)]

extern crate failure;
extern crate test;

use failure::Error;
use std::collections::VecDeque;
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
    let mut file = File::open("input/day25.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    println!("a: {:?}", run_a(&input));

    Ok(())
}

fn run_a(input: &str) -> usize {
    let input: String = input.chars().filter(|&c| c != '.' && c != ':').collect();
    let turing = make_turing_machine(&input);

    let mut tape = VecDeque::new();
    tape.push_back(0);

    let mut pointer = 0;
    let (mut state, iterations) = make_initial_state(&input);

    for _ in 0..iterations {
        let current_value = tape[pointer];
        let (value_0, value_1) = turing[(state - b'A') as usize];

        let (write_value, is_next_left, next_state) =
            if current_value == 0 { value_0 } else { value_1 };

        tape[pointer] = write_value;

        if is_next_left {
            if pointer == 0 {
                tape.push_front(0);
            } else {
                pointer -= 1;
            }
        } else {
            if pointer == tape.len() - 1 {
                tape.push_back(0);
            }
            pointer += 1;
        }

        state = next_state;
    }

    tape.into_iter().sum()
}

fn make_initial_state(input: &str) -> (u8, usize) {
    let mut lines = input.lines();
    let seed_state = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .as_bytes()[0];

    let checksum_iteration: usize = lines
        .next()
        .unwrap()
        .split_whitespace()
        .nth(5)
        .unwrap()
        .parse()
        .unwrap();

    (seed_state, checksum_iteration)
}

type Transistion = (usize, bool, u8);

fn make_turing_machine(input: &str) -> Vec<(Transistion, Transistion)> {
    let mut turing = Vec::new();

    let mut input = input.lines().skip(2);

    while let Some(_) = input.next() {
        input.next();

        let mut state = ((0, false, 0), (0, false, 0));
        for i in 0..2 {
            input.next();

            let current_value = usize::from(
                input
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .nth(4)
                    .unwrap()
                    .as_bytes()[0] - b'0',
            );

            let is_next_left = "left" == input.next().unwrap().split_whitespace().nth(6).unwrap();

            let next_state = input
                .next()
                .unwrap()
                .split_whitespace()
                .nth(4)
                .unwrap()
                .as_bytes()[0];

            if i == 0 {
                state.0 = (current_value, is_next_left, next_state);
            } else {
                state.1 = (current_value, is_next_left, next_state);
            }
        }

        turing.push(state);
    }

    turing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        let input = "Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.";

        assert_eq!(3, run_a(&input));
    }

    #[bench]
    fn test_run(b: &mut test::Bencher) {
        b.iter(|| {
            let mut file = File::open("input/day25.txt").unwrap();
            let mut input = String::new();
            file.read_to_string(&mut input).unwrap();
            assert_eq!(2725, run_a(&input));
        });
    }
}

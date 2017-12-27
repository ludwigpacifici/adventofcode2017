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
    let mut file = File::open("input/day22.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    input.pop();

    let bursts = 10_000;
    println!("a: {:?}", run_a(&input, bursts));

    let bursts = 10_000_000;
    println!("b: {:?}", run_b(&input, bursts));

    Ok(())
}

fn run_a(input: &str, bursts: usize) -> usize {
    let (mut nodes, start) = make_nodes(input);
    let mut carrier = VirusCarrier::new(start);

    for _ in 0..bursts {
        carrier.next_a(&mut nodes);
    }

    carrier.infection_count
}

fn run_b(input: &str, bursts: usize) -> usize {
    let (mut nodes, start) = make_nodes(input);
    let mut carrier = VirusCarrier::new(start);

    for _ in 0..bursts {
        carrier.next_b(&mut nodes);
    }

    carrier.infection_count
}

fn make_nodes(input: &str) -> (HashMap<(i64, i64), Flag>, (i64, i64)) {
    let middle = (
        (input.lines().count() / 2) as i64,
        (input.lines().take(1).next().unwrap().chars().count() / 2) as i64,
    );

    let mut nodes = HashMap::new();
    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            match c {
                '#' => {
                    nodes.insert((i as i64, j as i64), Flag::Infected);
                }
                '.' => {
                    nodes.insert((i as i64, j as i64), Flag::Clean);
                }
                'W' => {
                    nodes.insert((i as i64, j as i64), Flag::Weakened);
                }
                'F' => {
                    nodes.insert((i as i64, j as i64), Flag::Flagged);
                }
                _ => {
                    eprintln!("Unknown node state: {}", c);
                }
            };
        }
    }

    (nodes, middle)
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Flag {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

fn next_flag(flag: &Flag) -> Flag {
    match *flag {
        Flag::Clean => Flag::Weakened,
        Flag::Weakened => Flag::Infected,
        Flag::Infected => Flag::Flagged,
        Flag::Flagged => Flag::Clean,
    }
}

fn turn_its_left(direction: &Direction) -> Direction {
    match *direction {
        Direction::Up => Direction::Left,
        Direction::Left => Direction::Down,
        Direction::Down => Direction::Right,
        Direction::Right => Direction::Up,
    }
}

fn turn_its_right(direction: &Direction) -> Direction {
    match *direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn reverse_direction(direction: &Direction) -> Direction {
    match *direction {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
    }
}

#[derive(Debug)]
struct VirusCarrier {
    facing: Direction,
    position: (i64, i64),
    infection_count: usize,
}

impl VirusCarrier {
    fn new(position: (i64, i64)) -> VirusCarrier {
        VirusCarrier {
            facing: Direction::Up,
            position,
            infection_count: 0,
        }
    }

    fn next_a(&mut self, nodes: &mut HashMap<(i64, i64), Flag>) {
        let flag = nodes.entry(self.position).or_insert(Flag::Clean);

        match *flag {
            Flag::Clean => {
                self.facing = turn_its_left(&self.facing);
                self.infection_count += 1;
                *flag = Flag::Infected;
            }
            Flag::Infected => {
                self.facing = turn_its_right(&self.facing);
                *flag = Flag::Clean;
            }
            _ => {}
        };

        match self.facing {
            Direction::Up => self.position.0 -= 1,
            Direction::Down => self.position.0 += 1,
            Direction::Left => self.position.1 -= 1,
            Direction::Right => self.position.1 += 1,
        };
    }

    fn next_b(&mut self, nodes: &mut HashMap<(i64, i64), Flag>) {
        let flag = nodes.entry(self.position).or_insert(Flag::Clean);

        match *flag {
            Flag::Clean => {
                self.facing = turn_its_left(&self.facing);
            }
            Flag::Weakened => {
                self.infection_count += 1;
            }
            Flag::Infected => {
                self.facing = turn_its_right(&self.facing);
            }
            Flag::Flagged => {
                self.facing = reverse_direction(&self.facing);
            }
        };

        *flag = next_flag(flag);

        match self.facing {
            Direction::Up => self.position.0 -= 1,
            Direction::Down => self.position.0 += 1,
            Direction::Left => self.position.1 -= 1,
            Direction::Right => self.position.1 += 1,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        let input = "..#
#..
...";

        let bursts = 7;
        assert_eq!(5, run_a(input, bursts));

        let bursts = 70;
        assert_eq!(41, run_a(input, bursts));

        let bursts = 10_000;
        assert_eq!(5587, run_a(input, bursts));
    }

    #[test]
    fn test_run_b() {
        let input = "..#
#..
...";

        let bursts = 100;
        assert_eq!(26, run_b(input, bursts));

        let bursts = 10_000_000;
        assert_eq!(2511944, run_b(input, bursts));
    }
}

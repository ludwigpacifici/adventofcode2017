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
    let mut file = File::open("input/day19.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    input.pop();

    println!("a: {:?}", run_a(&input));
    println!("b: {:?}", run_b(&input));

    Ok(())
}

fn run_a(input: &str) -> String {
    walk(input).0
}

fn run_b(input: &str) -> usize {
    walk(input).1
}

fn walk(input: &str) -> (String, usize) {
    let (map, mut position) = make_map(input);

    let mut direction = Direction::Down;
    let mut count = 0;
    let mut word = String::new();

    while let Some(&path) = map.get(&position) {
        match path {
            path if path.is_ascii_alphabetic() => {
                word.push(path);
                position = next_step(position, &direction);
            }
            '|' | '-' => {
                position = next_step(position, &direction);
            }
            '+' => {
                let choices = choose_turn(position, &direction);
                if map.contains_key(&(choices.0).0) {
                    position = (choices.0).0;
                    direction = (choices.0).1;
                } else {
                    position = (choices.1).0;
                    direction = (choices.1).1;
                }
            }
            _ => eprintln!("Unknown path: {:?}, at: {:?}", path, position),
        }
        count += 1;
    }

    (word, count)
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Position = (usize, usize);

fn next_step(position: Position, direction: &Direction) -> Position {
    match *direction {
        Direction::Up => (position.0 - 1, position.1),
        Direction::Down => (position.0 + 1, position.1),
        Direction::Left => (position.0, position.1 - 1),
        Direction::Right => (position.0, position.1 + 1),
    }
}

fn choose_turn(
    position: Position,
    direction: &Direction,
) -> ((Position, Direction), (Position, Direction)) {
    let choices = turn(direction);

    (
        (next_step(position, &choices.0), choices.0),
        (next_step(position, &choices.1), choices.1),
    )
}

fn turn(direction: &Direction) -> (Direction, Direction) {
    match *direction {
        Direction::Up | Direction::Down => (Direction::Left, Direction::Right),
        Direction::Left | Direction::Right => (Direction::Up, Direction::Down),
    }
}

fn make_map(input: &str) -> (HashMap<Position, char>, Position) {
    let mut map = HashMap::new();

    for (i, l) in input.lines().enumerate() {
        for (j, b) in l.chars().enumerate() {
            match b {
                path if path.is_ascii_alphabetic() => {
                    map.insert((i, j), b);
                }
                '-' | '|' | '+' => {
                    map.insert((i, j), b);
                }
                _ => {}
            };
        }
    }

    (map, (0, input.chars().position(|b| b == '|').unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        let input = "     |
     |  +--+
     A  |  C
 F---|----E|--+
     |  |  |  D
     +B-+  +--+
";

        assert_eq!("ABCDEF", run_a(input));
    }

    #[test]
    fn test_run_b() {
        let input = "     |
     |  +--+
     A  |  C
 F---|----E|--+
     |  |  |  D
     +B-+  +--+
";

        assert_eq!(38, run_b(input));
    }
}

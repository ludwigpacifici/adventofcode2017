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
    let mut file = File::open("input/day3.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    input.pop();
    let input = input.parse::<u32>()?;

    println!("a: {:?}", run_a(input));
    println!("b: {:?}", run_b(input));

    Ok(())
}

macro_rules! distance {
    ($x:expr, $y:expr) => (
        if $x < $y {
            $y - $x
        } else {
            $x - $y
        })
}

fn run_a(input: u32) -> u32 {
    if input < 2 {
        return 0;
    }

    let mut iteration = 0;
    let mut previous_end_square = 1;

    while input > previous_end_square {
        let length = 2 * iteration + 1;
        iteration += 1;
        previous_end_square += 4 * (length + 1);
    }

    let length = 2 * iteration - 1;
    previous_end_square -= 4 * (length + 1);

    let corner0 = previous_end_square + length + 1;
    if input <= corner0 {
        return iteration + distance!(input, corner0 - iteration);
    }

    let corner1 = corner0 + length + 1;
    if input <= corner1 {
        return iteration + distance!(input, corner1 - iteration);
    }

    let corner2 = corner1 + length + 1;
    if input <= corner2 {
        return iteration + distance!(input, corner2 - iteration);
    }

    let corner3 = corner2 + length + 1;
    iteration + distance!(input, corner3 - iteration)
}

#[derive(Debug, Clone)]
enum Direction {
    Xmax,
    Ymax,
    Xmin,
    Ymin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Spiral {
    coordinate: Coordinate,
    iteration: i32,
    direction: Direction,
}

impl Spiral {
    fn new() -> Spiral {
        Spiral {
            coordinate: Coordinate { x: 0, y: 0 },
            iteration: 0,
            direction: Direction::Xmax,
        }
    }

    fn next(&self) -> Spiral {
        let mut next = self.clone();

        match self.direction {
            Direction::Xmax => {
                next.coordinate.x += 1;
                if self.coordinate.x == self.iteration {
                    next.iteration += 1;
                    next.direction = Direction::Ymax;
                }
            }

            Direction::Ymax => if self.coordinate.y == self.iteration {
                next.direction = Direction::Xmin;
                next.coordinate.x -= 1;
            } else {
                next.coordinate.y += 1;
            },

            Direction::Xmin => if self.coordinate.x == -self.iteration {
                next.direction = Direction::Ymin;
                next.coordinate.y -= 1;
            } else {
                next.coordinate.x -= 1;
            },

            Direction::Ymin => if self.coordinate.y == -self.iteration {
                next.direction = Direction::Xmax;
                next.coordinate.x += 1;
            } else {
                next.coordinate.y -= 1;
            },
        }
        next
    }

    fn neighbors(&self) -> Vec<Coordinate> {
        let x = self.coordinate.x;
        let y = self.coordinate.y;
        vec![
            Coordinate { x: x - 1, y: y - 1 },
            Coordinate { x: x - 1, y: y },
            Coordinate { x: x - 1, y: y + 1 },
            Coordinate { x: x, y: y - 1 },
            Coordinate { x: x, y: y + 1 },
            Coordinate { x: x + 1, y: y - 1 },
            Coordinate { x: x + 1, y: y },
            Coordinate { x: x + 1, y: y + 1 },
        ]
    }
}

fn run_b(input: u32) -> u32 {
    let mut dir = Spiral::new();
    let mut spiral_path = HashMap::new();
    spiral_path.insert(dir.coordinate, 1u32);
    let mut last_value = 0u32;

    while last_value <= input {
        dir = dir.next();
        last_value = dir.neighbors()
            .iter()
            .filter_map(|coord| spiral_path.get(coord))
            .sum();

        spiral_path.insert(dir.coordinate, last_value);
    }

    last_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        assert_eq!(0, run_a(1));
        assert_eq!(3, run_a(12));
        assert_eq!(2, run_a(23));
        assert_eq!(31, run_a(1024));
    }

    #[test]
    fn test_run_b() {
        assert_eq!(2, run_b(1));
        assert_eq!(4, run_b(2));
        assert_eq!(5, run_b(4));
        assert_eq!(10, run_b(5));
    }
}

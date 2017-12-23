#![feature(exclusive_range_pattern)]

extern crate adventofcode2017;
extern crate failure;

use adventofcode2017::knot_hash;
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
    let mut file = File::open("input/day14.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    input.pop();

    println!("a: {:?}", run_a(&input, 128));
    println!("b: {:?}", run_b(&input, 128));

    Ok(())
}

fn run_a(input: &str, disk_size: usize) -> u32 {
    let mut used_count = 0;

    for i in 0..disk_size {
        let input = format!("{}-{}", input, i);
        used_count += knot_hash(&input, 256)
            .to_ascii_lowercase()
            .chars()
            .map(|c| match c {
                '1' | '2' | '4' | '8' => 1,
                '3' | '5' | '6' | '9' | 'a' | 'c' => 2,
                '7' | 'b' | 'd' | 'e' => 3,
                'f' => 4,
                _ => 0,
            })
            .sum::<u32>();
    }

    used_count
}

fn run_b(input: &str, disk_size: usize) -> u32 {
    let mut disk = make_disk(input, disk_size);
    let mut stack = Vec::new();
    let mut region_count = 0;

    for i in 0..disk_size {
        for j in 0..disk_size {
            stack.push((i, j));
            let mut did_something = false;

            while let Some(region) = stack.pop() {
                if disk[region.0][region.1] == '1' {
                    disk[region.0][region.1] = '0';
                    did_something = true;

                    if let Some(neighbour) = up(region, 0) {
                        stack.push(neighbour);
                    }
                    if let Some(neighbour) = down(region, disk_size) {
                        stack.push(neighbour);
                    }
                    if let Some(neighbour) = left(region, 0) {
                        stack.push(neighbour);
                    }
                    if let Some(neighbour) = rigth(region, disk_size) {
                        stack.push(neighbour);
                    }
                }
            }

            if did_something {
                region_count += 1;
            }
        }
    }

    region_count
}

fn up(coordinates: (usize, usize), min: usize) -> Option<(usize, usize)> {
    if coordinates.0 >= (min + 1) {
        Some((coordinates.0 - 1, coordinates.1))
    } else {
        None
    }
}

fn down(coordinates: (usize, usize), max: usize) -> Option<(usize, usize)> {
    if coordinates.0 < (max - 1) {
        Some((coordinates.0 + 1, coordinates.1))
    } else {
        None
    }
}

fn left(coordinates: (usize, usize), min: usize) -> Option<(usize, usize)> {
    if coordinates.1 >= (min + 1) {
        Some((coordinates.0, coordinates.1 - 1))
    } else {
        None
    }
}

fn rigth(coordinates: (usize, usize), max: usize) -> Option<(usize, usize)> {
    if coordinates.1 < (max - 1) {
        Some((coordinates.0, coordinates.1 + 1))
    } else {
        None
    }
}

fn make_disk(input: &str, disk_size: usize) -> Vec<Vec<char>> {
    let mut disk = Vec::new();

    for i in 0..disk_size {
        let input = format!("{}-{}", input, i);
        let binary: String = knot_hash(&input, 256)
            .to_ascii_lowercase()
            .chars()
            .filter_map(|c| {
                if c.is_ascii_hexdigit() {
                    Some(format!("{:04b}", c.to_digit(16).unwrap()))
                } else {
                    None
                }
            })
            .collect();

        disk.push(binary.chars().collect());
    }

    disk
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        assert_eq!(8108, run_a("flqrgnkx", 128));
    }

    #[test]
    fn test_run_b() {
        assert_eq!(1242, run_b("flqrgnkx", 128));
    }
}

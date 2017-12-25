extern crate failure;

use failure::Error;
use std::cmp::Ordering;
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
    let mut file = File::open("input/day20.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    input.pop();

    println!("a: {:?}", run_a(&input));
    println!("b: {:?}", run_b(&input));

    Ok(())
}

fn run_a(input: &str) -> usize {
    input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let particule: Vec<_> = l.replace("p=<", "")
                .replace(">, v=<", ",")
                .replace(">, a=<", ",")
                .replace(">", "")
                .split(',')
                .filter_map(|n| n.trim().parse::<i64>().ok())
                .collect::<Vec<_>>();

            (
                i,
                manhattan_distance(&particule[6..9]),
                manhattan_distance(&particule[0..3])
                    .cmp(&manhattan_distance(&next(&particule)[0..3])),
            )
        })
        .min_by(|a, b| {
            let compare_acceleration = (a.1).cmp(&b.1);
            if compare_acceleration == Ordering::Equal {
                (a.2).cmp(&b.2)
            } else {
                compare_acceleration
            }
        })
        .unwrap()
        .0
}

fn run_b(input: &str) -> usize {
    let mut particules: HashMap<_, _> = input
        .lines()
        .map(|l| {
            l.replace("p=<", "")
                .replace(">, v=<", ",")
                .replace(">, a=<", ",")
                .replace(">", "")
                .split(',')
                .filter_map(|n| n.trim().parse::<i64>().ok())
                .collect::<Vec<_>>()
        })
        .enumerate()
        .collect();

    for _ in 0..39 {
        let mut positions: HashMap<Vec<i64>, usize> = HashMap::new();

        for v in particules.values_mut() {
            next(v).iter().enumerate().for_each(|(i, c)| v[i] = *c);
            *positions.entry(v[0..3].to_vec()).or_insert(0) += 1;
        }

        particules.retain(|_, v| positions[&v[0..3]] == 1);
    }

    particules.len()
}

fn manhattan_distance(coordinates: &[i64]) -> u64 {
    coordinates.iter().map(|n| n.abs() as u64).sum()
}

fn zip_sum(a: &[i64], b: &[i64]) -> Vec<i64> {
    a.iter().zip(b.iter()).map(|(p, v)| p + v).collect()
}

fn next(particule: &[i64]) -> Vec<i64> {
    let position = &particule[0..3];
    let velocity = &particule[3..6];
    let acceleration = &particule[6..9];

    let mut velocity = zip_sum(velocity, acceleration);
    let mut position = zip_sum(position, &velocity);

    position.append(&mut velocity);
    position.append(&mut acceleration.to_vec());

    position
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        let input = "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>";

        assert_eq!(0, run_a(input));
    }

    #[test]
    fn test_run_b() {
        let input = "p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>
p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>
p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>
p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>";

        assert_eq!(1, run_b(input));
    }
}

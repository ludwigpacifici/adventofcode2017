#![feature(slice_patterns)]

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
    let mut file = File::open("input/day11.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    input.pop();

    println!("a: {:?}", run_a(&input));
    println!("b: {:?}", run_b(&input));

    Ok(())
}

fn run_a(input: &str) -> Option<usize> {
    walk_hex_grid(input).iter().last().cloned()
}

fn run_b(input: &str) -> Option<usize> {
    walk_hex_grid(input).iter().max().cloned()
}

fn walk_hex_grid(input: &str) -> Vec<usize> {
    let mut aggregated_path = HashMap::new();

    input
        .split(',')
        .map(|direction| {
            *aggregated_path.entry(direction).or_insert(0) += 1;

            simplify_edges(&["n", "s"], &mut aggregated_path);
            simplify_edges(&["ne", "sw"], &mut aggregated_path);
            simplify_edges(&["nw", "se"], &mut aggregated_path);
            simplify_edges(&["n", "sw", "se"], &mut aggregated_path);
            simplify_edges(&["s", "nw", "ne"], &mut aggregated_path);

            distance(&aggregated_path)
        })
        .collect()
}

fn simplify_edges<'a>(edges: &[&'a str], aggregated_path: &mut HashMap<&'a str, usize>) {
    if let Some(min) = edges
        .iter()
        .map(|key| *aggregated_path.entry(key).or_insert(0))
        .min()
    {
        edges
            .iter()
            .for_each(|key| *aggregated_path.entry(key).or_insert(0) -= min);
    }
}

fn distance(simplified_path: &HashMap<&str, usize>) -> usize {
    let all_directions: Vec<_> = ["n", "ne", "se", "s", "sw", "nw"]
        .iter()
        .map(|key| simplified_path[key])
        .collect();

    match *all_directions.as_slice() {
        [a, b, c, 0, 0, 0]
        | [0, a, b, c, 0, 0]
        | [0, 0, a, b, c, 0]
        | [0, 0, 0, a, b, c]
        | [c, 0, 0, 0, a, b]
        | [b, c, 0, 0, 0, a] => a.max(c) + b,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        assert_eq!(Some(3), run_a("ne,ne,ne"));
        assert_eq!(Some(0), run_a("ne,ne,sw,sw"));
        assert_eq!(Some(2), run_a("ne,ne,s,s"));
        assert_eq!(Some(3), run_a("se,sw,se,sw,sw"));
    }
}

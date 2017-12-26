#![feature(iterator_step_by)]

extern crate bytecount;
extern crate failure;
extern crate itertools;

use failure::Error;
use itertools::Itertools;
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
    let mut file = File::open("input/day21.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    input.pop();

    let seed = ".#.
..#
###"
        .replace('\n', "");

    let iterations = 5;
    println!("a: {:?}", fractal_art(&input, &seed, iterations));

    let iterations = 18;
    println!("b: {:?}", fractal_art(&input, &seed, iterations));

    Ok(())
}

fn fractal_art(input: &str, seed: &str, iterations: usize) -> usize {
    let mut enhancement_rules: HashMap<_, _> = HashMap::new();

    input.lines().for_each(|l| {
        let mut it = l.split(" => ").map(|l| l.replace('/', "").into_bytes());
        let k = it.next().unwrap();
        let v = it.next().unwrap();

        for k in rotate_flip(k) {
            enhancement_rules.insert(k, v.clone());
        }
    });

    let mut frame = String::from(seed).into_bytes();
    let mut frame_size = 3;
    let mut chunk = chunk_size(frame_size);

    for _ in 0..iterations {
        let next_frame_size = next_frame_size(frame_size, chunk);
        let mut hidden_frame = vec![0; next_frame_size * next_frame_size];
        println!("{}", next_frame_size);
        for block in generate_chunk_positions(frame_size, chunk) {
            let view: Vec<_> = (0..chunk)
                .cartesian_product(0..chunk)
                .map(|c| block + frame_size * c.0 + c.1)
                .map(|i| frame[i])
                .collect();

            let new_view = &enhancement_rules[&view];

            let shift = hidden_frame.iter().position(|&v| v == 0).unwrap();
            for i in 0..new_view.len() {
                hidden_frame[shift + (i % (chunk + 1)) + i / (chunk + 1) * next_frame_size] =
                    new_view[i];
            }
        }

        frame = hidden_frame;
        chunk = chunk_size(next_frame_size);
        frame_size = next_frame_size;
    }

    bytecount::count(&frame, b'#')
}

fn chunk_size(size: usize) -> usize {
    if size % 2 == 0 {
        2
    } else {
        3
    }
}

fn next_frame_size(frame_size: usize, chunk: usize) -> usize {
    frame_size / chunk * (chunk + 1)
}

fn generate_chunk_positions(size: usize, chunk: usize) -> Vec<usize> {
    (0..size)
        .step_by(chunk)
        .cartesian_product((0..size).step_by(chunk))
        .map(|start| start.0 * size + start.1)
        .collect()
}

fn rotate_flip(k: Vec<u8>) -> Vec<Vec<u8>> {
    let mut keys = vec![k];

    for _ in 0..3 {
        let new_key = rotate(keys.last().unwrap());
        keys.push(new_key);
    }

    let new_key = flip(keys.first().unwrap());
    keys.push(new_key);

    for _ in 0..3 {
        let new_key = rotate(keys.last().unwrap());
        keys.push(new_key);
    }

    keys
}

fn rotate(k: &[u8]) -> Vec<u8> {
    match k.len() {
        4 => vec![k[2], k[0], k[3], k[1]],
        9 => vec![k[6], k[3], k[0], k[7], k[4], k[1], k[8], k[5], k[2]],
        _ => vec![],
    }
}

fn flip(k: &[u8]) -> Vec<u8> {
    match k.len() {
        4 => vec![k[2], k[3], k[0], k[1]],
        9 => vec![k[6], k[7], k[8], k[3], k[4], k[5], k[0], k[1], k[2]],
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fractal_art() {
        let input = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";

        let seed = ".#.
..#
###"
            .replace('\n', "");

        let iterations = 2;
        assert_eq!(12, fractal_art(input, &seed, iterations));
    }
}

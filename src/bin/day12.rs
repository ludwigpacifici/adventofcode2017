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
    let mut file = File::open("input/day12.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let input = make_village(&input);
    println!("a: {:?}", run_a(&input));
    println!("b: {:?}", run_b(&input));

    Ok(())
}

fn run_a(input: &[Vec<usize>]) -> usize {
    let mut visited = vec![false; input.len()];
    let mut stack = vec![0];

    while let Some(program) = stack.pop() {
        visited[program] = true;
        input[program].iter().for_each(|&n| {
            if !visited[n] {
                stack.push(n)
            }
        });
    }

    visited.iter().filter(|&n| *n).count()
}

fn run_b(input: &[Vec<usize>]) -> usize {
    let mut visited = vec![false; input.len()];
    let mut stack = Vec::new();
    let mut count = 0;

    while let Some(orphean) = visited.iter().enumerate().position(|(_, &n)| !n) {
        stack.push(orphean);
        count += 1;

        while let Some(program) = stack.pop() {
            visited[program] = true;
            input[program].iter().for_each(|&n| {
                if !visited[n] {
                    stack.push(n)
                }
            });
        }
    }

    count
}

fn make_village(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.replace("<->", ",")
                .split(',')
                .skip(1)
                .map(|n| n.trim())
                .filter_map(|n| n.parse().ok())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5
";
        let input = make_village(&input);
        assert_eq!(6, run_a(&input));
    }

    #[test]
    fn test_run_b() {
        let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5
";

        let input = make_village(&input);
        assert_eq!(2, run_b(&input));
    }
}

#[macro_use]
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
    let mut file = File::open("input/day8.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let (heap, history) = interpreter(&input);

    println!("a: {:?}", run_a(&heap));
    println!("b: {:?}", run_b(&history));

    Ok(())
}

fn run_a(heap: &HashMap<&str, i32>) -> Option<i32> {
    heap.values().max().cloned()
}

fn run_b(history: &[i32]) -> Option<i32> {
    history.iter().max().cloned()
}

fn interpreter(input: &str) -> (HashMap<&str, i32>, Vec<i32>) {
    let mut heap = HashMap::new();

    let history = input
        .lines()
        .map(|line| {
            let mut tokens = line.split_whitespace();

            let register = tokens.nth(0).unwrap();
            let op = tokens.nth(0).unwrap();
            let offset = tokens.nth(0).unwrap().parse().unwrap();

            let lhs_if = *heap.get(tokens.nth(1).unwrap()).unwrap_or(&0);
            let op_if = tokens.nth(0).unwrap();
            let rhs_if = tokens.nth(0).unwrap().parse().unwrap();

            let register = heap.entry(register).or_insert(0);
            if comparaison(op_if, lhs_if, rhs_if).unwrap() {
                *register = update(op, *register, offset).unwrap();
            }

            *register
        })
        .collect();

    (heap, history)
}

fn comparaison(op: &str, lhs: i32, rhs: i32) -> Result<bool, Error> {
    match op {
        "<" => Ok(lhs < rhs),
        "<=" => Ok(lhs <= rhs),
        ">" => Ok(lhs > rhs),
        ">=" => Ok(lhs >= rhs),
        "==" => Ok(lhs == rhs),
        "!=" => Ok(lhs != rhs),
        _ => Err(format_err!("Do not recognize comparaison operator: {}", op)),
    }
}

fn update(op: &str, register: i32, offset: i32) -> Result<i32, Error> {
    match op {
        "dec" => Ok(register - offset),
        "inc" => Ok(register + offset),
        _ => Err(format_err!("Not recognized operator: {}", op)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        let input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
";
        let (heap, _) = interpreter(&input);
        assert_eq!(Some(1), run_a(&heap));
    }

    #[test]
    fn test_run_b() {
        let input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
";
        let (_, history) = interpreter(&input);
        assert_eq!(Some(10), run_b(&history));
    }
}

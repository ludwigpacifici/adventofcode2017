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
    let mut file = File::open("input/day5.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    println!("a: {:?}", run_a(&input));
    println!("b: {:?}", run_b(&input));

    Ok(())
}

fn into_vec(input: &str) -> Vec<isize> {
    input.lines().filter_map(|n| n.parse().ok()).collect()
}

fn run_a(input: &str) -> usize {
    let logic = |_| 1;
    jumper(input, logic)
}

fn run_b(input: &str) -> usize {
    let logic = |offset| {
        if offset < 3 {
            1
        } else {
            -1
        }
    };
    jumper(input, logic)
}

fn jumper(input: &str, logic: fn(isize) -> isize) -> usize {
    let mut input = into_vec(input);
    let mut position = 0isize;
    let mut count = 0;

    while let Some(offset) = input.get_mut(position as usize) {
        let increment = logic(*offset);
        *offset += increment;
        position += *offset - increment;
        count += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        let input = "
0
3
0
1
-3
";
        assert_eq!(5, run_a(input));
    }

    #[test]
    fn test_run_b() {
        let input = "
0
3
0
1
-3
";
        assert_eq!(10, run_b(input));
    }
}

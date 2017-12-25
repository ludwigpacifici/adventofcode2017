extern crate failure;
extern crate primal;

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
    let mut file = File::open("input/day23.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    input.pop();

    println!("a: {:?}", run_a(&input));
    println!("b: {:?}", run_b());

    Ok(())
}

fn run_a(input: &str) -> u64 {
    let instructions: Vec<_> = input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        .collect();
    let mut cpu = Cpu::new(0, &instructions);

    while !cpu.lock {
        cpu.next();
    }

    cpu.mul_count
}

fn run_b() -> usize {
    (0..1_001)
        .filter(|&n| !primal::is_prime(107_900 + n * 17))
        .count()

    // loop {
    //     let mut f = 1;
    //     let mut d = 2;
    //     let mut g = 0;
    //     loop {
    //         let mut e = 2;
    //         loop {
    //             g = d * e - b;
    //             if g == 0 { // b == d*e i.e. b is not prime
    //                 f = 0;
    //             }
    //             e += 1;
    //             g = e - b;
    //             if g == 0 { // for e in (2..b)
    //                 break;
    //             }
    //         }
    //         d += 1;
    //         g = d - b;
    //         if g == 0 { // for d in (2..b)
    //             break;
    //         }
    //     }
    //     if f == 0 { // count not prime
    //         h += 1;
    //     }
    //     g = b - c;
    //     if g == 0 { // for i in (b..(c+1) + 17)
    //         return h;
    //     }
    //     b += 17;
    // }
}

struct Cpu<'a> {
    ip: usize,
    lock: bool,
    mul_count: u64,
    instructions: &'a [Vec<&'a str>],
    registers: [i64; 8],
}

impl<'a> Cpu<'a> {
    fn new(seed: i64, instructions: &'a [Vec<&'a str>]) -> Cpu {
        let mut cpu = Cpu {
            ip: 0,
            lock: false,
            mul_count: 0,
            instructions,
            registers: [0; 8],
        };
        cpu.set_register(parse_regrister("a"), seed);
        cpu
    }

    fn next(&mut self) {
        self.run_generic_instruction();
        self.next_ip();
    }

    fn run_generic_instruction(&mut self) {
        let instruction = &self.instructions[self.ip as usize];

        match instruction[0] {
            "set" => {
                let r = parse_regrister(instruction[1]);
                let v = self.eval(instruction[2]);
                self.set_register(r, v);
            }
            "sub" => {
                let r = parse_regrister(instruction[1]);
                let v = self.eval(instruction[1]) - self.eval(instruction[2]);
                self.set_register(r, v);
            }
            "mul" => {
                let r = parse_regrister(instruction[1]);
                let v = self.eval(instruction[2]) * self.get_register(r);
                self.set_register(r, v);
                self.mul_count += 1;
            }
            _ => {}
        };
    }

    fn next_ip(&mut self) {
        let instruction = &self.instructions[self.ip as usize];

        match instruction[0] {
            "set" | "sub" | "mul" => self.ip += 1,
            "jnz" => {
                if self.eval(instruction[1]) != 0 {
                    self.ip = (self.eval(instruction[2]) + self.ip as i64) as usize;
                } else {
                    self.ip += 1;
                }
            }
            op => eprintln!("Unknown instruction: {}", op),
        };

        if self.ip >= self.instructions.len() {
            self.lock = true;
        }
    }

    fn eval(&self, thing: &str) -> i64 {
        if let Ok(value) = thing.parse() {
            value
        } else {
            self.get_register(parse_regrister(thing))
        }
    }

    fn get_register(&self, r: usize) -> i64 {
        self.registers[r]
    }

    fn set_register(&mut self, r: usize, value: i64) {
        self.registers[r] = value;
    }
}

fn parse_regrister(r: &str) -> usize {
    usize::from(r.as_bytes()[0] - b'a')
}

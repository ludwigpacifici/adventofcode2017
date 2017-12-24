extern crate failure;

use failure::Error;
use std::collections::VecDeque;
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
    let mut file = File::open("input/day18.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    input.pop();

    println!("a: {:?}", run_a(&input));
    println!("b: {:?}", run_b(&input));

    Ok(())
}

fn run_a(input: &str) -> i64 {
    let instructions: Vec<_> = input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        .collect();
    let mut cpu = Cpu::new(0, &instructions);

    while cpu.rcv == 0 {
        cpu.next_a();
    }

    cpu.rcv
}

fn run_b(input: &str) -> u64 {
    let instructions: Vec<_> = input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        .collect();
    let mut cpu0 = Cpu::new(0, &instructions);
    let mut cpu1 = Cpu::new(1, &instructions);

    while !cpu0.lock || !cpu1.lock {
        cpu0.next_b(&mut cpu1.snd);
        cpu1.next_b(&mut cpu0.snd);
    }

    cpu1.snd_count
}

struct Cpu<'a> {
    ip: usize,
    lock: bool,
    snd_count: u64,
    rcv: i64,
    instructions: &'a [Vec<&'a str>],
    snd: VecDeque<i64>,
    registers: [i64; 26],
}

impl<'a> Cpu<'a> {
    fn new(id: i64, instructions: &'a [Vec<&'a str>]) -> Cpu {
        let mut cpu = Cpu {
            ip: 0,
            lock: false,
            snd_count: 0,
            rcv: 0,
            instructions,
            snd: VecDeque::new(),
            registers: [0; 26],
        };
        cpu.set_register(parse_regrister("p"), id);
        cpu
    }

    fn next_a(&mut self) {
        self.run_generic_instruction();
        self.run_a_instruction();
        self.next_ip();
    }

    fn run_a_instruction(&mut self) {
        let instruction = &self.instructions[self.ip as usize];

        match instruction[0] {
            "snd" => {
                let r = parse_regrister(instruction[1]);
                let v = self.get_register(r);
                self.snd.push_back(v);
            }
            "rcv" => {
                if self.eval(instruction[1]) == 0 {
                    return;
                }

                if let Some(val) = self.snd.pop_back() {
                    self.rcv = val;
                }
            }
            _ => {}
        };
    }

    fn next_b(&mut self, other_queue: &mut VecDeque<i64>) {
        self.run_generic_instruction();
        self.run_b_instruction(other_queue);
        if !self.lock {
            self.next_ip();
        }
    }

    fn run_b_instruction(&mut self, other_queue: &mut VecDeque<i64>) {
        let instruction = &self.instructions[self.ip as usize];

        match instruction[0] {
            "snd" => {
                let r = self.eval(instruction[1]);
                self.snd.push_back(r);
                self.snd_count += 1;
            }
            "rcv" => {
                if let Some(val) = other_queue.pop_front() {
                    self.lock = false;
                    self.rcv = val;
                    self.set_register(parse_regrister(instruction[1]), val);
                } else {
                    self.lock = true;
                }
            }
            _ => {}
        };
    }

    fn run_generic_instruction(&mut self) {
        let instruction = &self.instructions[self.ip as usize];

        match instruction[0] {
            "set" => {
                let r = parse_regrister(instruction[1]);
                let v = self.eval(instruction[2]);
                self.set_register(r, v);
            }
            "add" => {
                let r = parse_regrister(instruction[1]);
                let v = self.eval(instruction[2]) + self.get_register(r);
                self.set_register(r, v);
            }
            "mul" => {
                let r = parse_regrister(instruction[1]);
                let v = self.eval(instruction[2]) * self.get_register(r);
                self.set_register(r, v);
            }
            "mod" => {
                let v = self.eval(instruction[2]);
                if v != 0 {
                    let v = self.eval(instruction[1]) % v;
                    let r = parse_regrister(instruction[1]);
                    self.set_register(r, v);
                }
            }
            _ => {}
        };
    }

    fn next_ip(&mut self) {
        let instruction = &self.instructions[self.ip as usize];

        match instruction[0] {
            "snd" | "set" | "add" | "mul" | "mod" | "rcv" => self.ip += 1,
            "jgz" => {
                if self.eval(instruction[1]) > 0 {
                    self.ip = (self.eval(instruction[2]) + self.ip as i64) as usize;
                } else {
                    self.ip += 1
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        let input = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";

        assert_eq!(4, run_a(input));
    }

    #[test]
    fn test_run_b() {
        let input = "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";

        assert_eq!(3, run_b(input));
    }
}

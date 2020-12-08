use std::{
    error::Error,
    io::{BufRead, stdin},
    str::FromStr,
};

#[derive(Debug, Copy, Clone)]
enum Opcode {
    Acc,
    Jmp,
    Nop,
}
use Opcode::*;

#[derive(Debug, Copy, Clone)]
struct Instruction {
    op: Opcode,
    imm: i32,
    reached: bool,
}

impl FromStr for Instruction {
    type Err = Box::<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {

        Ok(Instruction {
            op: match &s[..3] {
                "acc" => {
                    Acc
                },
                "jmp" => {
                    Jmp
                },
                "nop" => {
                    Nop
                },
                _ => {
                    Err("Unknown instruction")?
                },
            },
            imm: s[4..].parse::<i32>()?,
            reached: false,
        })
        
    }
}

#[derive(Debug, Clone)]
struct Machine {
    acc: i32,
    pc: usize,
    code: Vec<Instruction>,
}

impl Machine {
    fn new(code: Vec<Instruction>) -> Self {
        Machine {
            acc: 0,
            pc: 0,
            code
        }
    }
    fn run(&mut self) {
        loop {
            match self.code[self.pc] {
                Instruction { reached: true, .. } => break,
                Instruction { op: Acc, imm, .. } => self.acc += imm,
                _ => (),
            };
            let pc_offset = match self.code[self.pc] {
                Instruction { op: Jmp, imm, .. } => imm as isize,
                _ => 1,
            };
            self.code[self.pc].reached = true;
            self.pc = (self.pc as isize + pc_offset) as usize;
            if self.pc == self.code.len() {
                break
            }
        }
    }
}

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();

    let code = stdin.lines().map(|r| r.unwrap().parse().unwrap()).collect::<Vec<Instruction>>();

    let mut machine = Machine::new(code.clone());
    machine.run();
    println!("part 1: {}", machine.acc);

    for i in 0..(code.len()) {
        let mut fixed_code = code.clone();
        fixed_code[i].op = match fixed_code[i].op {
            Nop => Jmp,
            Jmp => Nop,
            Acc => continue,
        };
        let mut machine = Machine::new(fixed_code);
        machine.run();
        if machine.pc == code.len() {
            println!("part 2: {}", machine.acc);
        }
    }
}
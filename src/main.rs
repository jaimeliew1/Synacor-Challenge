use std::fs;
use std::io::prelude::*;

static CHALLENGE_FN: &str = "challenge.bin";

const HALT: u16 = 0;
const SET: u16 = 1;
const PUSH: u16 = 2;
const POP: u16 = 3;
const EQ: u16 = 4;
const GT: u16 = 5;
const JMP: u16 = 6;
const JT: u16 = 7;
const JF: u16 = 8;
const ADD: u16 = 9;
const MULT: u16 = 10;
const MOD: u16 = 11;
const AND: u16 = 12;
const OR: u16 = 13;
const NOT: u16 = 14;
const RMEM: u16 = 15;
const WMEM: u16 = 16;
const CALL: u16 = 17;
const RET: u16 = 18;
const OUT: u16 = 19;
const IN: u16 = 20;
const NOOP: u16 = 21;

#[derive(Debug, Clone)]
struct Halt;

#[derive(Clone)]
struct Computer {
    cursor: usize,
    registers: [u16; 8],
    memory: [u16; 32768],
    stack: Vec<u16>,
}

#[derive(Debug)]
enum Arg {
    Reg(usize, u16),
    V(u16),
}

impl Computer {
    fn new(memory: &[u16]) -> Computer {
        let mut mem = [0_u16; 32768];
        mem[0..memory.len()].clone_from_slice(memory);
        Computer {
            cursor: 0,
            registers: [0_u16; 8],
            memory: mem,
            stack: Vec::new(),
        }
    }

    fn get_args(&self) -> (u16, u16, u16) {
        let a;
        match self.memory[self.cursor as usize] {
            PUSH | JMP | JT | JF | CALL | WMEM | OUT => {
                a = match self.memory[self.cursor + 1] {
                    v if v > 32767 && v <= 32775 => self.registers[v as usize - 32768],
                    v => v,
                };
            }
            _ => {
                a = self.memory[self.cursor + 1] as u16 - 32768;
            }
        }
        let b = *self
            .registers
            .get(self.memory[self.cursor + 2] as usize - 32768)
            .unwrap_or(&self.memory[self.cursor + 2]);
        let c = *self
            .registers
            .get(self.memory[self.cursor + 3] as usize - 32768)
            .unwrap_or(&self.memory[self.cursor + 3]);
        (a, b, c)
    }

    fn toarg(&self, argument: u16) -> Arg {
        match argument {
            v if v > 32767 && v <= 32775 => {
                Arg::Reg(v as usize - 32768, self.registers[v as usize - 32768])
            }
            v => Arg::V(v),
            _ => panic!(),
        }
    }
    fn _peek_iter(&self, _input: &Vec<u8>) {
        let (a, b, c) = self.get_args();
        let (a, b, c) = (
            self.toarg(self.memory[self.cursor + 1]),
            self.toarg(self.memory[self.cursor + 2]),
            self.toarg(self.memory[self.cursor + 3]),
        );

        let op_str = match self.memory[self.cursor] {
            HALT => "HALT",
            SET => "SET",
            PUSH => "PUSH",
            POP => "POP",
            EQ => "EQ",
            GT => "GT",
            JMP => "JMP",
            JT => "JT",
            JF => "JF",
            ADD => "ADD",
            MULT => "MULT",
            MOD => "MOD",
            AND => "AND",
            OR => "OR",
            NOT => "NOT",
            RMEM => "RMEM",
            WMEM => "WMEM",
            CALL => "CALL",
            RET => "RET",
            OUT => "OUT",
            IN => "IN",
            NOOP => "NOOP",
            _ => panic!(),
        };
        print!("op {} ", self.cursor);
        match self.memory[self.cursor] {
            EQ | GT | ADD | MULT | MOD | AND | OR => println!("{}: {:?}", op_str, (a, b, c)),
            SET | JT | JF | NOT | RMEM | WMEM => println!("{}: {:?}", op_str, (a, b)),
            OUT | IN | JMP => println!("{}: {:?}", op_str, (a)),
            NOOP => println!("{}", op_str),
            RET => println!("{}, stack {:?}", op_str, self.stack.last().unwrap()),
            CALL => println!(
                "{}, {:?}, pushed {:?} {:?} (len {})",
                op_str,
                a,
                self.cursor + 2,
                self.stack.iter().rev().take(5).collect::<Vec<_>>(),
                self.stack.len()
            ),
            POP => println!(
                "{}, {:?}, popped {:?} {:?} (len {})",
                op_str,
                a,
                self.stack.last().unwrap(),
                self.stack.iter().rev().take(5).collect::<Vec<_>>(),
                self.stack.len()
            ),
            PUSH => println!(
                "{}, {:?},  {:?} (len {})",
                op_str,
                a,
                self.stack.iter().rev().take(5).collect::<Vec<_>>(),
                self.stack.len()
            ),
            _ => panic!(),
        };
    }
    fn iter(&mut self, input: &mut Vec<u8>) -> Result<Option<u8>, Halt> {
        let op = self.memory[self.cursor];
        let (a, b, c) = self.get_args();
        self.cursor += match op {
            EQ | GT | ADD | MULT | MOD | AND | OR => 4,
            SET | JT | JF | NOT | RMEM | WMEM => 3,
            PUSH | POP | CALL | OUT | IN => 2,
            NOOP => 1,
            JMP | RET => 0,
            _ => panic!(),
        };

        match op {
            HALT => return Err(Halt),
            SET => self.registers[a as usize] = b,
            PUSH => self.stack.push(a),
            POP => self.registers[a as usize] = self.stack.pop().unwrap(),
            EQ => self.registers[a as usize] = (b == c) as u16,
            GT => self.registers[a as usize] = (b > c) as u16,
            JMP => self.cursor = a as usize,
            JT => {
                if a != 0 {
                    self.cursor = b as usize;
                }
            }
            JF => {
                if a == 0 {
                    self.cursor = b as usize;
                }
            }
            ADD => self.registers[a as usize] = (b + c) % 32768,
            MULT => self.registers[a as usize] = (b * c) % 32768,
            MOD => self.registers[a as usize] = b % c,
            AND => self.registers[a as usize] = b & c,
            OR => self.registers[a as usize] = b | c,
            NOT => self.registers[a as usize] = !b % 32768,
            RMEM => self.registers[a as usize] = self.memory[b as usize],
            WMEM => self.memory[a as usize] = b,
            CALL => {
                self.stack.push(self.cursor as u16);
                self.cursor = a as usize
            }
            RET => self.cursor = self.stack.pop().unwrap() as usize,
            IN => {
                self.registers[a as usize] = match input.pop() {
                    Some(val) => val as u16,
                    _ => std::io::stdin()
                        .bytes()
                        .next()
                        .and_then(|result| result.ok())
                        .map(|byte| byte as u16)
                        .unwrap() as u16,
                };
                return Ok(Some(self.registers[a as usize] as u8));
            }

            OUT => return Ok(Some(a as u8)),
            NOOP => (),

            _ => {
                panic!()
            }
        };
        Ok(None)
    }
}

fn main() {
    let bytes = std::fs::read(CHALLENGE_FN).unwrap();
    let data: Vec<u16> = bytes
        .chunks(2)
        .map(|b| u16::from_le_bytes([b[0], b[1]]))
        .collect();

    let input = fs::read_to_string("to_teleporter.txt").expect("can't find file");
    let mut input: Vec<u8> = input.chars().rev().map(|c| c as u8).collect();

    let mut computer = Computer::new(&data);

    while !input.is_empty() {
        if let Ok(Some(out)) = computer.iter(&mut input) {
            print!("{}", out as char);
        }
    }

    let input = fs::read_to_string("to_orb.txt").expect("can't find file");
    let mut input: Vec<u8> = input.chars().rev().map(|c| c as u8).collect();
    computer.registers[7] = 25734 as u16;
    while !input.is_empty() {
        // skip super long loop
        if computer.cursor == 5489 {
            computer.cursor = 5491;
            computer.registers[0] = 6;
            computer.registers[1] = 5;
        }
        if let Ok(Some(out)) = computer.iter(&mut input) {
            print!("{}", out as char);
        }
    }

    let mut empty_vec = Vec::new();
    loop {
        // computer.peek_iter(&empty_vec);
        if let Ok(Some(out)) = computer.iter(&mut empty_vec) {
            print!("{}", out as char);
        }
    }
}

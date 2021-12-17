use std::io::Read;

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

fn get_args(cursor: &usize, memory: &[u16], registers: &[u16]) -> (u16, u16, u16) {
    let a;
    match memory[*cursor as usize] {
        PUSH | JMP | JT | JF | CALL | WMEM | OUT => {
            a = match memory[cursor + 1] {
                v if v > 32767 && v <= 32775 => registers[v as usize - 32768],
                v => v,
            };
        }
        _ => {
            a = memory[cursor + 1] as u16 - 32768;
        }
    }
    let b = *registers
        .get(memory[*cursor + 2] as usize - 32768)
        .unwrap_or(&memory[*cursor + 2]);
    let c = *registers
        .get(memory[*cursor + 3] as usize - 32768)
        .unwrap_or(&memory[*cursor + 3]);
    (a, b, c)
}

fn compute(data: &Vec<u16>) {
    let mut cursor: usize = 0;
    let mut registers = [0_u16; 8];
    let mut memory = [0_u16; 32768];
    let mut stack: Vec<u16> = Vec::new();
    // let mut input;
    let mut step = 0;

    memory[0..data.len()].clone_from_slice(data);
    loop {
        step += 1;
        let op = memory[cursor];
        let (a, b, c) = get_args(&cursor, &memory, &registers);
        cursor += match op {
            EQ | GT | ADD | MULT | MOD | AND | OR => 4,
            SET | JT | JF | NOT | RMEM | WMEM => 3,
            PUSH | POP | CALL | OUT | IN => 2,
            NOOP => 1,
            JMP | RET => 0,
            _ => panic!(),
        };

        match op {
            HALT => break,
            SET => registers[a as usize] = b,
            PUSH => stack.push(a),
            POP => registers[a as usize] = stack.pop().unwrap(),
            EQ => registers[a as usize] = (b == c) as u16,
            GT => registers[a as usize] = (b > c) as u16,
            JMP => cursor = a as usize,
            JT => {
                if a != 0 {
                    cursor = b as usize;
                }
            }
            JF => {
                if a == 0 {
                    cursor = b as usize;
                }
            }
            ADD => registers[a as usize] = (b + c) % 32768,
            MULT => registers[a as usize] = (b * c) % 32768,
            MOD => registers[a as usize] = b % c,
            AND => registers[a as usize] = b & c,
            OR => registers[a as usize] = b | c,
            NOT => registers[a as usize] = !b % 32768,
            RMEM => registers[a as usize] = memory[b as usize],
            WMEM => memory[a as usize] = b,
            CALL => {
                stack.push(cursor as u16);
                cursor = a as usize
            }
            RET => cursor = stack.pop().unwrap() as usize,
            IN => {
                registers[a as usize] = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as u16)
                    .unwrap()
            }

            OUT => print!("{}", a as u8 as char),
            NOOP => (),

            _ => {
                println!("Operation: {}", op);
                break;
            }
        }
    }
}
fn main() {
    let bytes = std::fs::read(CHALLENGE_FN).unwrap();

    let data: Vec<u16> = bytes
        .chunks(2)
        .map(|b| u16::from_le_bytes([b[0], b[1]]))
        .collect();

    compute(&data);
}

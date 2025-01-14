use super::load_file::load_file;

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

#[derive(Clone, Copy, PartialEq, Eq)]
enum OpCode {
    // a = a / 2^x
    Adv,
    // b = b xor x
    Bxl,
    // b = x % 8
    Bst,
    // jump to x if a != 0
    Jnz,
    // B = B xor C
    Bxc,
    // print x % 8
    Out,
    // b = a / 2^x
    Bdv,
    // c = a / 2^x
    Cdv,
}

const OP_CODES: [OpCode; 8] = [
    OpCode::Adv,
    OpCode::Bxl,
    OpCode::Bst,
    OpCode::Jnz,
    OpCode::Bxc,
    OpCode::Out,
    OpCode::Bdv,
    OpCode::Cdv,
];

struct Instruction {
    op_code: OpCode,
    x: u8,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = if self.x <= 3 {
            self.x.to_string()
        } else {
            ((b'A' + self.x - 4) as char).to_string()
        };
        let x = self.x;
        match self.op_code {
            // a = a / 2^op
            OpCode::Adv => write!(f, "DIV A, 2^{op}"),
            // b = b xor x
            OpCode::Bxl => write!(f, "XOR B, {x}"),
            // b = op % 8
            OpCode::Bst => write!(f, "MOD B, {op}, 8"),
            // jump to x if a != 0
            OpCode::Jnz => write!(f, "JNZ {x}"),
            // B = B xor C
            OpCode::Bxc => write!(f, "XOR B, C"),
            // print op % 8
            OpCode::Out => write!(f, "OUT {op} % 8"),
            // b = a / 2^op
            OpCode::Bdv => write!(f, "DIV B, A, 2^{op}"),
            // c = a / 2^op
            OpCode::Cdv => write!(f, "DIV C, A, 2^{op}"),
        }
    }
}

fn decode(i: u8, x: u8) -> Instruction {
    Instruction {
        op_code: OP_CODES[i as usize],
        x,
    }
}

pub struct State {
    reg: [usize; 3],
    ip: usize,
    memory: Vec<u8>,
    output: Vec<u8>,
}

impl State {
    fn with_program(program: Vec<u8>) -> Self {
        Self {
            reg: [0; 3],
            ip: 0,
            memory: program,
            output: Vec::<u8>::new(),
        }
    }

    fn fetch(&mut self) -> Option<Instruction> {
        if self.ip < self.memory.len() {
            let result = decode(self.memory[self.ip], self.memory[self.ip + 1]);
            self.ip += 2;
            Some(result)
        } else {
            None
        }
    }

    fn execute(&mut self, Instruction { op_code, x }: Instruction) {
        let operand = self.operand(x);
        match op_code {
            OpCode::Adv => {
                self.reg[A] /= 2usize.pow(operand as u32);
            }
            OpCode::Bxl => {
                self.reg[B] ^= x as usize;
            }
            OpCode::Bst => {
                self.reg[B] = operand % 8;
            }
            OpCode::Jnz => {
                if self.reg[A] != 0 {
                    self.ip = x as usize;
                }
            }
            OpCode::Bxc => {
                self.reg[B] ^= self.reg[C];
            }
            OpCode::Out => {
                self.output.push((operand % 8) as u8);
            }
            OpCode::Bdv => {
                self.reg[B] = self.reg[A] / 2usize.pow(operand as u32);
            }
            OpCode::Cdv => {
                self.reg[C] = self.reg[A] / 2usize.pow(operand as u32);
            }
        }
    }

    fn operand(&self, x: u8) -> usize {
        match x {
            0..=3 => x as usize,
            4 => self.reg[A],
            5 => self.reg[B],
            6 => self.reg[C],
            _ => unreachable!(),
        }
    }

    pub fn print_program(&self) {
        for i in (0..self.memory.len()).step_by(2) {
            let result = decode(self.memory[i], self.memory[i + 1]);
            println!("{result}");
        }
    }
}

fn preprocess(input: String) -> State {
    let mut chunks = input.split("\n\n");

    let registers = chunks
        .next()
        .unwrap()
        .lines()
        .filter(|x| !x.is_empty())
        .enumerate();
    assert!(registers.clone().count() == 3);

    let program = chunks
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    assert!(chunks.next().is_none());

    let mut state = State::with_program(program);

    for (i, line) in registers {
        let mut iter = line.split(":");
        state.reg[i] = iter.nth(1).unwrap().trim().parse::<usize>().unwrap();
    }
    state
}

pub fn part1(input: String) -> String {
    let mut state = preprocess(input);

    while let Some(instruction) = state.fetch() {
        let reg = state.reg;
        state.execute(instruction);
        for i in 0..state.reg.len() {
            if reg[i] != state.reg[i] {}
        }
    }

    state
        .output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn part2_helper(state: &mut State, a: usize, n: usize) -> Option<usize> {
    let start = if n == 0 { 1 } else { 0 };
    let memory_start = state.memory.len() - (n + 1);
    'outer: for i in start..8 {
        let a = a + i;
        state.reg[A] = a;
        state.reg[B] = 0;
        state.reg[C] = 0;
        state.ip = 0;
        state.output.clear();
        while let Some(instruction) = state.fetch() {
            let out = instruction.op_code == OpCode::Out;
            state.execute(instruction);
            if out {
                let index = state.output.len() - 1;
                if state.output[index] != state.memory[memory_start + index] {
                    continue 'outer;
                }
            }
        }
        assert!(state.output.len() == (n + 1));

        if n == 15 {
            return Some(a);
        } else {
            let result = part2_helper(state, a * 8, n + 1);
            if result.is_some() {
                return result;
            }
        }
    }
    None
}

pub fn part2(input: String) -> usize {
    let mut state = preprocess(input);
    part2_helper(&mut state, 0, 0).unwrap()
}

pub fn test_input() -> String {
    load_file("res/day_17_test_input.txt")
}

pub fn input() -> String {
    load_file("res/day_17_input.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    static PART1_TEST_EXPECTED_RESULT: &str = "4,6,3,5,6,3,5,2,1,0";
    static PART1_EXPECTED_RESULT: &str = "1,6,3,6,5,6,5,1,7";
    static PART2_EXPECTED_RESULT: usize = 247839653009594;

    #[test]
    fn part1_with_test_input() {
        assert_eq!(part1(test_input()), PART1_TEST_EXPECTED_RESULT);
    }

    #[test]
    fn part1_with_input() {
        assert_eq!(part1(input()), PART1_EXPECTED_RESULT);
    }

    #[test]
    fn part2_with_input() {
        assert_eq!(part2(input()), PART2_EXPECTED_RESULT);
    }
}

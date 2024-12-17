use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn parse_register(register: &str) -> usize {
    let (_, value) = register
        .split_once(": ")
        .expect("register seperated by ': '");

    value.parse().expect("valid register value")
}

fn parse_registers(registers: &[String]) -> (usize, usize, usize) {
    let a = parse_register(&registers[0]);
    let b = parse_register(&registers[1]);
    let c = parse_register(&registers[2]);

    (a, b, c)
}

fn parse_instructions(instructions: &str) -> Vec<u8> {
    let (_, instructions) = instructions
        .split_once(": ")
        .expect("instructions seperated by ': '");

    instructions
        .split(',')
        .map(|value| value.parse().expect("valid u8"))
        .collect()
}

struct Program {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,

    instructions: Vec<u8>,

    instruction_pointer: usize,
}

impl Program {
    fn new(registers: (usize, usize, usize), instructions: Vec<u8>) -> Self {
        Self {
            reg_a: registers.0,
            reg_b: registers.1,
            reg_c: registers.2,

            instructions,
            instruction_pointer: 0,
        }
    }

    fn run(&mut self) -> Vec<usize> {
        let mut output = Vec::new();

        while self.instructions.get(self.instruction_pointer).is_some() {
            if let Some(out) = self.run_next_instruction() {
                output.push(out);
            }
        }

        output
    }

    fn run_next_instruction(&mut self) -> Option<usize> {
        match self.instructions[self.instruction_pointer] {
            0 => {
                self.adv();
                None
            }
            1 => {
                self.bxl();
                None
            }
            2 => {
                self.bst();
                None
            }
            3 => {
                self.jnz();
                None
            }
            4 => {
                self.bxc();
                None
            }
            5 => Some(self.out()),
            6 => {
                self.bdv();
                None
            }
            7 => {
                self.cdv();
                None
            }
            x => panic!("Invalid instruction {x}"),
        }
    }

    fn get_combo_operand(&self, operand: u8) -> usize {
        match operand {
            0..=3 => operand as usize,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            x => panic!("Invalid operand {x}"),
        }
    }

    fn adv(&mut self) {
        let numerator = self.reg_a;
        let denominator = 2_usize
            .pow(self.get_combo_operand(self.instructions[self.instruction_pointer + 1]) as u32);

        self.reg_a = numerator / denominator;
        self.instruction_pointer += 2;
    }

    fn bxl(&mut self) {
        let operand = self.instructions[self.instruction_pointer + 1];

        self.reg_b ^= operand as usize;
        self.instruction_pointer += 2;
    }

    fn bst(&mut self) {
        let operand = self.get_combo_operand(self.instructions[self.instruction_pointer + 1]);

        self.reg_b = operand % 8;
        self.instruction_pointer += 2;
    }

    fn jnz(&mut self) {
        if self.reg_a == 0 {
            self.instruction_pointer += 2;
            return;
        }

        self.instruction_pointer = self.instructions[self.instruction_pointer + 1] as usize;
    }

    fn bxc(&mut self) {
        self.reg_b ^= self.reg_c;
        self.instruction_pointer += 2;
    }

    fn out(&mut self) -> usize {
        let operand = self.get_combo_operand(self.instructions[self.instruction_pointer + 1]);
        self.instruction_pointer += 2;
        operand % 8
    }

    fn bdv(&mut self) {
        let numerator = self.reg_a;
        let denominator = 2_usize
            .pow(self.get_combo_operand(self.instructions[self.instruction_pointer + 1]) as u32);

        self.reg_b = numerator / denominator;
        self.instruction_pointer += 2;
    }

    fn cdv(&mut self) {
        let numerator = self.reg_a;
        let denominator = 2_usize
            .pow(self.get_combo_operand(self.instructions[self.instruction_pointer + 1]) as u32);

        self.reg_c = numerator / denominator;
        self.instruction_pointer += 2;
    }
}

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|line| line.expect("valid line"))
        .collect::<Vec<_>>();

    let mut splits = lines.split(|line| line.is_empty());
    let registers = splits.next().expect("registers are present");
    let instructions = splits.next().expect("program is present");

    let registers = parse_registers(registers);
    let instructions = parse_instructions(&instructions[0]);

    let mut program = Program::new(registers, instructions);
    let outputs = program.run();

    let output = outputs
        .iter()
        .map(|output| output.to_string())
        .collect::<Vec<_>>()
        .join(",");

    println!("Program output is: {output}");

    Ok(())
}

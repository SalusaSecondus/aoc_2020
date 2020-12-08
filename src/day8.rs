use crate::read_file;
use anyhow::{bail, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Clone)]
enum OpCode {
    Nop,
    Acc,
    Jmp,
}

impl OpCode {
    fn parse(text: &str) -> Result<OpCode> {
        let text = text.trim();
        let op = match text {
            "nop" => OpCode::Nop,
            "acc" => OpCode::Acc,
            "jmp" => OpCode::Jmp,
            _ => bail!("Invalid OpCode"),
        };
        Ok(op)
    }
}
#[derive(Debug, Clone)]
struct Instruction {
    op: OpCode,
    arg: i32,
}

impl Instruction {
    fn parse(line: &str) -> Result<Instruction> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(...) +(.*)$").unwrap();
        }
        let c = RE.captures(line).context("Invalid line")?;
        let op = c.get(1).context("Invalid line")?;
        let op = OpCode::parse(op.as_str())?;
        let arg = c.get(2).context("Invalid line")?.as_str().parse()?;

        Ok(Instruction { op, arg })
    }
}
#[derive(Debug, Clone)]
struct Program {
    pub pc: usize,
    pub accumulator: i32,
    pub instructions: Vec<Instruction>,
}

impl Program {
    fn load_program(file_name: &str) -> Result<Program> {
        let mut instructions = vec![];
        let lines = read_file(file_name)?;
        for line in lines {
            let line = line?;
            let line = line.trim();
            instructions.push(Instruction::parse(line)?);
        }

        Ok(Program {
            pc: 0,
            accumulator: 0,
            instructions,
        })
    }

    fn step(&mut self) -> Result<()> {
        let instruction = self
            .instructions
            .get(self.pc)
            .context("No such instruction")?;

        // println!("OP: {:?} {}", instruction.op, instruction.arg);
        match &instruction.op {
            OpCode::Nop => self.pc += 1,
            OpCode::Jmp => self.pc = (instruction.arg + self.pc as i32) as usize,
            OpCode::Acc => {
                self.pc += 1;
                self.accumulator += instruction.arg;
            }
        }
        // println!("Inner pc: {}", self.pc);
        Ok(())
    }

    fn run_till_end(&mut self) -> Result<()> {
        let mut seen = HashSet::new();

        while !seen.contains(&self.pc) {
            seen.insert(self.pc);
            if self.step().is_err() {
                return Ok(());
            }
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day8_smoke1() -> Result<()> {
        let mut program = Program::load_program("day8_smoke.txt")?;
        program.run_till_end()?;
        assert_eq!(5, program.accumulator);
        Ok(())
    }
    #[test]
    fn day8_1() -> Result<()> {
        let mut program = Program::load_program("day8.txt")?;
        program.run_till_end()?;
        println!("Day8.1: {}", program.accumulator);
        Ok(())
    }

    #[test]
    fn day8_2() -> Result<()> {
        let template = Program::load_program("day8.txt")?;
        for idx in 0 .. template.instructions.len() {
            let mut dupe = template.clone();
            match dupe.instructions[idx].op {
                OpCode::Nop => dupe.instructions[idx].op = OpCode::Jmp,
                OpCode::Jmp => dupe.instructions[idx].op = OpCode::Nop,
                _ => (),
            }
            dupe.run_till_end()?;
            if dupe.pc == template.instructions.len() {
                println!("Day8.2: {}", dupe.accumulator);
                return Ok(())
            }
        }
        Ok(())
    }
}

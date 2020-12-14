use anyhow::{bail, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct BitMask {
    ones: u64,
    zeros: u64,
    display: String,
}

impl FromStr for BitMask {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 36 {
            bail!("Invalid length for BitMask");
        }
        let mut ones = 0;
        let mut zeros = 0;
        for c in s.chars() {
            ones <<= 1;
            zeros <<= 1;
            zeros += 1;
            match c {
                'X' => (),
                '1' => ones += 1,
                '0' => zeros -= 1,
                _ => bail!("Invalid symbol"),
            };
        }

        Ok(BitMask {
            ones,
            zeros,
            display: s.to_owned(),
        })
    }
}

impl fmt::Display for BitMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display)
    }
}

impl BitMask {
    fn new() -> BitMask {
        "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".parse().unwrap()
    }

    fn apply(&self, num: u64) -> u64 {
        let num = num | self.ones;
        let num = num & self.zeros;
        num
    }

    fn apply2(&self, num: u64) -> Vec<u64> {
        let mut result = vec![num | self.ones];
        let mut shift = 35;
        for x in self.display.chars() {
            let or_mask = 1 << shift;
            let and_mask = !or_mask;
            if x == 'X' {
                result = result
                    .iter()
                    .flat_map(|v| vec![*v | or_mask, *v & and_mask])
                    .collect();
            }
            shift -= 1;
        }

        result
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Mask(BitMask),
    Store(u64, u64),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref MASK_RE: Regex = Regex::new(r"^mask = ([X01]{36})$").unwrap();
            static ref STORE_RE: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
        }

        if let Some(c) = STORE_RE.captures(s) {
            let loc = c.get(1).context("No location?")?.as_str().parse()?;
            let value = c.get(2).context("No value?")?.as_str().parse()?;
            return Ok(Instruction::Store(loc, value));
        }
        if let Some(c) = MASK_RE.captures(s) {
            let mask = c.get(1).context("No mask?")?.as_str().parse()?;
            return Ok(Instruction::Mask(mask));
        }
        bail!("Invalid instruction");
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Mask(mask) => write!(f, "mask = {}", mask),
            Instruction::Store(loc, value) => write!(f, "mem[{}] = {}", loc, value),
        }
    }
}

fn load_program(file_name: &str) -> Result<Vec<Instruction>> {
    let mut result = vec![];
    for line in crate::read_file(file_name)? {
        let line = line?;
        let line = line.trim();
        result.push(line.parse()?);
    }
    Ok(result)
}

struct Computer {
    mask: BitMask,
    memory: HashMap<u64, u64>,
}

impl Computer {
    fn new() -> Computer {
        Computer {
            mask: BitMask::new(),
            memory: HashMap::new(),
        }
    }

    fn execute(&mut self, program: &[Instruction]) {
        for i in program {
            match i {
                Instruction::Mask(mask) => self.mask = mask.clone(),
                Instruction::Store(loc, value) => {
                    self.memory.insert(*loc, self.mask.apply(*value));
                }
            };
        }
    }

    fn execute2(&mut self, program: &[Instruction]) {
        for i in program {
            match i {
                Instruction::Mask(mask) => self.mask = mask.clone(),
                Instruction::Store(loc, value) => {
                    for mem in self.mask.apply2(*loc) {
                        self.memory.insert(mem, *value);
                    }
                }
            };
        }
    }

    fn sum(&self) -> u64 {
        self.memory.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14_smoke1() -> Result<()> {
        let program = load_program("day14_smoke.txt")?;
        let mut computer = Computer::new();
        computer.execute(&program);
        assert_eq!(165, computer.sum());
        assert_eq!(101, *computer.memory.get(&7).unwrap());
        assert_eq!(64, *computer.memory.get(&8).unwrap());
        assert_eq!(2, computer.memory.len());

        Ok(())
    }

    #[test]
    fn day14_1() -> Result<()> {
        let program = load_program("day14.txt")?;
        let mut computer = Computer::new();
        computer.execute(&program);
        println!("Day 14.1: {}", computer.sum());

        Ok(())
    }

    #[test]
    fn day14_smoke2() -> Result<()> {
        let program = load_program("day14_smoke2.txt")?;
        let mut computer = Computer::new();
        computer.execute2(&program);
        assert_eq!(208, computer.sum());

        Ok(())
    }

    #[test]
    fn day14_2() -> Result<()> {
        let program = load_program("day14.txt")?;
        let mut computer = Computer::new();
        computer.execute2(&program);
        println!("Day 14.2: {}", computer.sum());

        Ok(())
    }
}

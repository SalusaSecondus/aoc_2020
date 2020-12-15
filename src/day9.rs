use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};

use crate::read_file;
use anyhow::{bail, Context, Result};

fn load_numbers(file_name: &str) -> Result<VecDeque<u64>> {
    let mut result = VecDeque::new();

    for line in read_file(file_name)? {
        let line = line?;
        let line = line.trim();
        result.push_back(line.parse()?);
    }
    Ok(result)
}
#[derive(Debug)]
struct XmasState {
    preamble_size: usize,
    past: VecDeque<u64>,
    preamble: VecDeque<u64>,
    future: VecDeque<u64>,
}

impl XmasState {
    fn load(mut values: VecDeque<u64>, preamble_size: usize) -> XmasState {
        let future = values.split_off(preamble_size);
        XmasState {
            preamble_size,
            past: VecDeque::new(),
            preamble: values,
            future,
        }
    }

    fn is_valid(&self, next_value: u64) -> bool {
        let mut map = HashMap::new();
        for n in &self.preamble {
            let count = map.entry(n).or_insert(0);
            *count += 1;
        }

        for n in &self.preamble {
            if n > &next_value {
                continue;
            }
            let other = next_value - n;
            let needed = if other == *n { 2 } else { 1 };
            if map.get(&other).unwrap_or(&0) >= &needed {
                return true;
            }
        }
        false
    }

    fn is_next_valid(&self) -> bool {
        let next_value = self.future.get(0);
        if next_value.is_none() {
            return false;
        }
        self.is_valid(*next_value.unwrap())
    }

    fn step(&mut self) -> Result<()> {
        let next_value = self.future.pop_front().context("No more values")?;
        self.preamble.push_back(next_value);
        let discarded = self.preamble.pop_front().context("Assertion error")?;
        self.past.push_back(discarded);

        Ok(())
    }

    fn find_first_invalid(&mut self) -> Result<u64> {
        while self.is_next_valid() {
            self.step()?;
        }

        Ok(*self.future.get(0).context("No more values")?)
    }
}

fn find_range_sum(values: &[u64], target: &u64) -> Result<(u64, u64)> {
    for start in 0..(values.len() - 1) {
        let mut min = u64::MAX;
        let mut max = 0;
        let mut running_sum = 0;
        for curr in &values[start..] {
            if *curr < min {
                min = *curr;
            }
            if *curr > max {
                max = *curr;
            }
            running_sum += curr;
            match running_sum.cmp(target) {
                Ordering::Greater => continue,
                Ordering::Equal => return Ok((min, max)),
                _ => (),
            }
        }
    }

    bail!("No range found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9_smoke1() -> Result<()> {
        let mut values = load_numbers("day9_smoke.txt")?;
        let mut state = XmasState::load(values.clone(), 5);
        println!("Day9 smoke1 state {:?}", state);
        assert_eq!(127, state.find_first_invalid()?);

        let values = values.make_contiguous();
        let (min, max) = find_range_sum(values, &127)?;
        assert_eq!(15, min);
        assert_eq!(47, max);
        let result = min + max;
        assert_eq!(62, result);
        Ok(())
    }

    #[test]
    fn day9_1() -> Result<()> {
        let mut values = load_numbers("day9.txt")?;
        let mut state = XmasState::load(values.clone(), 25);
        let first_invalid = state.find_first_invalid()?;
        println!("Day9.1 {}", first_invalid);
        assert_eq!(29221323, first_invalid);

        let values = values.make_contiguous();
        let (min, max) = find_range_sum(values, &first_invalid)?;
        println!("Day9.2 {}", min + max);
        assert_eq!(4389369, min + max);
        Ok(())
    }
}

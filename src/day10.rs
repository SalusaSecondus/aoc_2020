use std::collections::{HashMap, HashSet};

use anyhow::{bail, Context, Result};

fn find_diffs(adapters: &Vec<i64>) -> Result<[u32; 3]> {
    let mut result = [0, 0, 0];

    let mut adapters = adapters.clone();
    adapters.sort();
    let mut previous = 0;
    for j in adapters {
        let gap = j - previous;
        if gap == 0 || gap > 3 {
            bail!("Invalid gap");
        }
        result[(gap - 1) as usize] += 1;
        previous = j;
    }
    result[2] += 1; // Laptop
    Ok(result)
}

fn count_ways(adapters: &Vec<i64>) -> Result<(i64, i64, HashMap<i64, i64>)> {
    let mut adapters = adapters.clone();
    adapters.sort();

    let mut set = HashSet::new();
    for a in &adapters {
        set.insert(a);
    }

    let mut ways = HashMap::new();
    ways.insert(0, 1);

    let laptop = adapters.last().context("No elements?")? + 3;
    adapters.push(laptop);

    for j in adapters {
        let mut count = 0;
        count += ways.get(&(j - 3)).unwrap_or(&0);
        count += ways.get(&(j - 2)).unwrap_or(&0);
        count += ways.get(&(j - 1)).unwrap_or(&0);
        ways.insert(j, count);
        // println!("{} has {} ways", j, count);
    }

    Ok((laptop, *ways.get(&laptop).unwrap(), ways))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_numbers;

    #[test]
    fn day10_smoke1() -> Result<()> {
        let adapters = load_numbers("day10_smoke1.txt")?;
        let gaps = find_diffs(&adapters)?;
        assert_eq!(7, gaps[0]);
        assert_eq!(5, gaps[2]);

        let adapters = load_numbers("day10_smoke2.txt")?;
        let gaps = find_diffs(&adapters)?;
        assert_eq!(22, gaps[0]);
        assert_eq!(10, gaps[2]);
        Ok(())
    }

    #[test]
    fn day10_1() -> Result<()> {
        let adapters = load_numbers("day10.txt")?;
        let gaps = find_diffs(&adapters)?;
        println!("Day 10.1 gaps {:?}, answer {}", gaps, gaps[0] * gaps[2]);

        Ok(())
    }

    #[test]
    fn day10_smoke2() -> Result<()> {
        let adapters = load_numbers("day10_smoke1.txt")?;

        let answer = count_ways(&adapters)?;
        println!("Day10 smoke 2 ways {:?}", answer.2);
        assert_eq!(22, answer.0);
        assert_eq!(8, answer.1);

        let adapters = load_numbers("day10_smoke2.txt")?;

        let answer = count_ways(&adapters)?;
        assert_eq!(52, answer.0);
        assert_eq!(19208, answer.1);

        Ok(())
    }

    #[test]
    fn day10_2() -> Result<()> {
        let adapters = load_numbers("day10.txt")?;

        let answer = count_ways(&adapters)?;
        println!("Day10.2 {}", answer.1);

        Ok(())
    }
}

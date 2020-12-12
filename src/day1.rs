#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};
    use std::collections::HashMap;

    use crate::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn day_1_1() -> Result<()> {
        let mut entries = HashMap::new();
        for line in read_file("day1.txt")? {
            let val: i32 = line?.parse()?;
            entries.insert(val, 1 as i32);
        }

        for k in entries.keys() {
            let target = 2020 - k;
            if entries.contains_key(&target) {
                println!("Day1.1: {}", k * (2020 - k));
                return Ok(());
            }
        }

        Err(anyhow!("No solution found"))
    }

    #[test]
    fn day_1_2() -> Result<()> {
        let mut entries = HashMap::new();
        for line in read_file("day1.txt")? {
            let val: i32 = line?.parse()?;
            entries.insert(val, 1 as i32);
        }

        for k1 in entries.keys() {
            if *k1 > 2020 {
                continue;
            }
            for k2 in entries.keys() {
                if k1 + k2 > 2020 {
                    continue;
                }
                let target = 2020 - k1 - k2;
                if entries.contains_key(&target) {
                    println!("Day1.2: {}", k1 * k2 * target);
                    return Ok(());
                }
            }
        }

        Err(anyhow!("No solution found"))
    }
}

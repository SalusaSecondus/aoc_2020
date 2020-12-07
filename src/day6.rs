use crate::read_file;
use anyhow::Result;
use std::collections::HashMap;

struct Group {
    counts: HashMap<char, u32>,
    size: u32
}

fn read_yeses(file_name: &str) -> Result<Vec<Group>> {
    let mut result = vec![];
    let mut curr_group = HashMap::new();
    let mut size = 0;
    for line in read_file(file_name)? {
        let line = line?;
        let line = line.trim();
        if line.len() == 0 && curr_group.len() > 0 {
            result.push(
                Group {
                    counts: curr_group, size
                }

            );
            curr_group = HashMap::new();
            size = 0;
        } else {
            size += 1;
            for ans in line.chars() {
                let counter = curr_group.entry(ans).or_insert(0);
                *counter += 1;
            }
        }
    }
    if curr_group.len() > 0 {
        result.push(
            Group {
                counts: curr_group, size
            }

        );
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6_smoke() -> Result<()> {
        let groups = read_yeses("day6_smoke.txt")?;
        let mut count = 0;
        for g in &groups {
            count += g.counts.len();
        }
        println!("Day6 smoke: {}", count);

        count = 0;
        for g in &groups {
            for c in g.counts.values() {
                if *c == g.size {
                    count += 1;
                }
            }
        }
        println!("Day6 smoke2: {}", count);

        Ok(())
    }

    #[test]
    fn day6_1() -> Result<()> {
        let groups = read_yeses("day6.txt")?;
        let mut count = 0;
        for g in groups {
            count += g.counts.len();
        }
        println!("Day6.1: {}", count);

        Ok(())
    }
    
    #[test]
    fn day6_2() -> Result<()> {
        let groups = read_yeses("day6.txt")?;
        let mut count = 0;
        for g in &groups {
            for c in g.counts.values() {
                if *c == g.size {
                    count += 1;
                }
            }
        }
        println!("Day6.2: {}", count);

        Ok(())
    }
}

use anyhow::{Context, Result};

fn parse_busses(line: &str) -> Result<Vec<u64>> {
    line.split(',')
        .map(|item| {
            if item != "x" {
                item.parse().context("Could not parse")
            } else {
                Ok(1)
            }
        })
        .collect()
}

fn parse_problem(file_name: &str) -> Result<(u64, Vec<u64>)> {
    let mut input = crate::read_file(file_name)?;

    let line = input.next().context("No first line")??;

    let earliest_time = line.parse()?;
    let line = input.next().context("No second line")??;
    let busses = parse_busses(&line)?;

    Ok((earliest_time, busses))
}

fn print_crt_problem(busses: &[u64]) {
    for (idx, bus) in busses.iter().enumerate() {
        if *bus == 0 {
            continue;
        }
        let remainder = to_mod(idx as u64, *bus);

        println!("x = {} mod {}", remainder, bus);
    }
}

fn to_mod(idx: u64, bus: u64) -> u64 {
    let mut result = idx % bus;
    if result != 0 {
        result = bus - result;
    }
    result
}

fn crt_brute_force(nums: &[u64]) -> u64 {
    let mut solution = 0;
    let mut step = 1u64;

    for (idx, bus) in nums.iter().enumerate() {
        let target = to_mod(idx as u64, *bus);

        while solution % bus != target {
            solution += step;
        }
        step *= bus;
    }

    solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day13_smoke1() -> Result<()> {
        let problem = parse_problem("day13_smoke.txt")?;
        // print_crt_problem(&problem.1);
        let mut best_wait = u64::MAX;
        let mut best_bus = 0;

        for bus in problem.1 {
            if bus == 1 {
                continue;
            }
            let wait = to_mod(problem.0, bus);
            if wait < best_wait {
                best_wait = wait;
                best_bus = bus;
            }
        }
        let soonest = problem.0 + best_wait;
        assert_eq!(59, best_bus);
        assert_eq!(944, soonest);
        println!("Day 13.smoke: {}", best_wait * best_bus);

        Ok(())
    }

    #[test]
    fn day13_1() -> Result<()> {
        let problem = parse_problem("day13.txt")?;
        // print_crt_problem(&problem.1);
        let mut best_wait = u64::MAX;
        let mut best_bus = 0;

        for bus in problem.1 {
            if bus == 1 {
                continue;
            }
            let wait = to_mod(problem.0, bus);
            if wait < best_wait {
                best_wait = wait;
                best_bus = bus;
            }
        }
        println!("Day 13.1: {}", best_wait * best_bus);
        assert_eq!(119, best_wait * best_bus);

        Ok(())
    }

    #[test]
    fn day13_smoke2() -> Result<()> {
        let problem = parse_problem("day13_smoke.txt")?;
        let answer = crt_brute_force(&problem.1);

        assert_eq!(1068781, answer);

        Ok(())
    }

    #[test]
    fn day13_2() -> Result<()> {
        let problem = parse_problem("day13.txt")?;
        let answer = crt_brute_force(&problem.1);

        println!("Day 13.2: {}", answer);
        assert_eq!(1106724616194525, answer);
        Ok(())
    }
}

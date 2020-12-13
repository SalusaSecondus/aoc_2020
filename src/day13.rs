use anyhow::{Context, Result};

fn parse_busses(line: &str) -> Result<Vec<i32>> {
    let mut result = vec![];
    for item in line.split(",") {
        if item != "x" {
            result.push(item.parse()?);
        } else {
            result.push(0);
        }
    }

    Ok(result)
}

fn parse_problem(file_name: &str) -> Result<(i32, Vec<i32>)> {
    let mut lines = vec![];
    for l in crate::read_file(file_name)? {
        lines.push(l?);
    }

    let earliest_time = lines.get(0).context("No earliest time")?.parse()?;
    let busses = lines.get(1).context("No busses")?;
    let busses = parse_busses(&busses)?;

    Ok((earliest_time, busses))
}

fn print_crt_problem(busses: &[i32]) { 
    for idx in 0..busses.len() {
        let bus = busses[idx];
        if bus == 0 {
            continue;
        }
        let mut wait = idx as i32 % bus;
        if wait != 0 {
            wait = bus - wait;
        }

        println!("x = {} mod {}", wait, bus);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day13_smoke1() -> Result<()> {
        let problem = parse_problem("day13_smoke.txt")?;
        print_crt_problem(&problem.1);
        let mut best_wait = i32::MAX;
        let mut best_bus = 0;

        for bus in problem.1 {
            if bus == 0 {
                continue;
            }
            let mut wait = problem.0 % bus;
            if wait != 0 {
                wait = bus - wait;
            }
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
        print_crt_problem(&problem.1);
        let mut best_wait = i32::MAX;
        let mut best_bus = 0;

        for bus in problem.1 {
            if bus == 0 {
                continue;
            }
            let mut wait = problem.0 % bus;
            if wait != 0 {
                wait = bus - wait;
            }
            if wait < best_wait {
                best_wait = wait;
                best_bus = bus;
            }
        }
        println!("Day 13.1: {}", best_wait * best_bus);
        
        Ok(())
    }
}

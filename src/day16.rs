use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, collections::HashSet, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct FieldRule {
    range1: (i32, i32),
    range2: (i32, i32),
    name: String,
}

impl FromStr for FieldRule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([^:]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
        }
        let c = RE.captures(s).context("Invalid Line")?;
        let name = c.get(1).map(|c| c.as_str()).context("No name?")?.to_owned();
        let min1 = c.get(2).context("Missing group?")?.as_str().parse()?;
        let max1 = c.get(3).context("Missing group?")?.as_str().parse()?;
        let min2 = c.get(4).context("Missing group?")?.as_str().parse()?;
        let max2 = c.get(5).context("Missing group?")?.as_str().parse()?;

        Ok(FieldRule {
            range1: (min1, max1),
            range2: (min2, max2),
            name,
        })
    }
}

impl FieldRule {
    fn is_valid(&self, num: i32) -> bool {
        (num >= self.range1.0 && num <= self.range1.1)
            || (num >= self.range2.0 && num <= self.range2.1)
    }
}

fn is_field_valid(num: i32, rules: &HashMap<String, FieldRule>) -> bool {
    for rule in rules.values() {
        if rule.is_valid(num) {
            return true;
        }
    }
    false
}

fn is_ticket_valid(ticket: &[i32], rules: &HashMap<String, FieldRule>) -> bool {
    for num in ticket {
        let mut found_field = false;
        for rule in rules.values() {
            if rule.is_valid(*num) {
                found_field = true;
                break;
            }
        }
        if !found_field {
            return false;
        }
    }
    true
}

struct Input {
    fields: HashMap<String, FieldRule>,
    your_ticket: Vec<i32>,
    other_tickets: Vec<Vec<i32>>,
}

impl Input {
    fn load(file_name: &str) -> Result<Input> {
        let mut fields = HashMap::new();
        let mut other_tickets = vec![];

        let mut lines = crate::read_file(file_name)?;
        loop {
            let line = lines.next().context("Missing line")??;
            let line = line.trim();
            if line.is_empty() {
                break;
            }
            let rule: FieldRule = line.parse()?;
            fields.insert(rule.name.to_owned(), rule);
        }

        lines.next().context("Missing label line")??; // Discard this

        let your_ticket = lines
            .next()
            .context("Missing line?")??
            .split(',')
            .map(|n| n.parse().context("Bad number"))
            .collect::<Result<Vec<i32>>>()?;

        lines.next().context("Missing blank line")??; // Discard this
        lines.next().context("Missing label line")??; // Discard this

        for l in lines {
            let line = l?;
            other_tickets.push(
                line.split(',')
                    .map(|n| n.parse().context("Bad number"))
                    .collect::<Result<Vec<i32>>>()?,
            );
        }

        Ok(Input {
            fields,
            your_ticket,
            other_tickets,
        })
    }
}

fn simplify_uniques(possible_rules: &mut Vec<HashSet<&FieldRule>>) {
    let mut handled = HashSet::new();
    loop {
        let mut target_index = 0;
        let mut found = false;

        for (idx, set) in possible_rules.iter().enumerate() {
            if set.len() == 1 && !handled.contains(&idx) {
                // println!("Simplifying field {}", idx);
                target_index = idx;
                found = true;
                handled.insert(idx);
                break;
            }
        }
        if !found {
            return;
        }
        let rule = possible_rules[target_index]
            .iter()
            .next()
            .unwrap()
            .to_owned();
        // println!("\tSimplifying {} for field {}", rule.name, target_index);
        for (idx, set) in possible_rules.iter_mut().enumerate() {
            if idx != target_index {
                set.remove(&rule);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::bail;
    use std::{collections::HashSet, vec};

    #[test]
    fn day16_smoke1() -> Result<()> {
        let input = Input::load("day16_smoke.txt")?;

        assert!(is_ticket_valid(&input.your_ticket, &input.fields));
        let mut result = 0;

        for t in input.other_tickets {
            for n in t {
                if !is_field_valid(n, &input.fields) {
                    result += n;
                }
            }
        }
        assert_eq!(71, result);

        Ok(())
    }

    #[test]
    fn day16_1() -> Result<()> {
        let input = Input::load("day16.txt")?;

        assert!(is_ticket_valid(&input.your_ticket, &input.fields));
        let mut result = 0;

        for t in input.other_tickets {
            for n in t {
                if !is_field_valid(n, &input.fields) {
                    result += n;
                }
            }
        }
        println!("Day 16.1: {}", result);
        assert_eq!(25916, result);
        Ok(())
    }

    #[test]
    fn day16_2() -> Result<()> {
        let input = Input::load("day16.txt")?;

        assert!(is_ticket_valid(&input.your_ticket, &input.fields));

        let valid_tickets: Vec<Vec<i32>> = input
            .other_tickets
            .iter()
            .filter(|t| is_ticket_valid(t, &input.fields))
            .map(|t| t.to_owned())
            .collect();

        let mut possible_fields: Vec<HashSet<&FieldRule>> = vec![];
        for _ in 0..input.fields.len() {
            let mut set = HashSet::new();
            for k in input.fields.values() {
                set.insert(k);
            }
            possible_fields.push(set);
        }

        // for (idx, possible) in possible_fields.iter().enumerate() {
        //     println!("Field {} = {:?}", idx, possible);
        // }
        // println!();
        for (_ticket_num, t) in valid_tickets.iter().enumerate() {
            // println!("Ticket {}", ticket_num);
            for (idx, num) in t.iter().enumerate() {
                let mut to_remove = vec![];
                if possible_fields[idx].is_empty() {
                    bail!("No valid fields found!");
                }
                for rule in &possible_fields[idx] {
                    if !rule.is_valid(*num) {
                        // if ticket_num == 4 {
                        //     println!("\tRemoving rule {} for field {} and value {}", rule.name, idx, num);
                        // }
                        to_remove.push(*rule);
                    }
                }
                for rule in to_remove {
                    possible_fields[idx].remove(rule);
                }
            }
            simplify_uniques(&mut possible_fields);
            // for (idx, possible) in possible_fields.iter().enumerate() {
            //     println!("Field {} = {:?}", idx, possible);
            // }
            // println!();
        }

        // for (idx, possible) in possible_fields.iter().enumerate() {
        //     println!("Field {} = {:?}", idx, possible);
        // }

        let fields: Vec<&&FieldRule> = possible_fields
            .iter()
            .map(|s| s.iter().next().unwrap())
            .collect();
        let mut result: i64 = 1;
        for (idx, field) in fields.iter().enumerate() {
            if field.name.starts_with("departure") {
                result *= input.your_ticket[idx] as i64;
            }
        }

        println!("Day 16.2: {}", result);
        assert_eq!(2564529489989, result);

        Ok(())
    }
}

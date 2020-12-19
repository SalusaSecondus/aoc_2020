use std::collections::HashMap;

use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

fn load_day19_file(file_name: &str) -> Result<(HashMap<u32, String>, Vec<String>)> {
    lazy_static! {
        static ref RULE_RE: Regex = Regex::new(r"^(\d+): (.*)$").unwrap();
    }
    let mut raw_lines = HashMap::new();
    let mut messages = vec![];

    for line in crate::read_file(file_name)? {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        } else if let Some(captures) = RULE_RE.captures(line) {
            let rule_num = captures
                .get(1)
                .context("Missing number")?
                .as_str()
                .parse()?;
            let content = captures.get(2).context("Missing content")?.as_str();
            raw_lines.insert(rule_num, content.to_owned());
        } else {
            messages.push(line.to_owned());
        }
    }

    Ok((raw_lines, messages))
}

fn build_single_rule(rule_num: u32, raw: &HashMap<u32, String>) -> Result<Regex> {
    let mut pre_parsed = HashMap::new();
    let mut result = String::new();
    result += "^";
    result += &build_message_rule(rule_num, raw, &mut pre_parsed)?;
    result += "$";

    Regex::new(&result).context("Invalid regex")
}

// TODO: Use lifetimes to avoid extra copies
fn build_message_rule(
    rule_num: u32,
    raw: &HashMap<u32, String>,
    pre_parsed: &mut HashMap<u32, String>,
) -> Result<String> {
    lazy_static! {
        // static ref RULE_RE : Regex = Regex::new(r"^(\d+): (.*)$").unwrap();
        static ref LITERAL_RE : Regex = Regex::new("\"(.)\"").unwrap();
    }

    if let Some(result) = pre_parsed.get(&rule_num) {
        return Ok(result.to_owned());
    }

    let contents = raw.get(&rule_num).context("Missing raw rule")?;

    if let Some(literal_captures) = LITERAL_RE.captures(contents) {
        let result = literal_captures.get(1).context("Missing literal")?.as_str();
        pre_parsed.insert(rule_num, result.to_owned());
        Ok(result.to_owned())
    } else {
        let mut result = "(?:".to_owned();
        for element in contents.split(' ') {
            match element {
                "|" => result += element,
                _ => result += &build_message_rule(element.parse()?, raw, pre_parsed)?,
            };
        }
        result += ")";
        Ok(result)
    }
}

#[derive(Debug)]
enum Rule {
    Literal(char),
    Choice(Vec<u32>, Vec<u32>),
}

fn does_rule_match(rule_num: u32, s: &str, rules: &HashMap<u32, Rule>) -> Result<bool> {
    let chars: Vec<char> = s.chars().collect();
    let consumed = rule_match_internal(rule_num, &chars, 0, rules, 0)?;
    Ok(consumed == s.len())
}

fn rule_match_internal(
    rule_num: u32,
    s: &[char],
    pos: usize,
    rules: &HashMap<u32, Rule>,
    depth: usize,
) -> Result<usize> {
    for _ in 0..depth + 1 {
        print!("\t");
    }
    println!("{}", rule_num);
    if pos >= s.len() {
        println!("!!Out of bounds for rule {} and position {}", rule_num, pos);
        return Ok(0);
    }
    let rule = rules.get(&rule_num).context("Invalid rule number")?;
    match rule {
        Rule::Literal(c) => {
            if *c == s[pos] {
                return Ok(1);
            } else {
                return Ok(0);
            }
        }

        Rule::Choice(v1, v2) => {
            let mut failed = false;
            let mut offset = 0;
            for r in v1.iter() {
                let consumed = rule_match_internal(*r, s, pos + offset, rules, depth + 1)?;
                if consumed == 0 {
                    failed = true;
                    break;
                } else {
                    offset += consumed;
                }
            }
            if !failed {
                return Ok(offset);
            }

            if !v2.is_empty() {
                for _ in 0..depth + 1 {
                    print!("\t");
                }
                println!("--");
                let mut failed = false;
                let mut offset = 0;
                for r in v2.iter() {
                    let consumed = rule_match_internal(*r, s, pos + offset, rules, depth + 1)?;
                    if consumed == 0 {
                        failed = true;
                        break;
                    } else {
                        offset += consumed;
                    }
                }
                if !failed {
                    return Ok(offset);
                }
            }
        }
    };
    Ok(0)
}

fn load_day19_rules(file_name: &str) -> Result<(HashMap<u32, Rule>, Vec<String>)> {
    lazy_static! {
        static ref RULE_RE: Regex = Regex::new(r"^(\d+): (.*)$").unwrap();
        static ref LITERAL_RE: Regex = Regex::new("\"(.)\"").unwrap();
    }
    let mut rules = HashMap::new();
    let mut messages = vec![];

    for line in crate::read_file(file_name)? {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        } else if let Some(captures) = RULE_RE.captures(line) {
            let rule_num = captures
                .get(1)
                .context("Missing number")?
                .as_str()
                .parse()?;
            let content = captures.get(2).context("Missing content")?.as_str();

            if let Some(l) = LITERAL_RE.captures(content) {
                rules.insert(
                    rule_num,
                    Rule::Literal(
                        l.get(1)
                            .context("Missing literal")?
                            .as_str()
                            .chars()
                            .next()
                            .unwrap(),
                    ),
                );
            } else {
                let mut vec1 = vec![];
                let mut vec2 = vec![];
                let mut curr_vec = &mut vec1;
                for part in content.split(' ') {
                    match part {
                        "|" => curr_vec = &mut vec2,
                        _ => curr_vec.push(part.parse()?),
                    };
                }
                rules.insert(rule_num, Rule::Choice(vec1, vec2));
            }
        } else {
            messages.push(line.to_owned());
        }
    }

    Ok((rules, messages))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day19_smoke1() -> Result<()> {
        let (raw_rules, messages) = load_day19_file("day19_smoke.txt")?;
        let rule_0 = build_single_rule(0, &raw_rules)?;

        println!("Day 19 smoke1: {}", rule_0);
        let mut count = 0;
        for m in messages {
            if rule_0.is_match(&m) {
                println!("Day 19 smoke1 match {}", m);
                count += 1;
            }
        }
        println!("Day 19 smoke1: {}", count);
        assert_eq!(2, count);
        Ok(())
    }

    #[test]
    fn day19_smoke1_1() -> Result<()> {
        let (rules, messages) = load_day19_rules("day19_smoke.txt")?;

        let mut count = 0;
        for m in messages {
            if does_rule_match(0, &m, &rules)? {
                println!("Day 19 smoke1.1 match {}", m);
                count += 1;
            }
        }
        println!("Day 19 smoke1.1: {}", count);
        assert_eq!(2, count);
        Ok(())
    }

    #[test]
    fn day19_exp() -> Result<()> {
        let a: Vec<char> = "a".chars().collect();
        let aa: Vec<char> = "aa".chars().collect();
        let b: Vec<char> = "b".chars().collect();

        let mut rules = HashMap::new();
        rules.insert(0, Rule::Literal('a'));
        rules.insert(1, Rule::Literal('b'));
        rules.insert(2, Rule::Choice(vec![0], vec![1]));
        assert_eq!(1, rule_match_internal(0, &a, 0, &rules, 0)?);
        assert_eq!(0, rule_match_internal(0, &b, 0, &rules, 0)?);

        assert_eq!(0, rule_match_internal(1, &a, 0, &rules, 0)?);
        assert_eq!(1, rule_match_internal(1, &b, 0, &rules, 0)?);

        assert_eq!(1, rule_match_internal(2, &a, 0, &rules, 0)?);
        assert_eq!(1, rule_match_internal(2, &aa, 0, &rules, 0)?);
        assert_eq!(1, rule_match_internal(2, &b, 0, &rules, 0)?);

        Ok(())
    }
    #[test]
    fn day19_1() -> Result<()> {
        let (raw_rules, messages) = load_day19_file("day19.txt")?;
        let rule_0 = build_single_rule(0, &raw_rules)?;

        // println!("Day 19.1: {}", rule_0);
        let mut count = 0;
        for m in messages {
            if rule_0.is_match(&m) {
                // println!("Day 19 smoke1 match {}", m);
                count += 1;
            }
        }
        println!("Day 19.1: {}", count);
        assert_eq!(241, count);
        Ok(())
    }

    #[test]
    fn day19_1_1() -> Result<()> {
        let (rules, messages) = load_day19_rules("day19.txt")?;

        let mut count = 0;
        for m in messages {
            if does_rule_match(0, &m, &rules)? {
                // println!("Day 19 smoke1 match {}", m);
                count += 1;
            }
        }
        println!("Day 19.1.1: {}", count);
        assert_eq!(241, count);
        Ok(())
    }

    #[test]
    fn day19_smoke2() -> Result<()> {
        let (mut rules, messages) = load_day19_rules("day19_smoke2.txt")?;

        let mut count = 0;
        for m in &messages {
            // println!("\tChecking {}", m);
            if does_rule_match(0, m, &rules)? {
                println!("Day 19 smoke2.1 match {}", m);
                count += 1;
            }
        }

        assert_eq!(3, count);

        rules.insert(8, Rule::Choice(vec![42], vec![42, 8]));
        rules.insert(11, Rule::Choice(vec![42, 31], vec![42, 11, 31]));
        println!("Rule 8: {:?}", rules.get(&8));
        println!("Rule 11: {:?}", rules.get(&11));

        let mut count = 0;
        for m in &messages {
            println!("\tChecking {}", m);
            if does_rule_match(0, m, &rules)? {
                println!("Day 19 smoke2.2 match {}", m);
                count += 1;
            }
        }

        assert_eq!(12, count);

        Ok(())
    }
}

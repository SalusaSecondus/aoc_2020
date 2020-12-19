use std::collections::{HashMap, VecDeque};

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

#[derive(Debug, Clone)]
enum RuleOutput {
    Terminal(char),
    Nonterminal(Vec<Vec<u32>>),
}

#[derive(Debug, Clone)]
struct RuleTable(HashMap<u32, RuleOutput>);

impl RuleTable {
    fn parse_file(file_name: &str) -> Result<(RuleTable, Vec<String>)> {
        lazy_static! {
            static ref RULE_RE: Regex = Regex::new(r"^(\d+): (.*)$").unwrap();
            static ref LITERAL_RE: Regex = Regex::new("\"(.)\"").unwrap();
        }
        let mut messages = vec![];
        let mut map: HashMap<u32, RuleOutput> = HashMap::new();

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
                    let c = l.get(1).unwrap().as_str().chars().next().unwrap();
                    map.insert(rule_num, RuleOutput::Terminal(c));
                } else {
                    let mut output_choices = vec![];
                    let mut working_target: Vec<u32> = vec![];

                    for token in content.split(' ') {
                        if token == "|" {
                            output_choices.push(working_target);
                            working_target = vec![];
                        } else {
                            working_target.push(token.parse()?);
                        }
                    }
                    if !working_target.is_empty() {
                        output_choices.push(working_target);
                    }
                    map.insert(rule_num, RuleOutput::Nonterminal(output_choices));
                }
            } else {
                messages.push(line.to_owned());
            }
        }

        Ok((RuleTable(map), messages))
    }

    fn is_match(&self, rule_num: u32, s: &str) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let RuleTable(rules) = self;

        // let mut position = 0
        let mut possibilities = VecDeque::new();
        possibilities.push_back((0 as usize, vec![rules.get(&rule_num).unwrap()]));

        while !possibilities.is_empty() {
            let mut state = possibilities.pop_front().unwrap();
            // println!("\t {:?}", state);
            if state.0 == s.len() && state.1.is_empty() {
                // if !possibilities.is_empty() {
                //     println!("\tAccepted with {} possibilities left.", possibilities.len());
                // }
                return true;
            } else if state.0 == s.len() || state.1.is_empty() {
                continue;
            }

            let next_symbol = state.1.pop().unwrap();
            match next_symbol {
                RuleOutput::Terminal(c) => {
                    if *c == chars[state.0] {
                        state.0 += 1;
                        possibilities.push_back(state);
                    }
                }
                RuleOutput::Nonterminal(vv) => {
                    for choice in vv {
                        let mut new_state = state.clone();
                        for s in choice.iter().rev() {
                            let rule_output = rules.get(s).unwrap();
                            new_state.1.push(rule_output);
                        }
                        possibilities.push_back(new_state);
                    }
                }
            };
        }

        false
    }
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
                // println!("Day 19 smoke1 match {}", m);
                count += 1;
            }
        }
        println!("Day 19 smoke1: {}", count);
        assert_eq!(2, count);
        Ok(())
    }

    #[test]
    fn day19_smoke1_1() -> Result<()> {
        let (rules, messages) = RuleTable::parse_file("day19_smoke.txt")?;

        let mut count = 0;
        for m in messages {
            // println!("Day 19 smoke1.1: {}", m);
            if rules.is_match(0, &m) {
                // println!("\tmatch");
                count += 1;
            }
        }
        println!("Day 19 smoke1.1: {}", count);
        assert_eq!(2, count);
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
        let (rules, messages) = RuleTable::parse_file("day19.txt")?;

        let mut count = 0;
        for m in messages {
            if rules.is_match(0, &m) {
                // println!("Day 19 smoke1 match {}", m);
                count += 1;
            }
        }
        println!("Day 19.1.1: {}", count);
        assert_eq!(241, count);
        Ok(())
    }

    fn apply_part2(rules: &mut RuleTable) {
        rules
            .0
            .insert(8, RuleOutput::Nonterminal(vec![vec![42], vec![42, 8]]));
        rules.0.insert(
            11,
            RuleOutput::Nonterminal(vec![vec![42, 31], vec![42, 11, 31]]),
        );
    }

    #[test]
    fn day19_smoke2() -> Result<()> {
        let (mut rules, messages) = RuleTable::parse_file("day19_smoke2.txt")?;

        let mut count = 0;
        for m in &messages {
            // println!("\tChecking {}", m);
            if rules.is_match(0, m) {
                // println!("Day 19 smoke2.1 match {}", m);
                count += 1;
            }
        }

        assert_eq!(3, count);

        apply_part2(&mut rules);

        let mut count = 0;
        for m in &messages {
            // println!("\tChecking {}", m);
            if rules.is_match(0, m) {
                // println!("Day 19 smoke2.1 match {}", m);
                count += 1;
            }
        }

        assert_eq!(12, count);

        Ok(())
    }

    #[test]
    fn day19_2() -> Result<()> {
        let (mut rules, messages) = RuleTable::parse_file("day19.txt")?;

        apply_part2(&mut rules);

        let mut count = 0;
        for m in messages {
            if rules.is_match(0, &m) {
                // println!("Day 19 smoke1 match {}", m);
                count += 1;
            }
        }
        println!("Day 19.2: {}", count);
        assert_eq!(424, count);
        Ok(())
    }
}

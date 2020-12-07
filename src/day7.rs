use crate::read_file;

use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, vec};

#[derive(Debug)]
struct Rule {
    color: String,
    contents: HashMap<String, u32>,
}

impl Rule {
    fn parse(line: &str) -> Rule {
        lazy_static! {
            static ref CONTAINER_RE: Regex = Regex::new(r"^(.*?) bags contain ").unwrap();
            static ref CONTENT_RE: Regex = Regex::new(r"(\d+) (.*?) bags?(?:,|\.)").unwrap();
        }
        let outer = CONTAINER_RE.captures(line).unwrap();
        let color = outer.get(1).unwrap().as_str().to_string();

        let mut contents = HashMap::new();

        for caps in CONTENT_RE.captures_iter(line) {
            contents.insert(
                caps.get(2).unwrap().as_str().to_string(),
                caps.get(1).unwrap().as_str().parse().unwrap(),
            );
        }
        Rule { color, contents }
    }
}

fn parse_rules(file_name: &str) -> HashMap<String, Rule> {
    let mut result = HashMap::new();
    for line in read_file(file_name) {
        let line = line.unwrap();
        let line = line.trim();

        let rule = Rule::parse(line);
        result.insert(rule.color.to_owned(), rule);
    }

    result
}

fn find_containers(rules: &HashMap<String, Rule>, target: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    loop {
        // println!("Looping with found {:?}", result);
        let mut tmp_vec = vec![];
        for rule in rules.values() {
            // println!("Checking rule for {}", rule.color);
            if rule.contents.contains_key(target) {
                tmp_vec.push(rule.color.to_owned());
            }
            for t in &result {
                // println!("\tChecking if {} has {}", rule.color, t);
                if rule.contents.contains_key(t) {
                    // println!("\t++It does!");
                    tmp_vec.push(rule.color.to_owned());
                }
            }
        }
        let mut found_new = false;
        for t in tmp_vec {
            // println!("Should I insert {}?", t);
            if !result.contains(&t) {
                result.push(t);
                found_new = true;
            }
        }
        if !found_new {
            break;
        }
    }

    result
}

fn count_bags(rules: &HashMap<String, Rule>, target: &str) -> u32 {
    let mut queue = vec![(target, 1)];
    let mut result = 0;
    while queue.len() > 0 {
        // println!("Count {}, Current queue: {:?}", result, queue);
        let current = queue.pop().unwrap();
        result += current.1;
        let rule = rules.get(current.0).unwrap();
        for e in &rule.contents {
            queue.push((e.0, e.1 * current.1));
        }
    }
    result - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7_smoke1() {
        let rules = parse_rules("day7_smoke.txt");
        // println!("Day7 smoke1 rules: {:?}", rules);
        let outermost = find_containers(&rules, "shiny gold");
        println!("Day7 smoke1: {:?}", outermost);
        assert_eq!(4, outermost.len());
    }

    #[test]
    fn day7_1() {
        let rules = parse_rules("day7.txt");
        // println!("Day7  rules: {:?}", rules);
        let outermost = find_containers(&rules, "shiny gold");
        println!("Day7.1: {}", outermost.len());
    }

    #[test]
    fn day7_smoke2() {
        let rules = parse_rules("day7_smoke2.txt");
        let size = count_bags(&rules, "shiny gold");
        assert_eq!(126, size);
    }

    #[test]
    fn day7_2() {
        let rules = parse_rules("day7.txt");
        let size = count_bags(&rules, "shiny gold");
        println!("Day7.2: {}", size);
    }
}

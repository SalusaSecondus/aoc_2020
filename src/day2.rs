use lazy_static::lazy_static;
use regex::Regex;

use crate::*;

struct PwPolicy {
    min: usize,
    max: usize,
    target: char,
}

impl PwPolicy {
    fn parse(text: &str) -> PwPolicy {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+) ([a-z])").unwrap();
        }
        let caps = RE.captures(text).unwrap();
        PwPolicy {
            min: caps.get(1).unwrap().as_str().parse().unwrap(),
            max: caps.get(2).unwrap().as_str().parse().unwrap(),
            target: caps.get(3).unwrap().as_str().chars().nth(0).unwrap(),
        }
    }

    fn is_valid(&self, pw: &str) -> bool {
        let mut count = 0;
        for c in pw.chars() {
            if c == self.target {
                count += 1;
            }
        }

        // println!("Min: {}, Max: {}, Target: {}, PW: {}, Count: {}", self.min, self.max, self.target, pw, count);
        return count >= self.min && count <= self.max;
    }

    fn is_valid2(&self, pw: &str) -> bool {
        let chars: Vec<char> = pw.chars().collect();
        let mut count = 0;

        if chars[self.min - 1] == self.target {
            count += 1;
        }
        if chars[self.max - 1] == self.target {
            count += 1;
        }
        count == 1
    }
}

fn load_data(file_name: &str) -> Vec<(PwPolicy, String)> {
    let mut result = vec![];
    for line in read_file(file_name) {
        let line_s = line.unwrap();
        let parts: Vec<&str> = line_s.split(":").collect();
        let policy = PwPolicy::parse(parts[0]);
        result.push((policy, parts[1].trim().to_owned()));
    }

    result
}

#[cfg(test)]
mod tests {
    use day2::{PwPolicy, load_data};

    use crate::*;

    #[test]
    fn day_2() {
        let input = load_data("day2.txt");
        let mut valid1 = 0;
        let mut valid2 = 0;
        for elem in input {
            if elem.0.is_valid(&elem.1) {
                valid1 += 1;
            }
            if elem.0.is_valid2(&elem.1) {
                valid2 += 1;
            }
        }
        println!("Day 2.1: {}", valid1);
        println!("Day 2.2: {}", valid2);
    }
}

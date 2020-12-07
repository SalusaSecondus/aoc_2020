use crate::read_file;
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

fn load_passports(file_name: &str) -> Result<Vec<HashMap<String, String>>> {
    let lines = read_file(file_name)?;
    let mut result = vec![];
    let mut curr = HashMap::new();

    for line in lines {
        let line = line?;
        let line = line.trim();
        if line.len() == 0 {
            if passport_valid(&curr) {
                result.push(curr);
            }
            curr = HashMap::new();
        } else {
            let parts = line.split(" ");
            for part in parts {
                let part = part.trim();
                if part.len() == 0 {
                    continue;
                }
                // println!("Part: >{}< ", part);
                curr.insert(part[0..3].to_owned(), part[4..].to_owned());
            }
        }
        if passport_valid(&curr) {
            result.push(curr);
            curr = HashMap::new();
        }
    }

    Ok(result)
}

fn passport_valid(passport: &HashMap<String, String>) -> bool {
    lazy_static! {
        static ref YEAR_RE: Regex = Regex::new(r"^\d\d\d\d$").unwrap();
        static ref HEIGHT_RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
        static ref HAIR_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        static ref EYE_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        static ref PID_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }
    let empty_string = String::new();
    let byr = passport.get("byr").unwrap_or(&empty_string);
    let iyr = passport.get("iyr").unwrap_or(&empty_string);
    let eyr = passport.get("eyr").unwrap_or(&empty_string);
    let hgt = passport.get("hgt").unwrap_or(&empty_string);
    let hcl = passport.get("hcl").unwrap_or(&empty_string);
    let ecl = passport.get("ecl").unwrap_or(&empty_string);
    let pid = passport.get("pid").unwrap_or(&empty_string);

    if !YEAR_RE.is_match(byr) {
        return false;
    } else {
        let byr: i32 = byr.parse().unwrap_or(0);
        if byr < 1920 || byr > 2002 {
            return false;
        }
    }
    if !YEAR_RE.is_match(iyr) {
        return false;
    } else {
        let iyr: i32 = iyr.parse().unwrap();
        if iyr < 2010 || iyr > 2020 {
            return false;
        }
    }
    if !YEAR_RE.is_match(eyr) {
        return false;
    } else {
        let eyr: i32 = eyr.parse().unwrap_or(0);
        if eyr < 2020 || eyr > 2030 {
            return false;
        }
    }
    let hgt = HEIGHT_RE.captures(hgt);
    if hgt.is_none() {
        return false;
    }
    if let Some(hgt) = hgt {
        let hgt_val: u32 = hgt
            .get(1)
            .context("Group not found")
            .and_then(|g| Ok(g.as_str()))
            .and_then(|s| s.parse().context("Could not parse"))
            .unwrap_or(0u32);
        let hgt_str = hgt
            .get(2)
            .context("Could not find unit")
            .and_then(|g| Ok(g.as_str()))
            .unwrap_or("fake_unit");
        match hgt_str {
            "cm" => {
                if hgt_val < 150 || hgt_val > 193 {
                    return false;
                }
            }
            "in" => {
                if hgt_val < 59 || hgt_val > 76 {
                    return false;
                }
            }
            _ => return false,
        }
    } else {
        return false;
    }

    if !HAIR_RE.is_match(hcl) {
        return false;
    }
    if !EYE_RE.is_match(ecl) {
        return false;
    }
    if !PID_RE.is_match(pid) {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4_1() -> Result<()> {
        let passports = load_passports("day4.txt")?;
        println!("Day4.2: {}", passports.len());

        Ok(())
    }
}

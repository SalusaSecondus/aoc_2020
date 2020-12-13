#![allow(dead_code)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;

use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

use anyhow::{Context, Result};

const FILE_BASE: &str = r"res\";

pub fn read_file(file_name: &str) -> Result<Lines<BufReader<File>>> {
    let input = File::open(FILE_BASE.to_owned() + file_name).context("Could not open file")?;
    let reader = BufReader::new(input);

    Ok(reader.lines())
}

fn load_numbers(file_name: &str) -> Result<Vec<i64>> {
    let mut result = vec![];

    for line in read_file(file_name)? {
        let line = line?;
        let line = line.trim();
        result.push(line.parse()?);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

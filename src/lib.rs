#![allow(dead_code)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

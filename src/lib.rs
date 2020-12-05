#![allow(dead_code)]

mod day1;
mod day2;
mod day3;
mod day4;

use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub fn read_file(file_name: &str) -> Lines<BufReader<File>> {
    let input = File::open(file_name).unwrap();
    let reader = BufReader::new(input);
    reader.lines()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

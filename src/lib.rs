mod day1;
mod day2;

use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

const FILE_BASE: &str = "c:\\Users\\rubin\\rust\\aoc_2020\\";

fn read_file(file_name: &str) -> Lines<BufReader<File>> {
    let input = File::open(FILE_BASE.to_owned() + file_name).unwrap();
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

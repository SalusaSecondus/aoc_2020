use std::collections::HashMap;

use crate::*;

#[derive(Debug)]
struct TreeMap {
    width: usize,
    height: usize,
    trees: HashMap<(usize, usize), char>,
}

impl TreeMap {
    fn read_file(file_name: &str) -> TreeMap {
        let lines = read_file(file_name);
        let mut width = 0;
        let mut height = 0;
        let mut trees = HashMap::new();
        for line in lines {
            width = 0;
            let line = line.unwrap();
            let line = line.trim();
            for unit in line.chars() {
                trees.insert((width, height), unit);
                width += 1;
            }
            height += 1;
        }
        TreeMap {
            width,
            height,
            trees,
        }
    }

    fn get_plant(&self, x: usize, y: usize) -> &char {
        self.trees.get(&(x % self.width, y)).unwrap_or(&' ')
    }

    fn count_trees(&self, x_diff: usize, y_diff: usize) -> u32 {
        let mut tree_count = 0;
        let mut x = 0;
        let mut y = 0;
        loop {
            tree_count += match self.get_plant(x, y) {
                '.' => 0,
                '#' => 1,
                _ => break,
            };
            x += x_diff;
            y += y_diff;
        }
        tree_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_smoke() {
        let map = TreeMap::read_file("day3_smoke.txt");

        println!("Day3 Smoke: {}", map.count_trees(3, 1));
    }

    #[test]
    fn day3_1() {
        let map = TreeMap::read_file("day3.txt");
        println!("Day3.1: {}", map.count_trees(3, 1));
    }

    #[test]
    fn day3_2() {
        let map = TreeMap::read_file("day3.txt");

        let mut product: u64 = 1;
        product *= map.count_trees(1, 1) as u64;
        product *= map.count_trees(3, 1) as u64;
        product *= map.count_trees(5, 1) as u64;
        product *= map.count_trees(7, 1) as u64;
        product *= map.count_trees(1, 2) as u64;
        println!("Day3.2: {}", product);
    }
}

use crate::read_file;

use anyhow::{bail, Result};
use std::cmp::min;
use std::fmt;

#[derive(PartialEq, Eq, Debug, Clone)]
enum SeatState {
    Floor,
    Empty,
    Occupied,
}

impl SeatState {
    fn parse(s: char) -> Result<Self> {
        Ok(match s {
            '.' => SeatState::Floor,
            'L' => SeatState::Empty,
            '#' => SeatState::Occupied,
            _ => bail!("Invalid code"),
        })
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Ferry {
    seats: Vec<Vec<SeatState>>,
    height: usize,
    width: usize,
}

impl fmt::Display for Ferry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.seats {
            for seat in row {
                match seat {
                    SeatState::Floor => write!(f, ".")?,
                    SeatState::Empty => write!(f, "L")?,
                    SeatState::Occupied => write!(f, "#")?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Ferry {
    fn load_file(file_name: &str) -> Result<Ferry> {
        let mut seats = vec![];
        let mut width = 0;
        for line in read_file(file_name)? {
            let mut row = vec![];
            let line = line?;
            let line = line.trim();
            for c in line.chars() {
                row.push(SeatState::parse(c)?);
            }
            width = row.len();
            seats.push(row);
        }

        let height = seats.len();
        Ok(Ferry {
            seats,
            height,
            width,
        })
    }

    fn count_occupied(&self) -> u32 {
        let mut result = 0;
        for row in &self.seats {
            for seat in row {
                result += match seat {
                    SeatState::Occupied => 1,
                    _ => 0,
                }
            }
        }
        result
    }

    fn get(&self, row: i32, col: i32) -> &SeatState {
        if row < 0 || col < 0 {
            return &SeatState::Floor;
        }
        let row = row as usize;
        let col = col as usize;
        if let Some(row) = self.seats.get(row) {
            return row.get(col).unwrap_or(&SeatState::Empty);
        } else {
            return &SeatState::Empty;
        }
    }

    fn get_context(&self, row: usize, col: usize) -> (&SeatState, u8) {
        let row = row as i32;
        let col = col as i32;

        let mut occupied = 0;
        let center = self.get(row, col);
        for x_diff in -1..2i32 {
            for y_diff in -1..2i32 {
                if x_diff == 0 && y_diff == 0 {
                    continue;
                }
                if self.get(row + x_diff, col + y_diff) == &SeatState::Occupied {
                    occupied += 1;
                }
            }
        }

        (center, occupied)
    }

    fn get_context2(&self, row: usize, col: usize) -> (&SeatState, u8) {
        let row = row as i32;
        let col = col as i32;

        let min_diff = min(self.height, self.width) as i32;
        let center = self.get(row, col);
        let mut occupied = 0;

        // North
        for diff in 1..self.height as i32 {
            match self.get(row - diff, col) {
                SeatState::Empty => break,
                SeatState::Occupied => {
                    occupied += 1;
                    // println!("North");
                    break;
                }
                SeatState::Floor => {}
            }
        }
        // South
        for diff in 1..self.height as i32 {
            match self.get(row + diff, col) {
                SeatState::Empty => break,
                SeatState::Occupied => {
                    occupied += 1;
                    // println!("South");

                    break;
                }
                SeatState::Floor => {}
            }
        }
        // East
        for diff in 1..self.width as i32 {
            match self.get(row, col + diff) {
                SeatState::Empty => break,
                SeatState::Occupied => {
                    occupied += 1;
                    // println!("East");

                    break;
                }
                SeatState::Floor => {}
            }
        }
        // West
        for diff in 1..self.width as i32 {
            match self.get(row, col - diff) {
                SeatState::Empty => break,
                SeatState::Occupied => {
                    occupied += 1;
                    // println!("West");

                    break;
                }
                SeatState::Floor => {}
            }
        }
        // North-East
        for diff in 1..min_diff {
            match self.get(row - diff, col + diff) {
                SeatState::Empty => break,
                SeatState::Occupied => {
                    occupied += 1;
                    // println!("North-East");

                    break;
                }
                SeatState::Floor => {}
            }
        }
        // South-East
        for diff in 1..min_diff {
            match self.get(row + diff, col + diff) {
                SeatState::Empty => break,
                SeatState::Occupied => {
                    occupied += 1;
                    // println!("South-East");

                    break;
                }
                SeatState::Floor => {}
            }
        }
        // South-West
        for diff in 1..min_diff {
            match self.get(row + diff, col - diff) {
                SeatState::Empty => break,
                SeatState::Occupied => {
                    occupied += 1;
                    // println!("South-West");

                    break;
                }
                SeatState::Floor => {}
            }
        }
        // North-West
        for diff in 1..min_diff {
            match self.get(row - diff, col - diff) {
                SeatState::Empty => break,
                SeatState::Occupied => {
                    occupied += 1;
                    // println!("North-West");

                    break;
                }
                SeatState::Floor => {}
            }
        }

        (center, occupied)
    }

    fn step(&self) -> Ferry {
        let mut next = vec![];

        for row in 0..self.height {
            let mut next_row = vec![];
            for col in 0..self.width {
                let ctx = self.get_context(row, col);
                let new_seat = match ctx {
                    (SeatState::Floor, _) => SeatState::Floor,
                    (SeatState::Empty, 0) => SeatState::Occupied,
                    (SeatState::Occupied, count) => {
                        if count >= 4 {
                            SeatState::Empty
                        } else {
                            SeatState::Occupied
                        }
                    }
                    _ => ctx.0.clone(),
                };
                next_row.push(new_seat);
            }
            next.push(next_row);
        }

        Ferry {
            seats: next,
            height: self.height,
            width: self.width,
        }
    }

    fn step2(&self) -> Ferry {
        let mut next = vec![];

        for row in 0..self.height {
            let mut next_row = vec![];
            for col in 0..self.width {
                let ctx = self.get_context2(row, col);
                let new_seat = match ctx {
                    (SeatState::Floor, _) => SeatState::Floor,
                    (SeatState::Empty, 0) => SeatState::Occupied,
                    (SeatState::Occupied, count) => {
                        if count >= 5 {
                            SeatState::Empty
                        } else {
                            SeatState::Occupied
                        }
                    }
                    _ => ctx.0.clone(),
                };
                next_row.push(new_seat);
            }
            next.push(next_row);
        }

        Ferry {
            seats: next,
            height: self.height,
            width: self.width,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Ferry;
    use anyhow::Result;

    #[test]
    fn day11_smoke1() -> Result<()> {
        let mut prev = Ferry::load_file("day11_smoke.txt")?;
        loop {
            println!("Day11 smoke1 \n{}\n", prev);
            let next = prev.step();
            if next == prev {
                break;
            }
            prev = next;
        }

        assert_eq!(prev.count_occupied(), 37);
        Ok(())
    }

    #[test]
    fn day11_1() -> Result<()> {
        let mut prev = Ferry::load_file("day11.txt")?;
        loop {
            // println!("Day11 smoke1 \n{}\n", prev);
            let next = prev.step();
            if next == prev {
                break;
            }
            prev = next;
        }

        println!("Day 11.1: {}", prev.count_occupied());
        Ok(())
    }

    #[test]
    fn day11_smoke2() -> Result<()> {
        let mut prev = Ferry::load_file("day11_smoke.txt")?;
        // println!("Day11 smoke2 ctx {:?}", prev.get_context2(1, 9));
        // prev = prev.step();
        // println!("Day11 smoke2 ctx {:?}", prev.get_context2(1, 9));

        loop {
            println!("Day11 smoke2 ctx {:?}", prev.get_context2(1, 9));
            println!("Day11 smoke2 \n{}", prev);
            let next = prev.step2();
            if next == prev {
                break;
            }
            prev = next;
        }

        assert_eq!(prev.count_occupied(), 26);
        Ok(())
    }

    #[test]
    fn day11_2() -> Result<()> {
        let mut prev = Ferry::load_file("day11.txt")?;
        loop {
            // println!("Day11 smoke1 \n{}\n", prev);
            let next = prev.step2();
            if next == prev {
                break;
            }
            prev = next;
        }

        println!("Day 11.2: {}", prev.count_occupied());
        Ok(())
    }
}

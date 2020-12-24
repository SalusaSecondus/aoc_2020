use anyhow::{bail, Context, Result};
use std::{fmt::Display, str::FromStr};

struct Cups {
    cups: [u8; 9],
    current_cup: u8,
}

impl Cups {
    fn answer(&self) -> String {
        let mut result = String::new();
        let mut idx = 0;
        while self.cups[idx] != 1 {
            // println!("Advancing: {}", idx);
            self.advance_to_next(&mut idx, true);
        }
        self.advance_to_next(&mut idx, true);
        while self.cups[idx] != 1 {
            result += &self.cups[idx].to_string();
            self.advance_to_next(&mut idx, true);
        }
        result
    }

    fn turn(&mut self) -> Result<()> {
        // The crab picks up the three cups that are immediately clockwise of the current cup.
        // They are removed from the circle; cup spacing is adjusted as necessary to maintain the circle.
        let hand = self.three_after_current()?;

        // println!("pick up: {:?}", hand);
        // The crab selects a destination cup: the cup with a label equal to the current cup's label minus one.
        // If this would select one of the cups that was just picked up,
        // the crab will keep subtracting one until it finds a cup that wasn't just picked up.
        // If at any point in this process the value goes below the lowest value on any cup's label,
        // it wraps around to the highest value on any cup's label instead.
        let destination_idx = self.calculate_destination()?;
        // println!(
        //     "destination: {} ({})",
        //     self.cups[destination_idx], destination_idx
        // );
        // println!("\tPre Move: {:?}", self.cups);

        // The crab places the cups it just picked up so that they are immediately clockwise of the destination cup.
        // They keep the same order as when they were picked up
        self.move_pieces(destination_idx, hand)?;
        // println!("\tPost Move:{:?}", self.cups);

        // self.insert_hand(destination_idx, hand)?;

        self.next_current()?;

        Ok(())
    }

    fn next_current(&mut self) -> Result<()> {
        let mut current_idx = Option::None;
        for (idx, t) in self.cups.iter().enumerate() {
            if t == &self.current_cup {
                current_idx = Some(idx);
            }
        }

        let mut current_idx = current_idx.context("No current?")?;
        self.advance_to_next(&mut current_idx, true);
        self.current_cup = self.cups[current_idx];

        Ok(())
    }

    fn insert_hand(&mut self, destination_idx: usize, hand: [u8; 3]) -> Result<()> {
        let mut destination_idx = destination_idx;
        self.advance_to_next(&mut destination_idx, false);
        for cup in &hand {
            self.cups[destination_idx] = *cup;
            self.advance_to_next(&mut destination_idx, false);
        }

        Ok(())
    }

    fn move_pieces(&mut self, destination_idx: usize, hand: [u8; 3]) -> Result<()> {
        let mut starting_idx = Option::None;
        for (idx, t) in self.cups.iter().enumerate() {
            if t == &self.current_cup {
                starting_idx = Some(idx);
            }
        }

        let mut insert_idx = starting_idx.context("No current cup")?;
        self.advance_to_next(&mut insert_idx, false);
        let mut pull_idx = insert_idx + 3;
        if pull_idx >= self.cups.len() {
            pull_idx = 0;
        }

        while self.cups[pull_idx] == 0 {
            pull_idx += 1;
            if pull_idx >= self.cups.len() {
                pull_idx = 0;
            }
        }

        let destination_value = self.cups[destination_idx];
        loop {
            self.cups[insert_idx] = self.cups[pull_idx];
            self.cups[pull_idx] = 0;
            if self.cups[insert_idx] == destination_value {
                break;
            }
            self.advance_to_next(&mut insert_idx, false);
            self.advance_to_next(&mut pull_idx, true);
        }
        // println!("\tIntraMove:{:?}", self.cups);
        for cup in &hand {
            self.advance_to_next(&mut insert_idx, false);
            self.cups[insert_idx] = *cup;
        }
        Ok(())
    }

    fn advance_to_next(&self, idx: &mut usize, skip_free: bool) {
        *idx += 1;
        if *idx >= self.cups.len() {
            *idx = 0;
        }

        if skip_free {
            while self.cups[*idx] == 0 {
                // println!("Idx1: {}", idx);
                *idx += 1;
                if *idx >= self.cups.len() {
                    *idx = 0;
                }
            }
        }
    }

    fn calculate_destination(&self) -> Result<usize> {
        let mut destination = self.current_cup;

        loop {
            destination -= 1;
            if destination == 0 {
                destination = 9;
            }
            for (idx, x) in self.cups.iter().enumerate() {
                if x == &destination {
                    return Ok(idx);
                }
            }
        }

        // bail!("No destination found");
    }

    fn three_after_current(&mut self) -> Result<[u8; 3]> {
        let mut hand = [0u8; 3];
        let mut starting_idx = Option::None;
        for (idx, t) in self.cups.iter().enumerate() {
            if t == &self.current_cup {
                starting_idx = Some(idx);
            }
        }

        let mut pull_idx = starting_idx.context("No current cup")? + 1;

        let mut hand_idx = 0;
        while hand_idx < 3 {
            if pull_idx == self.cups.len() {
                pull_idx = 0;
            }
            if self.cups[pull_idx] != 0 {
                hand[hand_idx] = self.cups[pull_idx];
                self.cups[pull_idx] = 0;
                hand_idx += 1;
            }
            pull_idx += 1;
        }
        Ok(hand)
    }
}

impl FromStr for Cups {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cups = [0u8; 9];
        if s.len() > 9 {
            bail!("Too many cups");
        }
        for (idx, c) in s.chars().enumerate() {
            cups[idx] = c as u8 - b'0';
        }

        let current_cup = cups[0];

        Ok(Cups { cups, current_cup })
    }
}

impl Display for Cups {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for c in &self.cups {
            if c == &self.current_cup {
                write!(f, " ({})", c)?;
            } else {
                write!(f, " {}", c)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day23_smoke() -> Result<()> {
        let mut cups: Cups = "389125467".parse()?;
        for turn in 1..11 {
            println!("-- move {} --", turn);
            println!("cups:{}", cups);
            cups.turn()?;
        }

        println!("--final --");
        println!("cups:{}", cups);
        assert_eq!("92658374", &cups.answer());

        let mut cups: Cups = "389125467".parse()?;
        for _ in 0..100 {
            cups.turn()?;
        }
        assert_eq!("67384529", &cups.answer());

        Ok(())
    }

    #[test]
    fn day23_1() -> Result<()> {
        let mut cups: Cups = "186524973".parse()?;
        for _ in 0..100 {
            cups.turn()?;
        }
        assert_eq!("67384529", &cups.answer());

        Ok(())
    }
}

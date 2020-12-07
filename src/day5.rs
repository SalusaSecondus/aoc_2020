use std::cmp::Ordering;

#[derive(Eq)]
struct BoardingPass {
    code: String,
    row: u8,
    col: u8,
    id: u16,
}

impl BoardingPass {
    fn parse(code: &str) -> BoardingPass {
        let code = code.trim().to_owned();
        assert_eq!(code.len(), 10);

        let mut row = 0;
        let mut col = 0;

        let mut chars = code.chars();
        for _ in 0..7 {
            row = row << 1;
            row += match chars.next().unwrap() {
                'B' => 1,
                'F' => 0,
                _ => panic!("Wrong code"),
            };
        }
        let row = row;
        for _ in 0..3 {
            col = col << 1;
            col += match chars.next().unwrap() {
                'R' => 1,
                'L' => 0,
                _ => panic!("Wrong code"),
            };
        }
        let col = col;
        let id = (row as u16) * 8 + (col as u16);
        BoardingPass { code, row, col, id }
    }
}

impl Ord for BoardingPass {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for BoardingPass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BoardingPass {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use crate::read_file;
    use std::collections::HashMap;

    #[test]
    fn day5_smoke() {
        let pass = BoardingPass::parse("FBFBBFFRLR");
        assert_eq!(44, pass.row);
        assert_eq!(5, pass.col);
        assert_eq!(357, pass.id);

        let pass = BoardingPass::parse("BFFFBBFRRR");
        assert_eq!(70, pass.row);
        assert_eq!(7, pass.col);
        assert_eq!(567, pass.id);
    }

    #[test]
    fn day5_1() -> Result<()> {
        let mut highest_id = 0;
        for line in read_file("day5.txt")? {
            let pass = BoardingPass::parse(&line.unwrap());
            if pass.id > highest_id {
                highest_id = pass.id;
            }
        }
        println!("Day5.1: {}", highest_id);

        Ok(())
    }

    #[test]
    fn day5_2() -> Result<()> {
        let mut lowest_id = 1000;
        let mut highest_id = 0;

        let mut passes = HashMap::new();
        for line in read_file("day5.txt")? {
            let pass = BoardingPass::parse(&line?);
            if pass.id < lowest_id {
                lowest_id = pass.id;
            }
            if pass.id > highest_id {
                highest_id = pass.id;
            }
            passes.insert(pass.id, pass);
        }

        for guess in (lowest_id + 1)..highest_id {
            if !passes.contains_key(&guess)
                && passes.contains_key(&(guess - 1))
                && passes.contains_key(&(guess + 1))
            {
                println!("Day5.2: {}", guess);
            }
        }

        Ok(())
    }
}

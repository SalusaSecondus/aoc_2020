use std::str::FromStr;

use anyhow::{bail, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn rotate(&self, deg: i32) -> Result<Direction> {
        let mut deg = deg;
        if deg < 0 {
            deg += 360;
        }

        Ok(match deg {
            0 => self.clone(),
            360 => self.clone(),
            90 => match self {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::East => Direction::South,
                Direction::West => Direction::North,
            },
            180 => match self {
                Direction::North => Direction::South,
                Direction::South => Direction::North,
                Direction::East => Direction::West,
                Direction::West => Direction::East,
            },
            270 => match self {
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
                Direction::West => Direction::South,
            },
            _ => bail!("Invalid rotation"),
        })
    }
}
#[derive(Debug, Clone)]
enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([NSEWLRF])(\d+)").unwrap();
        }
        let s = s.trim();
        let c = RE.captures(s).context("Invalid entry")?;
        let num = c
            .get(2)
            .context("Missing number")?
            .as_str()
            .parse()
            .context("Invalid number")?;

        let command = c.get(1).context("Missing command")?.as_str();
        Ok(match command {
            "N" => Instruction::North(num),
            "S" => Instruction::South(num),
            "E" => Instruction::East(num),
            "W" => Instruction::West(num),
            "L" => Instruction::Left(num),
            "R" => Instruction::Right(num),
            "F" => Instruction::Forward(num),
            _ => bail!("Invalid command"),
        })
    }
}

fn load_route(file_name: &str) -> Result<Vec<Instruction>> {
    let mut result = vec![];
    for line in crate::read_file(file_name)? {
        let line = line?;
        let line = line.trim();
        result.push(line.parse()?);
    }

    Ok(result)
}

#[derive(Debug, Clone)]
struct Ship {
    direction: Direction,
    east: i32,
    north: i32,
    waypoint_east: i32,
    waypoint_north: i32,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            direction: Direction::East,
            north: 0,
            east: 0,
            waypoint_north: 1,
            waypoint_east: 10,
        }
    }

    fn step(&mut self, instruction: &Instruction) -> Result<()> {
        match instruction {
            Instruction::North(num) => self.north += num,
            Instruction::South(num) => self.north -= num,
            Instruction::East(num) => self.east += num,
            Instruction::West(num) => self.east -= num,
            Instruction::Forward(num) => self.move_forward(*num)?,
            Instruction::Left(num) => self.direction = self.direction.rotate(-*num)?,
            Instruction::Right(num) => self.direction = self.direction.rotate(*num)?,
        }
        Ok(())
    }

    fn step2(&mut self, instruction: &Instruction) -> Result<()> {
        match instruction {
            Instruction::North(num) => self.waypoint_north += num,
            Instruction::South(num) => self.waypoint_north -= num,
            Instruction::East(num) => self.waypoint_east += num,
            Instruction::West(num) => self.waypoint_east -= num,
            Instruction::Forward(num) => self.move_towards_waypoint(*num)?,
            Instruction::Left(num) => self.rotate_waypoint(-*num)?,
            Instruction::Right(num) => self.rotate_waypoint(*num)?,
        }
        Ok(())
    }

    fn move_towards_waypoint(&mut self, num: i32) -> Result<()> {
        self.north += num * self.waypoint_north;
        self.east += num * self.waypoint_east;

        Ok(())
    }

    fn rotate_waypoint(&mut self, deg: i32) -> Result<()> {
        let mut deg = deg;
        if deg < 0 {
            deg += 360;
        }

        match deg {
            0 => {}
            360 => {}
            90 => {
                let tmp_east = self.waypoint_east;
                self.waypoint_east = self.waypoint_north;
                self.waypoint_north = -tmp_east
            }
            270 => {
                let tmp_east = self.waypoint_east;
                self.waypoint_east = -self.waypoint_north;
                self.waypoint_north = tmp_east
            }
            180 => {
                self.waypoint_north = -self.waypoint_north;
                self.waypoint_east = -self.waypoint_east;
            }
            _ => bail!("Invalid degree"),
        };

        Ok(())
    }
    fn move_forward(&mut self, num: i32) -> Result<()> {
        match self.direction {
            Direction::North => self.north += num,
            Direction::South => self.north -= num,
            Direction::East => self.east += num,
            Direction::West => self.east -= num,
        };
        Ok(())
    }

    fn execute_route(&mut self, route: &[Instruction]) -> Result<()> {
        for i in route {
            self.step(i)?;
            // println!("DEBUG: {:?}", self);
        }

        Ok(())
    }

    fn execute_route2(&mut self, route: &[Instruction]) -> Result<()> {
        for i in route {
            self.step2(i)?;
            // println!("DEBUG: {:?}", self);
        }

        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12_smoke1() -> Result<()> {
        let route = load_route("day12_smoke.txt")?;
        let mut ship = Ship::new();
        ship.execute_route(&route)?;
        assert_eq!(17, ship.east);
        assert_eq!(-8, ship.north);
        assert_eq!(Direction::South, ship.direction);

        Ok(())
    }

    #[test]
    fn day12_smoke2() -> Result<()> {
        let route = load_route("day12_smoke.txt")?;
        let mut ship = Ship::new();
        ship.execute_route2(&route)?;
        assert_eq!(214, ship.east);
        assert_eq!(-72, ship.north);
        assert_eq!(Direction::East, ship.direction);

        Ok(())
    }

    #[test]
    fn day12_1() -> Result<()> {
        let route = load_route("day12.txt")?;
        let mut ship = Ship::new();
        ship.execute_route(&route)?;
        println!("Day 12.1: {}", ship.east.abs() + ship.north.abs());

        Ok(())
    }

    #[test]
    fn day12_2() -> Result<()> {
        let route = load_route("day12.txt")?;
        let mut ship = Ship::new();
        ship.execute_route2(&route)?;
        println!("Day 12.2: {}", ship.east.abs() + ship.north.abs());

        Ok(())
    }
}

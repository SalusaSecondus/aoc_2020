use anyhow::{bail, Result};
use std::{collections::HashSet, str::Chars};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn origin() -> Coord {
        Coord { x: 0, y: 0 }
    }
}

enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {
    fn step(&self, coord: &Coord) -> Coord {
        let mut coord = *coord;

        match self {
            Direction::East => coord.x += 2,
            Direction::SouthEast => {
                coord.x += 1;
                coord.y -= 1;
            }
            Direction::SouthWest => {
                coord.x -= 1;
                coord.y -= 1;
            }
            Direction::West => coord.x -= 2,
            Direction::NorthWest => {
                coord.x -= 1;
                coord.y += 1;
            }
            Direction::NorthEast => {
                coord.x += 1;
                coord.y += 1;
            }
        }

        coord
    }

    fn next(chars: &mut Chars) -> Result<Option<Direction>> {
        if let Some(c) = chars.next() {
            let result = match c {
                'e' => Direction::East,
                'w' => Direction::West,
                'n' => {
                    if let Some(c) = chars.next() {
                        match c {
                            'e' => Direction::NorthEast,
                            'w' => Direction::NorthWest,
                            _ => bail!("Invalid second character"),
                        }
                    } else {
                        bail!("Missing second character");
                    }
                }
                's' => {
                    if let Some(c) = chars.next() {
                        match c {
                            'e' => Direction::SouthEast,
                            'w' => Direction::SouthWest,
                            _ => bail!("Invalid second character"),
                        }
                    } else {
                        bail!("Missing second character");
                    }
                }
                _ => bail!("Invalid first character"),
            };
            Ok(Option::Some(result))
        } else {
            Ok(Option::None)
        }
    }
}

fn follow_path(floor: &mut HashSet<Coord>, path: &str) -> Result<()> {
    let mut chars = path.chars();

    let mut location = Coord::origin();

    while let Some(d) = Direction::next(&mut chars)? {
        location = d.step(&location);
    }

    #[allow(clippy::map_entry)]
    if floor.contains(&location) {
        floor.remove(&location);
    } else {
        floor.insert(location);
    }
    Ok(())
}

fn generation(floor: &HashSet<Coord>) -> HashSet<Coord> {
    let mut result = HashSet::new();

    for coord in floor.iter().flat_map(|coord| neighborhood(coord)) {
        let old_value = floor.contains(&coord);
            let count = count_neighbors(floor, &coord);
            let new_value;
            if old_value {
                new_value = count == 1 || count == 2;
            } else {
                new_value = count == 2;
            }
            if new_value {
                result.insert(coord);
            }
    }

    
        result
}

fn count_neighbors(floor: &HashSet<Coord>, coord: &Coord) -> usize {
    let mut result = 0;
    for c in neighborhood(coord) {
        if coord != &c && floor.contains(&c) {
            result += 1;
        }
    }
    result
}

fn neighborhood(coord: &Coord) -> Vec<Coord> {
    let mut result = vec![];
    result.push(*coord);
    result.push(Direction::East.step(&coord));
    result.push(Direction::SouthEast.step(&coord));
    result.push(Direction::SouthWest.step(&coord));
    result.push(Direction::West.step(&coord));
    result.push(Direction::NorthWest.step(&coord));
    result.push(Direction::NorthEast.step(&coord));

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn foo() -> Result<()> {
    //     let mut floor = HashMap::new();
    //     follow_path(&mut floor, "esew")?;

    //     println!("Foo? {:?}", floor);
    //     Ok(())
    // }
    #[test]
    fn day24_smoke1() -> Result<()> {
        let mut floor = HashSet::new();

        for l in crate::read_file("day24_smoke.txt")? {
            follow_path(&mut floor, &l?)?;
        }

        println!("Foo: {:?}", floor);
        let count = floor.len();

        assert_eq!(10, count);
        Ok(())
    }

    #[test]
    fn day24_1() -> Result<()> {
        let mut floor = HashSet::new();

        for l in crate::read_file("day24.txt")? {
            follow_path(&mut floor, &l?)?;
        }

        let count = floor.len();

        assert_eq!(312, count);
        Ok(())
    }

    #[test]
    fn day24_smoke2() -> Result<()> {
        let mut floor = HashSet::new();

        for l in crate::read_file("day24_smoke.txt")? {
            follow_path(&mut floor, &l?)?;
        }

        // println!("Foo: {:?}", floor);
        let count = floor.len();
        assert_eq!(10, count);
        for _day in 1..11 {
            floor = generation(&floor);
            println!("Day {}: {}", _day, floor.len());
        }
        for _day in 11..101 {
            floor = generation(&floor);
            // println!("Day {}: {}", _day, floor.len());
        }
        assert_eq!(2208, floor.len());
        Ok(())
    }

    #[test]
    fn day24_2() -> Result<()> {
        let mut floor = HashSet::new();

        for l in crate::read_file("day24.txt")? {
            follow_path(&mut floor, &l?)?;
        }

        // println!("Foo: {:?}", floor);
        let count = floor.len();
        assert_eq!(312, count);
        for _day in 1..11 {
            floor = generation(&floor);
            // println!("Day {}: {}", _day, floor.len());
        }
        for _day in 11..101 {
            floor = generation(&floor);
            // println!("Day {}: {}", _day, floor.len());
        }
        assert_eq!(3733, floor.len());
        Ok(())
    }
}

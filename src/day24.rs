use anyhow::{bail, Result};
use core::cmp::max;
use std::{cmp::min, collections::HashSet, str::Chars};

type CoordLimits = ((i32, i32), (i32, i32));

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

fn limits(floor: &HashSet<Coord>) -> CoordLimits {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    for c in floor {
        min_x = min(min_x, c.x);
        max_x = max(max_x, c.x);
        min_y = min(min_y, c.y);
        max_y = max(max_y, c.y);
    }

    ((min_x, max_x), (min_y, max_y))
}

fn generation(floor: &HashSet<Coord>, limits: &CoordLimits) -> (HashSet<Coord>, CoordLimits) {
    let mut result = HashSet::new();
    let ((min_x, max_x), (min_y, max_y)) = limits;

    let mut result_min_x = i32::MAX;
    let mut result_min_y = i32::MAX;
    let mut result_max_x = i32::MIN;
    let mut result_max_y = i32::MIN;

    for x in min_x - 2..max_x + 3 {
        for y in min_y - 1..max_y + 2 {
            let coord = Coord { x, y };
            let old_value = floor.contains(&coord);
            let count = count_neighbors(floor, &coord);
            let new_value;
            if old_value {
                new_value = count == 1 || count == 2;
            } else {
                new_value = count == 2;
            }
            if new_value {
                result_min_x = min(result_min_x, x);
                result_max_x = max(result_max_x, x);
                result_min_y = min(result_min_y, y);
                result_max_y = max(result_max_y, y);
                result.insert(coord);
            }
        }
    }

    (
        result,
        ((result_min_x, result_max_x), (result_min_y, result_max_y)),
    )
}

fn count_neighbors(floor: &HashSet<Coord>, coord: &Coord) -> usize {
    let mut result = 0;
    for c in neighbors(coord) {
        if floor.contains(&c) {
            result += 1;
        }
    }
    result
}

fn neighbors(coord: &Coord) -> Vec<Coord> {
    let mut result = vec![];
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
        let mut limit = limits(&floor);
        assert_eq!(10, count);
        for _day in 1..11 {
            let parts = generation(&floor, &limit);
            floor = parts.0;
            limit = parts.1;
            println!("Day {}: {}", _day, floor.len());
        }
        for _day in 11..101 {
            let parts = generation(&floor, &limit);
            floor = parts.0;
            limit = parts.1;
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
        let mut limit = limits(&floor);
        assert_eq!(312, count);
        for _day in 1..11 {
            let parts = generation(&floor, &limit);
            floor = parts.0;
            limit = parts.1;
            // println!("Day {}: {}", _day, floor.len());
        }
        for _day in 11..101 {
            let parts = generation(&floor, &limit);
            floor = parts.0;
            limit = parts.1;
            // println!("Day {}: {}", _day, floor.len());
        }
        assert_eq!(3733, floor.len());
        Ok(())
    }
}

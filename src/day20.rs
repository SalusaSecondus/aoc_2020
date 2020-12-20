use anyhow::{bail, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};
use std::{collections::HashMap, fmt::Display};

type PixelGrid = [[bool; 10]; 10];
#[derive(Debug, PartialEq, Eq, Clone)]
struct Tile {
    id: i32,
    pixels: PixelGrid,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Transformation {
    Rotate(u8), // 0-3. Clockwise
    RotateMirror(u8),
}

#[derive(Debug)]
enum Side {
    North,
    East,
    South,
    West,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Tile {}:", self.id)?;
        for y in 0..10 {
            for x in 0..10 {
                let icon = if self.pixels[x][y] { '#' } else { '.' };
                write!(f, "{}", icon)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Tile {
    #[allow(clippy::needless_range_loop)]
    fn transform(&self, t: &Transformation) -> Tile {
        let mut mirrored = self.pixels;

        if let Transformation::RotateMirror(_) = t {
            for x in 0..10 {
                for y in 0..10 {
                    mirrored[x][y] = self.pixels[9 - x][y];
                }
            }
        };

        let mut rotated = mirrored;

        let angle;
        if let Transformation::Rotate(a) = t {
            angle = *a;
        } else if let Transformation::RotateMirror(a) = t {
            angle = *a;
        } else {
            angle = 0;
        }

        for x in 0..10 {
            for y in 0..10 {
                let (rx, ry) = rotate_coords(angle, (x, y), 10);
                rotated[rx as usize][ry as usize] = mirrored[x as usize][y as usize];
            }
        }

        Tile {
            pixels: rotated,
            ..*self
        }
    }

    fn find_match(&self, others: &HashMap<i32, Tile>, side: &Side) -> Option<Tile> {
        for tile in others.values() {
            if let Some(t) = self.match_edge(tile, side) {
                return Option::Some(t);
            }
        }
        Option::None
    }
    fn match_edge(&self, other: &Tile, side: &Side) -> Option<Tile> {
        let mutations = [
            Transformation::Rotate(0),
            Transformation::Rotate(1),
            Transformation::Rotate(2),
            Transformation::Rotate(3),
            Transformation::RotateMirror(0),
            Transformation::RotateMirror(1),
            Transformation::RotateMirror(2),
            Transformation::RotateMirror(3),
        ];
        for t in mutations.iter() {
            let transformed = other.transform(t);

            let mut found = true;
            for idx in 0..10 {
                found &= match side {
                    Side::North => self.pixels[idx][0] == transformed.pixels[idx][9],
                    Side::East => self.pixels[9][idx] == transformed.pixels[0][idx],
                    Side::South => self.pixels[idx][9] == transformed.pixels[idx][0],
                    Side::West => self.pixels[0][idx] == transformed.pixels[9][idx],
                };
            }
            if found {
                return Option::Some(transformed);
            }
        }
        Option::None
    }
}

fn rotate_coords(angle: u8, coord: (i32, i32), scale: i32) -> (i32, i32) {
    let mut coord = coord;

    for _ in 0..angle {
        coord = (scale - coord.1 - 1, coord.0);
    }

    coord
}

fn load_tiles(file_name: &str) -> Result<HashMap<i32, Tile>> {
    lazy_static! {
        static ref ID_RE: Regex = Regex::new(r"^Tile (\d+):").unwrap();
    }
    let mut result = HashMap::new();

    let mut lines = crate::read_file(file_name)?;

    while let Some(line) = lines.next() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(c) = ID_RE.captures(line) {
            let id: i32 = c.get(1).unwrap().as_str().parse()?;
            let mut pixels: PixelGrid = [[false; 10]; 10];
            for y in 0..10 {
                if let Some(line) = lines.next() {
                    for (x, value) in line?.chars().enumerate() {
                        if x > 9 {
                            continue;
                        }
                        pixels[x][y] = match value {
                            '#' => true,
                            '.' => false,
                            _ => bail!("Invalid character"),
                        };
                    }
                }
            }
            result.insert(id, Tile { id, pixels });
        }
    }

    Ok(result)
}

#[allow(clippy::type_complexity)]
fn assemble_image(
    tiles: &HashMap<i32, Tile>,
) -> (((i32, i32), (i32, i32)), HashMap<(i32, i32), Tile>) {
    let mut image = HashMap::new();

    let mut tiles = tiles.clone();

    let first_tile = *tiles.keys().next().unwrap();
    // println!("\tFirst tile: {}", first_tile);
    let first_tile = tiles.remove(&first_tile).unwrap();

    image.insert((0, 0), first_tile);
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    while !tiles.is_empty() {
        // println!("\tTiles left: {:?}",  tiles.keys());
        let mut made_progress = false;
        #[allow(clippy::mut_range_bound)]
        for x in min_x..max_x + 1 {
            for y in min_y..max_y + 1 {
                let mut north_neighbor = Option::None;
                let mut south_neighbor = Option::None;
                let mut east_neighbor = Option::None;
                let mut west_neighbor = Option::None;
                if let Some(center) = image.get(&(x, y)) {
                    // North
                    if !image.contains_key(&(x, y - 1)) {
                        north_neighbor = center.find_match(&tiles, &Side::North);
                    }
                    // South
                    if !image.contains_key(&(x, y + 1)) {
                        south_neighbor = center.find_match(&tiles, &Side::South);
                    }
                    // East
                    if !image.contains_key(&(x + 1, y)) {
                        east_neighbor = center.find_match(&tiles, &Side::East);
                    }
                    // West
                    if !image.contains_key(&(x - 1, y)) {
                        west_neighbor = center.find_match(&tiles, &Side::West);
                    }
                }
                if let Some(n) = north_neighbor {
                    tiles.remove(&n.id);
                    image.insert((x, y - 1), n);
                    min_y = min(min_y, y - 1);
                    made_progress = true;
                }
                if let Some(n) = south_neighbor {
                    tiles.remove(&n.id);
                    image.insert((x, y + 1), n);
                    max_y = max(max_y, y + 1);
                    made_progress = true;
                }
                if let Some(n) = east_neighbor {
                    tiles.remove(&n.id);
                    image.insert((x + 1, y), n);
                    max_x = max(max_x, x + 1);
                    made_progress = true;
                }
                if let Some(n) = west_neighbor {
                    tiles.remove(&n.id);
                    image.insert((x - 1, y), n);
                    min_x = min(min_x, x - 1);
                    made_progress = true;
                }
            }
        }
        if !made_progress {
            panic!("No progress made!");
        }
    }

    (((min_x, min_y), (max_x, max_y)), image)
}

#[allow(clippy::type_complexity)]
fn merge_image(
    image: &(((i32, i32), (i32, i32)), HashMap<(i32, i32), Tile>),
) -> HashMap<(i32, i32), char> {
    let (((min_x, min_y), (max_x, max_y)), image) = image;

    let mut merged = HashMap::new();

    for tile_y in *min_y..*max_y + 1 {
        for tile_x in *min_x..*max_x + 1 {
            let tile = image.get(&(tile_x, tile_y)).unwrap();
            let base_x = tile_x - min_x;
            let base_y = tile_y - min_y;
            for inner_x in 1..9 {
                for inner_y in 1..9 {
                    if tile.pixels[inner_x][inner_y] {
                        let offset_x = (8 * base_x - 1) + inner_x as i32;
                        let offset_y = (8 * base_y - 1) + inner_y as i32;
                        merged.insert((offset_x as i32, offset_y as i32), '#');
                    }
                }
            }
        }
    }

    merged
}

fn display_image(image: &HashMap<(i32, i32), char>, dim: i32) {
    for y in 0..8 * dim {
        for x in 0..8 * dim {
            print!("{}", image.get(&(x, y)).unwrap_or(&'.'));
        }
        println!();
    }
}

fn count_monsters(image: &HashMap<(i32, i32), char>, dim: i32) -> (usize, usize) {
    let monster = [
        (18, 0),
        (0, 1),
        (5, 1),
        (6, 1),
        (11, 1),
        (12, 1),
        (17, 1),
        (18, 1),
        (19, 1),
        (1, 2),
        (4, 2),
        (7, 2),
        (10, 2),
        (13, 2),
        (16, 2),
    ];

    let mut monster_count = 0;
    // Yeah, this is ugly that I've explicitly modeled this in one case and do it manually elsewhere
    for rotation in &[0u8, 1, 2, 3] {
        for mirror in &[false, true] {
            for y in 0..8 * dim {
                for x in 0..8 * dim {
                    let mut found = true;
                    for m_coords in &monster {
                        let (m_x, m_y) = m_coords;

                        let m_x = m_x + x;
                        let m_y = m_y + y;

                        let coord = transform_coords(&(m_x, m_y), *mirror, *rotation, 8 * dim);

                        if image.get(&coord).unwrap_or(&'.') == &'.' {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        monster_count += 1;
                    }
                }
            }
        }
    }

    let hash_count = image.values().filter(|c| c == &&'#').count();
    let hash_count = hash_count - monster_count * monster.len();
    (monster_count, hash_count)
}

fn transform_coords(coord: &(i32, i32), mirror: bool, angle: u8, scale: i32) -> (i32, i32) {
    let mut coord = *coord;

    if mirror {
        coord.0 = scale - coord.0;
    }

    let coord = rotate_coords(angle, coord, scale);

    (coord.0 as i32, coord.1 as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day20_smoke() -> Result<()> {
        let tiles = load_tiles("day20_smoke.txt")?;
        // for t in tiles.values() {
        //     println!("{}", t);
        // }

        let (((min_x, min_y), (max_x, max_y)), image) = assemble_image(&tiles);

        for y in min_y..max_y + 1 {
            for x in min_x..max_x + 1 {
                print!("\t{}", image.get(&(x, y)).unwrap().id);
            }
            println!();
        }

        let mut result: u64 = 1;
        result *= image.get(&(min_x, min_y)).unwrap().id as u64;
        result *= image.get(&(max_x, min_y)).unwrap().id as u64;
        result *= image.get(&(min_x, max_y)).unwrap().id as u64;
        result *= image.get(&(max_x, max_y)).unwrap().id as u64;

        assert_eq!(20899048083289, result);
        Ok(())
    }

    #[test]
    fn day20_1() -> Result<()> {
        let tiles = load_tiles("day20.txt")?;
        // for t in tiles.values() {
        //     println!("{}", t);
        // }

        let (((min_x, min_y), (max_x, max_y)), image) = assemble_image(&tiles);

        // for y in min_y .. max_y + 1{
        //     for x in min_x .. max_x+1 {
        //         print!("\t{}", image.get(&(x,y)).unwrap().id);
        //     }
        //     println!();
        // }

        let mut result: u64 = 1;
        result *= image.get(&(min_x, min_y)).unwrap().id as u64;
        result *= image.get(&(max_x, min_y)).unwrap().id as u64;
        result *= image.get(&(min_x, max_y)).unwrap().id as u64;
        result *= image.get(&(max_x, max_y)).unwrap().id as u64;

        println!("Day 20.1: {}", result);
        assert_eq!(2699020245973, result);
        Ok(())
    }

    #[test]
    fn day20_smoke2() -> Result<()> {
        let tiles = load_tiles("day20_smoke.txt")?;
        let intermediate_form = assemble_image(&tiles);
        let merged = merge_image(&intermediate_form);

        display_image(&merged, 3);

        let (((min_x, min_y), (max_x, max_y)), image) = intermediate_form;

        for y in min_y..max_y + 1 {
            for x in min_x..max_x + 1 {
                print!("\t{}", image.get(&(x, y)).unwrap().id);
            }
            println!();
        }

        let (monster_count, roughness) = count_monsters(&merged, 3);
        println!("Monsters: {}\tRoughness: {}", monster_count, roughness);
        assert_eq!(2, monster_count);
        assert_eq!(273, roughness);
        Ok(())
    }

    #[test]
    fn day20_2() -> Result<()> {
        let tiles = load_tiles("day20.txt")?;
        let intermediate_form = assemble_image(&tiles);
        let merged = merge_image(&intermediate_form);

        // display_image(&merged, 12);

        // let (((min_x, min_y), (max_x, max_y)), image) = intermediate_form;

        // for y in min_y .. max_y + 1{
        //     for x in min_x .. max_x+1 {
        //         print!("\t{}", image.get(&(x,y)).unwrap().id);
        //     }
        //     println!();
        // }

        let (monster_count, roughness) = count_monsters(&merged, 12);
        println!("Monsters: {}\tRoughness: {}", monster_count, roughness);
        assert_eq!(37, monster_count);
        assert_eq!(2012, roughness);
        Ok(())
    }
}

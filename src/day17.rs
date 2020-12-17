use std::{
    cmp::{max, min},
    collections::HashMap,
    fmt::Display,
};

use anyhow::{bail, Result};

type Coordinate = (i32, i32, i32);

#[derive(PartialEq, Eq)]
enum CubeState {
    Active,
    Inactive,
}

impl Display for CubeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            CubeState::Active => write!(f, "#"),
            CubeState::Inactive => write!(f, "."),
        }
    }
}

impl CubeState {
    fn from(c: char) -> Result<CubeState> {
        match c {
            '.' => Ok(CubeState::Inactive),
            '#' => Ok(CubeState::Active),
            _ => bail!("Invalid symbol"),
        }
    }
}

struct World {
    cubes: HashMap<Coordinate, CubeState>,
    x_limits: (i32, i32),
    y_limits: (i32, i32),
    z_limits: (i32, i32),
}

struct HyperWorld {
    cubes: HashMap<(i32, i32, i32, i32), CubeState>,
    x_limits: (i32, i32),
    y_limits: (i32, i32),
    z_limits: (i32, i32),
    w_limits: (i32, i32),
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for z in self.z_limits.0..self.z_limits.1 + 1 {
            writeln!(f, "z={}", z)?;
            for y in self.y_limits.0..self.y_limits.1 + 1 {
                for x in self.x_limits.0..self.x_limits.1 + 1 {
                    let cube = self.cubes.get(&(x, y, z)).unwrap_or(&CubeState::Inactive);
                    write!(f, "{}", cube)?;
                }
                writeln!(f)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for HyperWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for w in self.w_limits.0..self.w_limits.1 + 1 {
            for z in self.z_limits.0..self.z_limits.1 + 1 {
                writeln!(f, "z={}, w={}", z, w)?;
                for y in self.y_limits.0..self.y_limits.1 + 1 {
                    for x in self.x_limits.0..self.x_limits.1 + 1 {
                        let cube = self
                            .cubes
                            .get(&(x, y, z, w))
                            .unwrap_or(&CubeState::Inactive);
                        write!(f, "{}", cube)?;
                    }
                    writeln!(f)?;
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl World {
    fn load(file_name: &str) -> Result<World> {
        let mut cubes = HashMap::new();
        let mut x_limits = (0, 0);
        let mut y_limits = (0, 0);
        let z_limits = (0, 0);

        for (y, line) in crate::read_file(file_name)?.enumerate() {
            y_limits.1 = max(y as i32, y_limits.1);
            let line = line?;
            let line = line.trim();
            for (x, c) in line.chars().enumerate() {
                let coord = (x as i32, y as i32, 0);
                x_limits.1 = max(x as i32, x_limits.1);
                let cube = CubeState::from(c)?;
                cubes.insert(coord, cube);
            }
        }

        Ok(World {
            cubes,
            x_limits,
            y_limits,
            z_limits,
        })
    }

    fn step(&mut self) {
        let mut cubes = HashMap::new();
        let mut x_limits = (0, 0);
        let mut y_limits = (0, 0);
        let mut z_limits = (0, 0);

        for z in self.z_limits.0 - 1..self.z_limits.1 + 2 {
            for y in self.y_limits.0 - 1..self.y_limits.1 + 2 {
                for x in self.x_limits.0 - 1..self.x_limits.1 + 2 {
                    let coor = (x, y, z);
                    let neighbor_count = self.neighbor_count(&coor);
                    // println!("DEBUG: {:?} => {}", coor, neighbor_count);
                    let old_state = self.get_cube_coor(&coor);
                    let new_state = match old_state {
                        CubeState::Active => {
                            if neighbor_count == 2 || neighbor_count == 3 {
                                CubeState::Active
                            } else {
                                CubeState::Inactive
                            }
                        }
                        CubeState::Inactive => {
                            if neighbor_count == 3 {
                                CubeState::Active
                            } else {
                                CubeState::Inactive
                            }
                        }
                    };
                    if new_state == CubeState::Active {
                        x_limits.0 = min(x_limits.0, x);
                        x_limits.1 = max(x_limits.1, x);
                        y_limits.0 = min(y_limits.0, y);
                        y_limits.1 = max(y_limits.1, y);
                        z_limits.0 = min(z_limits.0, z);
                        z_limits.1 = max(z_limits.1, z);
                        cubes.insert(coor, new_state);
                    }
                }
            }
        }

        self.cubes = cubes;
        self.x_limits = x_limits;
        self.y_limits = y_limits;
        self.z_limits = z_limits;
    }

    fn get_cube(&self, x: i32, y: i32, z: i32) -> &CubeState {
        self.get_cube_coor(&(x, y, z))
    }

    fn get_cube_coor(&self, coor: &Coordinate) -> &CubeState {
        self.cubes.get(coor).unwrap_or(&CubeState::Inactive)
    }

    fn neighbor_count(&self, coor: &Coordinate) -> u8 {
        let mut result = 0;
        for z_diff in -1..2 {
            for y_diff in -1..2 {
                for x_diff in -1..2 {
                    if z_diff == 0 && y_diff == 0 && x_diff == 0 {
                        continue;
                    }
                    let cube = self.get_cube(coor.0 + x_diff, coor.1 + y_diff, coor.2 + z_diff);
                    result += match cube {
                        CubeState::Active => 1,
                        CubeState::Inactive => 0,
                    }
                }
            }
        }
        result
    }

    fn count_active(&self) -> u32 {
        let mut result = 0;
        for cube in self.cubes.values() {
            if cube == &CubeState::Active {
                result += 1;
            }
        }
        result
    }
}

impl HyperWorld {
    fn load(file_name: &str) -> Result<HyperWorld> {
        let mut cubes = HashMap::new();
        let mut x_limits = (0, 0);
        let mut y_limits = (0, 0);
        let z_limits = (0, 0);
        let w_limits = (0, 0);

        for (y, line) in crate::read_file(file_name)?.enumerate() {
            y_limits.1 = max(y as i32, y_limits.1);
            let line = line?;
            let line = line.trim();
            for (x, c) in line.chars().enumerate() {
                let coord = (x as i32, y as i32, 0, 0);
                x_limits.1 = max(x as i32, x_limits.1);
                let cube = CubeState::from(c)?;
                cubes.insert(coord, cube);
            }
        }

        Ok(HyperWorld {
            cubes,
            x_limits,
            y_limits,
            z_limits,
            w_limits,
        })
    }

    fn step(&mut self) {
        let mut cubes = HashMap::new();
        let mut x_limits = (0, 0);
        let mut y_limits = (0, 0);
        let mut z_limits = (0, 0);
        let mut w_limits = (0, 0);

        for w in self.w_limits.0 - 1..self.w_limits.1 + 2 {
            for z in self.z_limits.0 - 1..self.z_limits.1 + 2 {
                for y in self.y_limits.0 - 1..self.y_limits.1 + 2 {
                    for x in self.x_limits.0 - 1..self.x_limits.1 + 2 {
                        let coor = (x, y, z, w);
                        let neighbor_count = self.neighbor_count(&coor);
                        // println!("DEBUG: {:?} => {}", coor, neighbor_count);
                        let old_state = self.get_cube_coor(&coor);
                        let new_state = match old_state {
                            CubeState::Active => {
                                if neighbor_count == 2 || neighbor_count == 3 {
                                    CubeState::Active
                                } else {
                                    CubeState::Inactive
                                }
                            }
                            CubeState::Inactive => {
                                if neighbor_count == 3 {
                                    CubeState::Active
                                } else {
                                    CubeState::Inactive
                                }
                            }
                        };
                        if new_state == CubeState::Active {
                            x_limits.0 = min(x_limits.0, x);
                            x_limits.1 = max(x_limits.1, x);
                            y_limits.0 = min(y_limits.0, y);
                            y_limits.1 = max(y_limits.1, y);
                            z_limits.0 = min(z_limits.0, z);
                            z_limits.1 = max(z_limits.1, z);
                            w_limits.0 = min(w_limits.0, w);
                            w_limits.1 = max(w_limits.1, w);
                            cubes.insert(coor, new_state);
                        }
                    }
                }
            }
        }

        self.cubes = cubes;
        self.x_limits = x_limits;
        self.y_limits = y_limits;
        self.z_limits = z_limits;
        self.w_limits = w_limits;
    }

    fn get_cube(&self, x: i32, y: i32, z: i32, w: i32) -> &CubeState {
        self.get_cube_coor(&(x, y, z, w))
    }

    fn get_cube_coor(&self, coor: &(i32, i32, i32, i32)) -> &CubeState {
        self.cubes.get(coor).unwrap_or(&CubeState::Inactive)
    }

    fn neighbor_count(&self, coor: &(i32, i32, i32, i32)) -> u8 {
        let mut result = 0;
        for w_diff in -1..2 {
            for z_diff in -1..2 {
                for y_diff in -1..2 {
                    for x_diff in -1..2 {
                        if z_diff == 0 && y_diff == 0 && x_diff == 0 && w_diff == 0 {
                            continue;
                        }
                        let cube = self.get_cube(
                            coor.0 + x_diff,
                            coor.1 + y_diff,
                            coor.2 + z_diff,
                            coor.3 + w_diff,
                        );
                        result += match cube {
                            CubeState::Active => 1,
                            CubeState::Inactive => 0,
                        }
                    }
                }
            }
        }
        result
    }

    fn count_active(&self) -> u32 {
        let mut result = 0;
        for cube in self.cubes.values() {
            if cube == &CubeState::Active {
                result += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day17_smoke1() -> Result<()> {
        let mut world = World::load("day17_smoke.txt")?;
        println!("{}", world);
        for _cycle in 0..6 {
            world.step();
            // println!("After {} cycle(s)\n{}\n", cycle + 1, world);
        }

        assert_eq!(112, world.count_active());
        Ok(())
    }

    #[test]
    fn day17_1() -> Result<()> {
        let mut world = World::load("day17.txt")?;
        println!("{}", world);
        for _cycle in 0..6 {
            world.step();
            // println!("After {} cycle(s)\n{}\n", cycle + 1, world);
        }

        println!("Day 17.1: {}", world.count_active());
        Ok(())
    }

    #[test]
    fn day17_smoke2() -> Result<()> {
        let mut world = HyperWorld::load("day17_smoke.txt")?;
        println!("{}", world);
        for _cycle in 0..6 {
            world.step();
            // println!("After {} cycle(s)\n{}\n", _cycle + 1, world);
        }

        assert_eq!(848, world.count_active());
        Ok(())
    }

    #[test]
    fn day17_2() -> Result<()> {
        let mut world = HyperWorld::load("day17.txt")?;
        println!("{}", world);
        for _cycle in 0..6 {
            world.step();
            // println!("After {} cycle(s)\n{}\n", cycle + 1, world);
        }

        println!("Day 17.2: {}", world.count_active());
        Ok(())
    }
}

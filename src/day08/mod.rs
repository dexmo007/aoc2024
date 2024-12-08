use std::collections::HashSet;

use itertools::Itertools;

use crate::aoc_core::{AocResult, AocTask};

pub struct Day08;

type Position = (isize, isize);

struct Antenna {
    frequency: char,
    position: Position,
}

fn get_antinodes(
    (height, width): (isize, isize),
    (ay, ax): Position,
    (by, bx): Position,
) -> Vec<Position> {
    let dy = (by as isize) - (ay as isize);
    let dx = (bx as isize) - (ax as isize);
    let mut antinodes = Vec::new();
    {
        let y = (by as isize) + dy;
        let x = (bx as isize) + dx;
        if y >= 0 && y < height && x >= 0 && x < width {
            antinodes.push((y, x));
        }
    }
    {
        let y = (ay as isize) - dy;
        let x = (ax as isize) - dx;
        if y >= 0 && y < height && x >= 0 && x < width {
            antinodes.push((y, x));
        }
    }
    return antinodes;
}

impl AocTask for Day08 {
    fn solve_a(&self, contents: String) -> AocResult {
        let mut antennas = Vec::new();
        let mut width = None;
        let mut height = 0isize;
        for (y, line) in contents.lines().enumerate() {
            width = Some(line.len() as isize);
            for (x, c) in line.chars().enumerate() {
                if c == '.' {
                    continue;
                }
                antennas.push(Antenna {
                    frequency: c,
                    position: (y as isize, x as isize),
                });
            }
            height += 1;
        }
        let width = width.ok_or("empty map")?;
        let antennas_by_frequency = antennas.iter().into_group_map_by(|&a| a.frequency);
        let mut antinodes = HashSet::new();
        for (_, antennas) in antennas_by_frequency {
            for (a, b) in antennas.iter().tuple_combinations() {
                antinodes.extend(get_antinodes((height, width), a.position, b.position));
            }
        }

        Ok(antinodes.len() as i64)
    }

    fn solve_b(&self, contents: String) -> AocResult {
        let mut antennas = Vec::new();
        let mut width = None;
        let mut height = 0isize;
        for (y, line) in contents.lines().enumerate() {
            width = Some(line.len() as isize);
            for (x, c) in line.chars().enumerate() {
                if c == '.' {
                    continue;
                }
                antennas.push(Antenna {
                    frequency: c,
                    position: (y as isize, x as isize),
                });
            }
            height += 1;
        }
        let width = width.ok_or("empty map")?;
        let antennas_by_frequency = antennas.iter().into_group_map_by(|&a| a.frequency);
        let mut antinodes = HashSet::new();
        for (_, antennas) in antennas_by_frequency {
            for (a, b) in antennas.iter().tuple_combinations() {
                antinodes.extend(get_antinodes_b((height, width), a.position, b.position));
            }
        }

        Ok(antinodes.len() as i64)
    }
}

fn get_antinodes_b(
    (height, width): (isize, isize),
    (ay, ax): Position,
    (by, bx): Position,
) -> Vec<Position> {
    let dy = (by as isize) - (ay as isize);
    let dx = (bx as isize) - (ax as isize);
    let mut antinodes = vec![(ay, ax), (by, bx)];
    {
        let mut y = by;
        let mut x = bx;
        loop {
            y += dy;
            x += dx;
            if y >= 0 && y < height && x >= 0 && x < width {
                antinodes.push((y, x));
            } else {
                break;
            }
        }
    }
    {
        let mut y = ay;
        let mut x = ax;
        loop {
            y -= dy;
            x -= dx;
            if y >= 0 && y < height && x >= 0 && x < width {
                antinodes.push((y, x));
            } else {
                break;
            }
        }
    }
    return antinodes;
}

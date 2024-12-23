use std::{collections::HashSet, num::ParseIntError, path::Path, str::FromStr};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::aoc_core::{read_file, AocResult, AocTask};

pub struct Day14;

impl AocTask for Day14 {
    fn solve_a(&self, contents: String) -> AocResult {
        let mut lines = contents.lines();
        let raw_dim = lines.next().ok_or("Empty input".to_owned())?;
        let (width, height) = raw_dim
            .split(",")
            .map(|d| parse_number(d))
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .map(|&d| d)
            .collect_tuple::<(usize, usize)>()
            .ok_or(format!("Invalid dimensions: {raw_dim}"))?;

        let mut robots = lines.map(Robot::from_str).collect::<Result<Vec<_>, _>>()?;

        for _ in 0..100 {
            for robot in &mut robots {
                let (x, y) = robot.position;
                let (vx, vy) = robot.velocity;
                robot.position = (wrapping_add(x, vx, width), wrapping_add(y, vy, height));
            }
        }

        let mut quadrants = [0usize, 0, 0, 0];

        for robot in robots {
            let (x, y) = robot.position;

            let mut quadrant = 0usize;
            if width % 2 == 1 && x == width / 2 {
                continue;
            }
            if height % 2 == 1 && y == height / 2 {
                continue;
            }
            if x >= width / 2 {
                quadrant |= 1;
            }
            if y >= height / 2 {
                quadrant |= 1 << 1;
            }
            quadrants[quadrant] += 1;
        }
        println!("{:?}", quadrants);
        Ok(quadrants.iter().product::<usize>() as i64)
    }

    fn solve_b(&self, contents: String) -> AocResult {
        let mut lines = contents.lines();
        let raw_dim = lines.next().ok_or("Empty input".to_owned())?;
        let (width, height) = raw_dim
            .split(",")
            .map(|d| parse_number(d))
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .map(|&d| d)
            .collect_tuple::<(usize, usize)>()
            .ok_or(format!("Invalid dimensions: {raw_dim}"))?;

        let mut robots = lines.map(Robot::from_str).collect::<Result<Vec<_>, _>>()?;

        let mut seconds = 0usize;
        loop {
            if is_christmas_tree_present(&robots) {
                break;
            }
            for robot in &mut robots {
                let (x, y) = robot.position;
                let (vx, vy) = robot.velocity;
                robot.position = (wrapping_add(x, vx, width), wrapping_add(y, vy, height));
            }
            seconds += 1;
        }

        Ok(seconds as i64)
    }
}

fn is_christmas_tree_present(robots: &[Robot]) -> bool {
    let mut positions = HashSet::new();
    for robot in robots {
        positions.insert(robot.position);
    }
    'robot: for robot in robots {
        let (x, y) = robot.position;
        for &(cx, cy) in CHRISTMAS_TREE.iter() {
            if !positions.contains(&(x + cx, y + cy)) {
                continue 'robot;
            }
        }
        return true;
    }
    false
}

fn wrapping_add(lhs: usize, rhs: i32, bound: usize) -> usize {
    if rhs < 0 {
        let arhs = -rhs as usize;
        if lhs < arhs {
            return bound - (arhs - lhs);
        }
        return lhs - arhs;
    }
    let arhs = rhs as usize;
    if bound - lhs <= arhs {
        return arhs - (bound - lhs);
    }

    lhs + arhs
}

struct Robot {
    position: (usize, usize),
    velocity: (i32, i32),
}

lazy_static! {
    static ref ROBOT_PATTERN: Regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
}

impl FromStr for Robot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = ROBOT_PATTERN
            .captures(s.trim())
            .ok_or(format!("invalid robot: {s}"))?;
        let [pos_x, pos_y, vx, vy] = captures.extract().1;

        Ok(Robot {
            position: (parse_number(pos_x)?, parse_number(pos_y)?),
            velocity: (parse_number(vx)?, parse_number(vy)?),
        })
    }
}

fn parse_number<T: FromStr<Err = ParseIntError>>(s: &str) -> Result<T, String> {
    s.parse::<T>().map_err(|e| e.to_string())
}

lazy_static! {
    static ref CHRISTMAS_TREE: Vec<(usize, usize)> = read_christmas_tree();
}

fn read_christmas_tree() -> Vec<(usize, usize)> {
    let content = read_file(Path::new(file!()), "tree.txt");

    content
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars().enumerate().filter_map(move |(x, c)| {
                if c == 'X' {
                    return Some((x, y));
                }
                None
            })
        })
        .collect::<Vec<_>>()
}

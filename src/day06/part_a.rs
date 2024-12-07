use std::str::FromStr;

use crate::aoc_core::AocResult;

use super::core::{move_and_get, turn_90_deg, Direction, UP};

#[derive(Debug, PartialEq, Eq, Clone)]
enum PositionType {
    Free,
    Visited,
    Obstruction,
}
impl FromStr for PositionType {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "." => Ok(Self::Free),
            "#" => Ok(Self::Obstruction),
            _ => Err(String::from("invalid position type representation")),
        }
    }
}

fn mark_visited<'a>(
    map: &'a mut Vec<Vec<PositionType>>,
    (mut y, mut x): (usize, usize),
    mut direction: Direction,
) {
    loop {
        match move_and_get(&map, y, x, direction) {
            None => return,
            Some((ny, nx, PositionType::Visited)) => {
                (y, x) = (ny, nx);
            }
            Some((_, _, PositionType::Obstruction)) => {
                direction = turn_90_deg(direction);
            }
            Some((ny, nx, PositionType::Free)) => {
                map[ny][nx] = PositionType::Visited;
                (y, x) = (ny, nx);
            }
        }
    }
}

struct Input {
    map: Vec<Vec<PositionType>>,
    position: (usize, usize),
    direction: Direction,
}

fn parse_input(contents: String) -> Result<Input, String> {
    let mut map = Vec::new();
    let mut position = None;
    for (y, line) in contents.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let position_type = match c {
                '^' => {
                    position = Some((y, x));
                    PositionType::Visited
                }
                c => c.to_string().as_str().parse::<PositionType>()?,
            };
            row.push(position_type);
        }
        map.push(row);
    }
    let position = position.ok_or(String::from("no starting position found"))?;
    Ok(Input {
        map,
        position,
        direction: UP,
    })
}

pub fn solve_a(contents: String) -> AocResult {
    let Input {
        mut map,
        direction,
        position,
    } = parse_input(contents)?;

    mark_visited(&mut map, position, direction);

    let visited = map
        .iter()
        .map(|row| row.iter().filter(|&p| *p == PositionType::Visited).count() as i64)
        .sum::<i64>();
    // for row in map {
    //     println!(
    //         "{}",
    //         row.iter()
    //             .map(|c| match c {
    //                 PositionType::Free => ".",
    //                 PositionType::Visited => "X",
    //                 PositionType::Obstruction => "#",
    //             })
    //             .join("")
    //     )
    // }
    Ok(visited)
}

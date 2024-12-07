use std::{ops::Shl, str::FromStr};

use crate::aoc_core::AocResult;

use super::core::{move_and_get, Direction as DirectionDelta, DOWN, LEFT, RIGHT, UP};

#[derive(Debug, PartialEq, Eq, Clone)]
enum PositionType {
    Free,
    Visited(u8),
    Obstruction,
}
impl FromStr for PositionType {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "." => Ok(Self::Free),
            "#" => Ok(Self::Obstruction),
            "^" => Ok(Self::Visited(1 << Direction::Up)),
            _ => Err(String::from("invalid position type representation")),
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}
impl From<Direction> for u8 {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
}
impl Shl<Direction> for u8 {
    type Output = u8;

    fn shl(self, rhs: Direction) -> Self::Output {
        self << <u8>::from(rhs)
    }
}
impl Direction {
    fn turn_90_deg(self) -> Direction {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
    fn as_deltas(self) -> DirectionDelta {
        match self {
            Self::Up => UP,
            Self::Right => RIGHT,
            Self::Down => DOWN,
            Self::Left => LEFT,
        }
    }
}

fn mark_visited_with_loop(
    map: &mut Vec<Vec<PositionType>>,
    (mut y, mut x): (usize, usize),
    mut direction: Direction,
) -> bool {
    let mut did_just_turn = false;
    loop {
        match move_and_get(&map, y, x, direction.as_deltas()) {
            None => return false,
            Some((ny, nx, PositionType::Visited(mask))) => {
                if did_just_turn && (mask & (1 << direction)) > 0 {
                    return true;
                }
                map[ny][nx] = PositionType::Visited(mask | (1 << direction));
                (y, x) = (ny, nx);
                did_just_turn = false;
            }
            Some((_, _, PositionType::Obstruction)) => {
                direction = direction.turn_90_deg();
                did_just_turn = true;
            }
            Some((ny, nx, PositionType::Free)) => {
                map[ny][nx] = PositionType::Visited(1 << direction);
                (y, x) = (ny, nx);
                did_just_turn = false;
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
            let position_type = PositionType::from_str(&c.to_string())?;
            match position_type {
                PositionType::Visited(_) => {
                    position = Some((y, x));
                }
                _ => (),
            }
            row.push(position_type);
        }
        map.push(row);
    }
    let position = position.ok_or(String::from("no starting position found"))?;
    Ok(Input {
        map,
        position,
        direction: Direction::Up,
    })
}

pub fn solve_b(contents: String) -> AocResult {
    let Input {
        mut map,
        direction,
        position: starting_position,
    } = parse_input(contents)?;
    let original_map = map.to_vec();
    let already_looped = mark_visited_with_loop(&mut map, starting_position, direction);
    if already_looped {
        return Err(String::from("Guard is already looping"));
    }

    let possible_obstruction_positions = map.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().filter_map(move |(x, p)| match *p {
            PositionType::Visited(_) if (y, x) != starting_position => Some((y, x)),
            _ => None,
        })
    });

    let mut looping_obstruction_positions = 0;
    for (oy, ox) in possible_obstruction_positions {
        let mut map = original_map.to_vec();
        map[oy][ox] = PositionType::Obstruction;

        if mark_visited_with_loop(&mut map, starting_position, direction) {
            looping_obstruction_positions += 1;
        }
    }

    // TODO maybe faster to clean the existing vector

    Ok(looping_obstruction_positions)
}

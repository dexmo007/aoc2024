use std::{
    collections::{HashMap, HashSet},
    ops::Shl,
    str::FromStr,
    time::{Duration, Instant},
};

use crate::aoc_core::AocResult;

use super::core::{Direction as DirectionDelta, DOWN, LEFT, RIGHT, UP};

#[derive(Debug, PartialEq, Eq, Clone)]
enum PositionType {
    Initial,
    Free,
    Obstruction,
}
impl FromStr for PositionType {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "^" => Ok(Self::Initial),
            "." => Ok(Self::Free),
            "#" => Ok(Self::Obstruction),
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

#[inline]
pub fn move_guard(
    (dim_y, dim_x): Dimensions,
    (y, x): Position,
    (dy, dx): DirectionDelta,
) -> Option<(usize, usize)> {
    let ny = y.checked_add_signed(dy)?;
    if ny >= dim_y {
        return None;
    }
    let nx = x.checked_add_signed(dx)?;
    if nx >= dim_x {
        return None;
    }
    Some((ny, nx))
}

fn mark_visited_with_loop<'a>(
    dimensions: Dimensions,
    obstructions: &HashSet<Position>,
    state: &State,
) -> (bool, HashMap<Position, u8>) {
    let State {
        mut position,
        mut direction,
    } = state;
    let mut visited = HashMap::new();
    visited.insert(position, 1 << direction);
    loop {
        match move_guard(dimensions, position, direction.as_deltas()) {
            None => return (false, visited),
            Some((ny, nx)) => match visited.get_mut(&(ny, nx)) {
                Some(mask) => {
                    if *mask & (1 << direction) > 0 {
                        return (true, visited);
                    }

                    *mask |= 1 << direction;

                    position = (ny, nx);
                }
                None => {
                    if obstructions.contains(&(ny, nx)) {
                        direction = direction.turn_90_deg();
                        continue;
                    }

                    visited.insert((ny, nx), 1 << direction);
                    position = (ny, nx);
                }
            },
        }
    }
}

type Position = (usize, usize);
type Dimensions = (usize, usize);

struct State {
    position: Position,
    direction: Direction,
}
struct Grid {
    dimensions: Dimensions,
    obstructions: HashSet<Position>,
}

fn parse_input(contents: String) -> Result<(Grid, State), String> {
    let mut obstructions = HashSet::new();
    let mut position = None;
    let mut dim_y = 0;
    let mut dim_x = None;
    for (y, line) in contents.lines().enumerate() {
        dim_y += 1;
        if dim_x.is_none() {
            dim_x = Some(line.len());
        }
        for (x, c) in line.chars().enumerate() {
            let position_type = PositionType::from_str(&c.to_string())?;
            match position_type {
                PositionType::Initial => {
                    position = Some((y, x));
                }
                PositionType::Obstruction => {
                    obstructions.insert((y, x));
                }
                PositionType::Free => (),
            }
        }
    }
    let position = position.ok_or("no starting position found")?;
    let dim_x = dim_x.ok_or("Empty grid")?;
    Ok((
        Grid {
            dimensions: (dim_y, dim_x),
            obstructions,
        },
        State {
            position,
            direction: Direction::Up,
        },
    ))
}

pub fn solve_b(contents: String) -> AocResult {
    let (grid, initial_state) = parse_input(contents)?;

    let (looped, visited) =
        mark_visited_with_loop(grid.dimensions, &grid.obstructions, &initial_state);
    if looped {
        return Err(String::from("Guard is already looping"));
    }

    let possible_obstruction_positions = visited
        .keys()
        .map(|&pos| pos)
        .filter(|&pos| pos != initial_state.position);

    let mut count = 0;
    let mut looping_obstruction_positions = 0;
    let mut previous_obstruction: Option<(usize, usize)> = None;
    let mut obstructions = grid.obstructions.clone();
    let mut duration = Vec::new();
    for (oy, ox) in possible_obstruction_positions {
        count += 1;
        if let Some(previous_obstruction) = previous_obstruction {
            obstructions.remove(&previous_obstruction);
        }

        obstructions.insert((oy, ox));
        let before = Instant::now();
        let (looped, _) = mark_visited_with_loop(grid.dimensions, &obstructions, &initial_state);
        duration.push(before.elapsed());
        if looped {
            looping_obstruction_positions += 1;
        }
        previous_obstruction = Some((oy, ox))
    }
    println!("{count}");
    println!(
        "{:?}",
        duration.iter().sum::<Duration>() / duration.len() as u32
    );

    Ok(looping_obstruction_positions)
}

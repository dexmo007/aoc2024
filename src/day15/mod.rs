use std::str::FromStr;

use crate::aoc_core::{AocResult, AocTask};

pub struct Day15;

impl AocTask for Day15 {
    fn solve_a(&self, contents: String) -> AocResult {
        let mut wh = Warehouse::from_str(&contents)?;

        for movement in wh.movements.clone() {
            wh.move_robot(&movement);
        }

        let mut result = 0;
        for (y, row) in wh.map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    WarehouseContent::BOX => {
                        result += 100 * (y + 1) + x + 1;
                    }
                    _ => (),
                }
            }
        }

        Ok(result as i64)
    }

    fn solve_b(&self, _contents: String) -> AocResult {
        todo!()
    }
}

#[derive(Clone, Copy)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}
impl Direction {
    fn as_deltas(&self) -> (i8, i8) {
        match self {
            Self::UP => (-1, 0),
            Self::RIGHT => (0, 1),
            Self::DOWN => (1, 0),
            Self::LEFT => (0, -1),
        }
    }
}
impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "^" => Self::UP,
            ">" => Self::RIGHT,
            "v" => Self::DOWN,
            "<" => Self::LEFT,
            _ => return Err(format!("invalid direction: {s}")),
        })
    }
}

struct Warehouse {
    map: Vec<Vec<WarehouseContent>>,
    dimensions: (usize, usize),
    robot_position: (usize, usize),
    movements: Vec<Direction>,
}
impl Warehouse {
    fn move_robot(&mut self, direction: &Direction) {
        let (y, x) = self.robot_position;
        if let Some((ny, nx)) = self.next_position(direction) {
            let content = &self.map[ny][nx];
            match content {
                WarehouseContent::WALL => {}
                WarehouseContent::EMPTY => {
                    self.map[ny][nx] = WarehouseContent::ROBOT;
                    self.map[y][x] = WarehouseContent::EMPTY;
                    self.robot_position = (ny, nx);
                }
                WarehouseContent::BOX => {
                    let mut cursor = (ny, nx);
                    let mut next;
                    loop {
                        next = Self::_next_position(cursor, self.dimensions, direction);
                        match next {
                            None => break,
                            Some((cy, cx)) => {
                                match self.map[cy][cx] {
                                    WarehouseContent::WALL => break,
                                    WarehouseContent::EMPTY => {
                                        self.map[ny][nx] = WarehouseContent::ROBOT;
                                        self.map[y][x] = WarehouseContent::EMPTY;
                                        self.map[cy][cx] = WarehouseContent::BOX;
                                        self.robot_position = (ny, nx);
                                        break;
                                    }
                                    WarehouseContent::ROBOT => unreachable!(),
                                    WarehouseContent::BOX => {}
                                };
                                cursor = (cy, cx);
                            }
                        }
                    }
                }
                WarehouseContent::ROBOT => unreachable!(),
            }
        }
    }

    fn next_position(&self, direction: &Direction) -> Option<(usize, usize)> {
        Self::_next_position(self.robot_position, self.dimensions, direction)
    }
    fn _next_position(
        position: (usize, usize),
        dimensions: (usize, usize),
        direction: &Direction,
    ) -> Option<(usize, usize)> {
        let (y, x) = position;
        let (dy, dx) = direction.as_deltas();
        let ny = y.checked_add_signed(dy as isize)?;
        let nx = x.checked_add_signed(dx as isize)?;
        let (height, width) = dimensions;
        if ny >= height || nx >= width {
            return None;
        }
        Some((ny, nx))
    }
}
impl FromStr for Warehouse {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        lines.next();
        let mut map = Vec::new();
        let mut robot_position = None;
        for (y, line) in (&mut lines).enumerate() {
            let raw_content = &line[1..line.len() - 1];
            let mut row = Vec::new();
            let mut all_walls = true;
            for (x, c) in raw_content.chars().enumerate() {
                let content = WarehouseContent::from_str(&c.to_string())?;
                match content {
                    WarehouseContent::ROBOT => {
                        robot_position = Some((y, x));
                        all_walls = false;
                    }
                    WarehouseContent::WALL => {}
                    _ => {
                        all_walls = false;
                    }
                }
                row.push(content);
            }
            if all_walls {
                break;
            }
            map.push(row);
        }
        lines.next();
        let movements = lines
            .flat_map(|line| line.chars().map(|c| Direction::from_str(&c.to_string())))
            .collect::<Result<Vec<_>, _>>()?;

        let dimensions = (map.len(), map[0].len());
        Ok(Warehouse {
            map,
            dimensions,
            robot_position: robot_position.ok_or("Robot position not found".to_owned())?,
            movements,
        })
    }
}

enum WarehouseContent {
    ROBOT,
    BOX,
    EMPTY,
    WALL,
}
impl FromStr for WarehouseContent {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "@" => Self::ROBOT,
            "O" => Self::BOX,
            "." => Self::EMPTY,
            "#" => Self::WALL,
            _ => return Err(format!("invalid warehouse content: {s}")),
        })
    }
}

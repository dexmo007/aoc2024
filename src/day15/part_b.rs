use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    str::FromStr,
};

use crate::aoc_core::AocResult;

pub(crate) fn solve_b(contents: String) -> AocResult {
    let mut wh = Warehouse::from_str(&contents)?.explode();
    for movement in wh.movements.clone() {
        wh.move_robot(&movement);
    }

    let mut result = 0;
    for (y, row) in wh.map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            match cell {
                WarehouseContent::BigBox(1) => {
                    result += 100 * (y + 1) + x + 2;
                }
                _ => (),
            }
        }
    }

    Ok(result as i64)
}

#[derive(Clone, Copy, PartialEq, Eq)]
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
    fn go_from(&self, (y, x): (usize, usize)) -> Option<(usize, usize)> {
        let (dy, dx) = self.as_deltas();
        let ny = y.checked_add_signed(dy as isize)?;
        let nx = x.checked_add_signed(dx as isize)?;
        Some((ny, nx))
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
                WarehouseContent::Box => panic!("should not exist anymore"),
                WarehouseContent::Wall => {}
                WarehouseContent::Empty => {
                    self.map[ny][nx] = WarehouseContent::Robot;
                    self.map[y][x] = WarehouseContent::Empty;
                    self.robot_position = (ny, nx);
                }
                WarehouseContent::BigBox(_) => {
                    let mut cursor = VecDeque::from(vec![(y, x)]);

                    let mut boxes_to_move = VecDeque::new();
                    let mut boxes_to_move_set = HashSet::new();
                    let mut seen = HashSet::new();
                    let mut blocked = false;
                    while !blocked && !cursor.is_empty() {
                        let position = cursor.pop_front().unwrap();
                        if seen.contains(&position) {
                            continue;
                        }
                        seen.insert(position);
                        let next = Self::_next_position(position, self.dimensions, direction);
                        match next {
                            None => {
                                blocked = true;
                                break;
                            }
                            Some((cy, cx)) => {
                                match self.map[cy][cx] {
                                    WarehouseContent::Wall => {
                                        blocked = true;
                                        break;
                                    }
                                    WarehouseContent::Empty => continue,
                                    WarehouseContent::BigBox(side) => match direction {
                                        Direction::DOWN | Direction::UP => {
                                            if boxes_to_move_set.contains(&(cy, cx)) {
                                                continue;
                                            }
                                            let cx_side =
                                                cx.checked_add_signed(side as isize).unwrap();
                                            boxes_to_move_set.insert((cy, cx));
                                            boxes_to_move_set.insert((cy, cx_side));
                                            cursor.push_back((cy, cx));
                                            cursor.push_back((cy, cx_side));
                                            boxes_to_move.push_front((cy, cx));
                                            boxes_to_move.push_front((cy, cx_side));
                                        }
                                        Direction::LEFT | Direction::RIGHT => {
                                            let edge = direction.go_from((cy, cx)).unwrap();
                                            cursor.push_back(edge);
                                            if boxes_to_move_set.contains(&edge) {
                                                continue;
                                            }
                                            boxes_to_move_set.insert(edge);
                                            boxes_to_move_set.insert((cy, cx));
                                            boxes_to_move.push_front((cy, cx));
                                            boxes_to_move.push_front(edge);
                                        }
                                    },
                                    WarehouseContent::Robot => unreachable!(),
                                    WarehouseContent::Box => panic!("should not exist anymore"),
                                };
                            }
                        }
                    }
                    if blocked {
                        return;
                    }
                    for (y, x) in boxes_to_move {
                        let (ny, nx) = direction.go_from((y, x)).unwrap();
                        self.map[ny][nx] = self.map[y][x];
                        self.map[y][x] = WarehouseContent::Empty;
                    }
                    self.map[y][x] = WarehouseContent::Empty;
                    self.map[ny][nx] = WarehouseContent::Robot;
                    self.robot_position = (ny, nx);
                }
                WarehouseContent::Robot => unreachable!(),
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

    #[allow(dead_code)]
    fn explode(&self) -> Self {
        let mut map = Vec::with_capacity(self.map.len() * 2);
        let mut robot_position = None;
        for (y, row) in self.map.iter().enumerate() {
            let mut exploded = Vec::with_capacity(row.len() * 2);
            for (x, cell) in row.iter().enumerate() {
                let cells = match *cell {
                    WarehouseContent::Box => {
                        (WarehouseContent::BigBox(1), WarehouseContent::BigBox(-1))
                    }
                    WarehouseContent::Empty => (WarehouseContent::Empty, WarehouseContent::Empty),
                    WarehouseContent::Wall => (WarehouseContent::Wall, WarehouseContent::Wall),
                    WarehouseContent::Robot => {
                        robot_position = Some((y, x * 2));
                        (WarehouseContent::Robot, WarehouseContent::Empty)
                    }
                    _ => unreachable!(),
                };
                exploded.push(cells.0);
                exploded.push(cells.1);
            }
            map.push(exploded);
        }

        let dimensions = (map.len(), map[0].len());
        Warehouse {
            map,
            dimensions,
            robot_position: robot_position.unwrap(),
            movements: self.movements.clone(),
        }
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
                    WarehouseContent::Robot => {
                        robot_position = Some((y, x));
                        all_walls = false;
                    }
                    WarehouseContent::Wall => {}
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

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            for cell in row {
                f.write_fmt(format_args!("{}", cell))?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
enum WarehouseContent {
    Robot,
    Box,
    Empty,
    Wall,
    BigBox(i8),
}
impl FromStr for WarehouseContent {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "@" => Self::Robot,
            "O" => Self::Box,
            "." => Self::Empty,
            "#" => Self::Wall,
            "[" => Self::BigBox(1),
            "]" => Self::BigBox(-1),
            _ => return Err(format!("invalid warehouse content: {s}")),
        })
    }
}
impl Display for WarehouseContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Robot => "@",
            Self::Box => "O",
            Self::BigBox(-1) => "]",
            Self::BigBox(1) => "[",
            Self::BigBox(_) => return Err(std::fmt::Error),
            Self::Wall => "#",
            Self::Empty => ".",
        };
        f.write_str(s)?;

        Ok(())
    }
}

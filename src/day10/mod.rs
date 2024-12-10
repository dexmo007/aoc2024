use std::collections::HashSet;

use crate::aoc_core::{AocResult, AocTask};

pub struct Day10;

impl AocTask for Day10 {
    fn solve_a(&self, contents: String) -> AocResult {
        let mut map = Vec::new();
        for line in contents.lines() {
            map.push(
                line.chars()
                    .map(|c| {
                        if c == '.' {
                            99u8
                        } else {
                            c.to_digit(10).unwrap() as u8
                        }
                    })
                    .collect::<Vec<_>>(),
            );
        }

        fn find_all_trails(
            map: &Vec<Vec<u8>>,
            y: usize,
            x: usize,
            current: u8,
            solutions: &mut HashSet<(usize, usize)>,
        ) {
            if current == 9 {
                solutions.insert((y, x));
                return;
            }
            if y > 0 {
                let next = map[y - 1][x];
                if next == current + 1 {
                    find_all_trails(map, y - 1, x, current + 1, solutions);
                }
            }
            {
                let next = map[y].get(x + 1);
                if next.is_some() && *next.unwrap() == current + 1 {
                    find_all_trails(map, y, x + 1, current + 1, solutions);
                }
            }
            {
                let next = map.get(y + 1).map(|row| row[x]);
                if next.is_some() && next.unwrap() == current + 1 {
                    find_all_trails(map, y + 1, x, current + 1, solutions);
                }
            }
            if x > 0 {
                let next = map[y][x - 1];
                if next == current + 1 {
                    find_all_trails(map, y, x - 1, current + 1, solutions);
                }
            }
        }

        let mut sum_of_scores = 0i64;
        for (y, row) in map.iter().enumerate() {
            for (x, &val) in row.iter().enumerate() {
                if val != 0 {
                    continue;
                }
                let mut trail_ends = HashSet::new();
                find_all_trails(&map, y, x, 0, &mut trail_ends);
                sum_of_scores += trail_ends.len() as i64;
            }
        }

        Ok(sum_of_scores)
    }

    fn solve_b(&self, contents: String) -> AocResult {
        let mut map = Vec::new();
        for line in contents.lines() {
            map.push(
                line.chars()
                    .map(|c| {
                        if c == '.' {
                            99u8
                        } else {
                            c.to_digit(10).unwrap() as u8
                        }
                    })
                    .collect::<Vec<_>>(),
            );
        }

        fn find_all_trails(map: &Vec<Vec<u8>>, y: usize, x: usize, current: u8) -> i64 {
            if current == 9 {
                return 1;
            }
            let mut res = 0;
            if y > 0 {
                let next = map[y - 1][x];
                if next == current + 1 {
                    res += find_all_trails(map, y - 1, x, current + 1);
                }
            }
            {
                let next = map[y].get(x + 1);
                if next.is_some() && *next.unwrap() == current + 1 {
                    res += find_all_trails(map, y, x + 1, current + 1);
                }
            }
            {
                let next = map.get(y + 1).map(|row| row[x]);
                if next.is_some() && next.unwrap() == current + 1 {
                    res += find_all_trails(map, y + 1, x, current + 1);
                }
            }
            if x > 0 {
                let next = map[y][x - 1];
                if next == current + 1 {
                    res += find_all_trails(map, y, x - 1, current + 1);
                }
            }
            res
        }

        let mut sum_of_ratings = 0i64;
        for (y, row) in map.iter().enumerate() {
            for (x, &val) in row.iter().enumerate() {
                if val != 0 {
                    continue;
                }
                sum_of_ratings += find_all_trails(&map, y, x, 0);
            }
        }

        Ok(sum_of_ratings)
    }
}

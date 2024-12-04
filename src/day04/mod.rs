use itertools::Itertools;

use crate::aoc_core::{AocResult, AocTask};

pub struct Day04;

#[inline]
fn mat_get(lines: &Vec<Vec<char>>, i: i32, j: i32) -> Option<&char> {
    if i < 0 || j < 0 {
        return None;
    }
    return lines.get(i as usize).and_then(|line| line.get(j as usize));
}

impl AocTask for Day04 {
    fn solve_a(&self, contents: String) -> AocResult {
        let lines = contents
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let mut occurances: i32 = 0;
        let width = lines.get(0).unwrap().len() as i32;
        for i in 0..(lines.len() as i32) {
            for j in 0..width {
                'dir_scan: for (di, dj) in [(0, 1), (1, 0), (1, 1), (-1, 1)] {
                    let mut word = String::from("");
                    for offset in 0..4 {
                        match mat_get(&lines, i + (di * offset), j + (dj * offset)) {
                            Some(c) => {
                                word.push(*c);
                            }
                            None => {
                                continue 'dir_scan;
                            }
                        }
                    }
                    if word == "XMAS" || word == "SAMX" {
                        occurances += 1;
                    }
                }
            }
        }
        println!("Result: {}", occurances);
        Ok(())
    }

    fn solve_b(&self, contents: String) -> AocResult {
        let lines = contents
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let mut occurances: i32 = 0;
        let width = lines.get(0).unwrap().len();
        for i in 1..(lines.len() - 1) {
            for j in 1..(width - 1) {
                let a = lines[i][j];
                if a != 'A' {
                    continue;
                }
                let left_up = lines[i - 1][j - 1];
                let right_down = lines[i + 1][j + 1];
                let right_up = lines[i - 1][j + 1];
                let left_down = lines[i + 1][j - 1];

                #[inline]
                fn is_ms(c1: char, c2: char) -> bool {
                    c1 == 'M' && c2 == 'S' || c1 == 'S' && c2 == 'M'
                }
                if is_ms(left_up, right_down) && is_ms(right_up, left_down) {
                    occurances += 1;
                }
            }
        }
        println!("Result: {}", occurances);
        Ok(())
    }
}

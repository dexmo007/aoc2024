use regex::{Captures, Regex};

use crate::aoc_core::{AocResult, AocTask};

pub struct Day03;

fn parse_factors(captures: Captures<'_>) -> (i64, i64) {
    let factor1 = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
    let factor2 = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
    return (factor1, factor2);
}

impl AocTask for Day03 {
    fn solve_a(&self, contents: String) -> AocResult {
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        let mut total: i64 = 0;
        for capture in re.captures_iter(&contents) {
            let (factor1, factor2) = parse_factors(capture);
            total += factor1 * factor2;
        }
        println!("Sum of products: {}", total);
        Ok(())
    }

    fn solve_b(&self, contents: String) -> AocResult {
        let do_pattern = r"do\(\)";
        let dont_pattern = r"don't\(\)";
        let mul_pattern = r"mul\((\d{1,3}),(\d{1,3})\)";

        let re = Regex::new(&format!(
            "(?:{}|{}|{})",
            dont_pattern, do_pattern, mul_pattern
        ))
        .unwrap();

        let mut total: i64 = 0;
        let mut mul_enabled = true;
        for capture in re.captures_iter(&contents) {
            let m = capture.get(0).unwrap().as_str();
            match m {
                "do()" => {
                    mul_enabled = true;
                }
                "don't()" => {
                    mul_enabled = false;
                }
                _ => {
                    if !mul_enabled {
                        continue;
                    }
                    let (f1, f2) = parse_factors(capture);
                    total += f1 * f2;
                }
            }
        }
        println!("Sum of products: {}", total);
        Ok(())
    }
}

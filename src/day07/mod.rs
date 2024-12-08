use std::{collections::VecDeque, num::TryFromIntError};

use itertools::Itertools;

use crate::aoc_core::{AocResult, AocTask};

pub struct Day07;

fn is_solvable_a(expected_result: u64, mut values: VecDeque<u64>) -> bool {
    fn _is_solvable(
        expected_result: u64,
        intermediate_result: u64,
        mut values: VecDeque<u64>,
    ) -> bool {
        if intermediate_result > expected_result {
            return false;
        }
        match values.pop_front() {
            None => return expected_result == intermediate_result,
            Some(val) => {
                _is_solvable(expected_result, intermediate_result * val, values.clone())
                    || _is_solvable(expected_result, intermediate_result + val, values)
            }
        }
    }

    match values.pop_front() {
        None => false,
        Some(first) => _is_solvable(expected_result, first, values),
    }
}

fn is_solvable_b(expected_result: u64, mut values: VecDeque<u64>) -> bool {
    fn _is_solvable(
        expected_result: u64,
        intermediate_result: u64,
        mut values: VecDeque<u64>,
    ) -> bool {
        if intermediate_result > expected_result {
            return false;
        }
        match values.pop_front() {
            None => return expected_result == intermediate_result,
            Some(val) => {
                _is_solvable(
                    expected_result,
                    intermediate_result * (10u64.pow(val.ilog10() + 1)) + val,
                    values.clone(),
                ) || _is_solvable(expected_result, intermediate_result * val, values.clone())
                    || _is_solvable(expected_result, intermediate_result + val, values)
            }
        }
    }

    match values.pop_front() {
        None => false,
        Some(first) => _is_solvable(expected_result, first, values),
    }
}

impl AocTask for Day07 {
    fn solve_a(&self, contents: String) -> AocResult {
        let mut equations = Vec::new();
        for line in contents.lines() {
            let (raw_result, raw_values) = line.split_once(":").ok_or("Invalid line")?;
            let result = raw_result.parse::<u64>().map_err(|e| e.to_string())?;
            let values: VecDeque<_> = raw_values
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<u64>())
                .try_collect()
                .map_err(|e| e.to_string())?;
            equations.push((result, values));
        }

        let mut total_result = 0u64;

        for (expected_result, values) in equations {
            if is_solvable_a(expected_result, values) {
                total_result += expected_result;
            }
        }

        total_result
            .try_into()
            .map_err(|e: TryFromIntError| e.to_string())
    }

    fn solve_b(&self, contents: String) -> AocResult {
        let mut equations = Vec::new();
        for line in contents.lines() {
            let (raw_result, raw_values) = line.split_once(":").ok_or("Invalid line")?;
            let result = raw_result.parse::<u64>().map_err(|e| e.to_string())?;
            let values: VecDeque<_> = raw_values
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<u64>())
                .try_collect()
                .map_err(|e| e.to_string())?;
            equations.push((result, values));
        }

        let mut total_result = 0u64;

        for (expected_result, values) in equations {
            if is_solvable_b(expected_result, values) {
                total_result += expected_result;
            }
        }

        total_result
            .try_into()
            .map_err(|e: TryFromIntError| e.to_string())
    }
}

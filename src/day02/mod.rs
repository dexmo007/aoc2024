use crate::aoc_core::{AocResult, AocTask};
use itertools::Itertools;
pub struct Day02;

impl AocTask for Day02 {
    fn solve_a(&self, contents: String) -> AocResult {
        let reports = parse_input(contents)?;

        let mut safe_reports = 0;
        for report in reports {
            if is_report_safe(report.iter()) {
                safe_reports += 1;
            }
        }
        Ok(safe_reports as i64)
    }

    fn solve_b(&self, contents: String) -> AocResult {
        let reports = parse_input(contents)?;
        let mut safe_reports = 0;
        for report in reports {
            for skip_index in 0..report.len() {
                let iter = report
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != skip_index)
                    .map(|(_, v)| v);
                if is_report_safe(iter) {
                    safe_reports += 1;
                    break;
                }
            }
        }
        Ok(safe_reports as i64)
    }
}

fn is_report_safe<'a, T: Iterator<Item = &'a i32>>(report: T) -> bool {
    let mut increasing = None;
    for (value, next_value) in report.tuple_windows() {
        if increasing.is_none() {
            increasing = Some(next_value > value)
        }
        let delta = next_value - value;
        let abs_delta = delta.abs();
        if abs_delta == 0 || abs_delta > 3 || (delta > 0) != increasing.unwrap() {
            return false;
        }
    }
    true
}

fn parse_input(contents: String) -> Result<Vec<Vec<i32>>, String> {
    let mut reports = Vec::new();
    for line in contents.lines() {
        let (values, errors): (Vec<_>, Vec<_>) = line
            .split_whitespace()
            .map(|v| v.parse::<i32>())
            .partition(Result::is_ok);
        if errors.len() > 0 {
            return Err(String::from("error parsing numbers"));
        }
        let values: Vec<_> = values.into_iter().map(Result::unwrap).collect();
        reports.push(values);
    }
    Ok(reports)
}

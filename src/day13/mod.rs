use std::str::FromStr;

use ndarray::prelude::*;
use ndarray_linalg::Solve;
use regex::Regex;

use crate::aoc_core::{AocResult, AocTask};

pub struct Day13;

const BUTTON_A_COST: usize = 3;
const BUTTON_B_COST: usize = 1;

const PRIZE_OFFSET: usize = 10000000000000;

impl AocTask for Day13 {
    fn solve_a(&self, contents: String) -> AocResult {
        let claw_machines: Vec<ClawMachine> = contents
            .split("\n\n")
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()?;

        let mut result = 0usize;
        for claw_machine in claw_machines {
            let btn: Array2<f64> = array![
                [
                    claw_machine.button_a.0 as f64,
                    claw_machine.button_b.0 as f64
                ],
                [
                    claw_machine.button_a.1 as f64,
                    claw_machine.button_b.1 as f64
                ]
            ];
            let prize: Array1<f64> =
                array![claw_machine.prize.0 as f64, claw_machine.prize.1 as f64];

            let solution = btn.solve_into(prize).map_err(|e| e.to_string())?;

            let a = solution[0].round() as usize;
            let b = solution[1].round() as usize;

            if a * claw_machine.button_a.0 + b * claw_machine.button_b.0 != claw_machine.prize.0
                || a * claw_machine.button_a.1 + b * claw_machine.button_b.1 != claw_machine.prize.1
            {
                continue;
            }

            result += a * BUTTON_A_COST + b * BUTTON_B_COST;

            // println!("{solution:?}");
        }

        Ok(result as i64)
    }

    fn solve_b(&self, contents: String) -> AocResult {
        let claw_machines: Vec<ClawMachine> = contents
            .split("\n\n")
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()?;

        let mut result = 0usize;
        for claw_machine in claw_machines {
            let btn: Array2<f64> = array![
                [
                    claw_machine.button_a.0 as f64,
                    claw_machine.button_b.0 as f64
                ],
                [
                    claw_machine.button_a.1 as f64,
                    claw_machine.button_b.1 as f64
                ]
            ];
            let prize: Array1<f64> = array![
                claw_machine.prize.0 as f64 + PRIZE_OFFSET as f64,
                claw_machine.prize.1 as f64 + PRIZE_OFFSET as f64
            ];

            let solution = btn.solve_into(prize).map_err(|e| e.to_string())?;
            println!("{solution:?}");

            let a = solution[0].round() as usize;
            let b = solution[1].round() as usize;
            // println!("a={a} b={b}");
            // println!(
            //     "sol={}, prize={}",
            //     a * claw_machine.button_a.0 + b * claw_machine.button_b.0,
            //     claw_machine.prize.0
            // );

            if a * claw_machine.button_a.0 + b * claw_machine.button_b.0 - PRIZE_OFFSET
                != claw_machine.prize.0
                || a * claw_machine.button_a.1 + b * claw_machine.button_b.1 - PRIZE_OFFSET
                    != claw_machine.prize.1
            {
                continue;
            }

            result += a * BUTTON_A_COST + b * BUTTON_B_COST;
        }

        Ok(result as i64)
    }
}

type Button = (usize, usize);
type Position = (usize, usize);

struct ClawMachine {
    button_a: Button,
    button_b: Button,
    prize: Position,
}
impl FromStr for ClawMachine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw = s.trim().lines().collect::<Vec<_>>();

        if raw.len() != 3 {
            return Err("expected 3 lines".to_owned());
        }
        Ok(ClawMachine {
            button_a: parse_line(raw[0], r"^Button A: X\+(\d+), Y\+(\d+)$")?,
            button_b: parse_line(raw[1], r"^Button B: X\+(\d+), Y\+(\d+)$")?,
            prize: parse_line(raw[2], r"^Prize: X=(\d+), Y=(\d+)$")?,
        })
    }
}

fn parse_line(s: &str, pattern: &str) -> Result<(usize, usize), String> {
    let btn_a = Regex::new(pattern)
        .unwrap()
        .captures(s.trim())
        .ok_or(format!("invalid input: {s}"))?;
    let [btn_a_x, btn_a_y] = btn_a.extract().1;
    Ok((btn_a_x.parse().unwrap(), btn_a_y.parse().unwrap()))
}

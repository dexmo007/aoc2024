use crate::aoc_core::{AocResult, AocTask};

mod core;
mod part_a;
mod part_b;

pub struct Day06;

impl AocTask for Day06 {
    fn solve_a(&self, contents: String) -> AocResult {
        part_a::solve_a(contents)
    }

    fn solve_b(&self, contents: String) -> AocResult {
        part_b::solve_b(contents)
    }
}

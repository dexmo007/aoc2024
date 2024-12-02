use crate::aoc_core::{AocResult, AocTask};

use std::{collections::HashMap, iter::zip};

pub struct Day01;

impl AocTask for Day01 {
    fn solve_a(&self, contents: String) -> AocResult {
        let mut left = Vec::new();
        let mut right = Vec::new();
        for line in contents.lines() {
            let values: Vec<&str> = line.trim().split_whitespace().collect();
            left.push(values[0].parse::<i32>().map_err(|e| e.to_string())?);
            right.push(values[1].parse::<i32>().map_err(|e| e.to_string())?);
        }

        left.sort();
        right.sort();

        let mut total_dist = 0;

        for (l, r) in zip(left, right) {
            let dist = (l - r).abs();
            total_dist += dist;
        }
        println!("Total distance is: {}", total_dist);
        Ok(())
    }

    fn solve_b(&self, contents: String) -> AocResult {
        let mut left: Vec<i32> = Vec::new();
        let mut right: Vec<i32> = Vec::new();
        for line in contents.lines() {
            let values: Vec<&str> = line.trim().split_whitespace().collect();
            left.push(values[0].parse::<i32>().map_err(|e| e.to_string())?);
            right.push(values[1].parse::<i32>().map_err(|e| e.to_string())?);
        }

        let mut right_occurances = HashMap::new();

        for r in right {
            let occ = right_occurances.get(&r).unwrap_or(&0);
            right_occurances.insert(r, occ + 1);
        }

        let mut similarity_score = 0;

        for l in left {
            let occ = right_occurances.get(&l).unwrap_or(&0);
            let score = l * occ;
            similarity_score += score;
        }
        println!("Similarity score is: {}", similarity_score);
        Ok(())
    }
}

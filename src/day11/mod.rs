use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Add, Div},
};

use crate::aoc_core::{AocResult, AocTask};

pub struct Day11;

impl AocTask for Day11 {
    fn solve_a(&self, contents: String) -> AocResult {
        let mut stones = contents
            .split_whitespace()
            .map(|v| v.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        for _ in 0..25 {
            blink(&mut stones);
        }
        // println!("{:?}", &stones);
        Ok(stones.len() as i64)
    }

    fn solve_b(&self, contents: String) -> AocResult {
        let stones = contents
            .split_whitespace()
            .map(|v| v.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        let mut stones = stones.iter().fold(HashMap::new(), |mut map, &stone| {
            let count = map.get(&stone).unwrap_or(&0u64);
            map.insert(stone, count + 1);
            map
        });

        for i in 0..75 {
            println!("Iteration: {i}, len: {}", stones.len());
            stones = blink_b(&stones);
        }
        // println!("{:?}", &stones);
        Ok(stones.values().fold(0u64, |count, c| count + c) as i64)
    }
}

fn blink(stones: &mut Vec<u64>) {
    let mut i = 0;
    while i < stones.len() {
        let offset = on_change(stones, i);
        i += offset + 1;
    }
}

fn on_change(stones: &mut Vec<u64>, i: usize) -> usize {
    let stone = stones.get_mut(i).unwrap();
    if *stone == 0 {
        *stone = 1;
        return 0;
    }
    let n_digits = n_digits(*stone);
    if n_digits % 2 == 0 {
        let p = 10u64.pow(n_digits / 2);
        let left = (*stone).div(p);
        *stone = *stone % p;
        stones.insert(i, left);
        return 1;
    }

    *stone = *stone * 2024;
    0
}

fn blink_b(stones: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut next = HashMap::new();
    for (&stone, &count) in stones {
        on_change_b(&mut next, stone, count);
    }
    next
}

#[inline]
fn on_change_b(stones: &mut HashMap<u64, u64>, stone: u64, count: u64) {
    if stone == 0 {
        update_add(stones, 1, count);
        return;
    }
    let n_digits = n_digits(stone);
    if n_digits % 2 == 0 {
        let p = 10u64.pow(n_digits / 2);
        update_add(stones, stone / p, count);
        update_add(stones, stone % p, count);
        return;
    }
    update_add(stones, stone * 2024, count);
}

#[inline]
fn update_add<K, V>(map: &mut HashMap<K, V>, key: K, value: V)
where
    K: Eq + Hash,
    V: Default + Copy + Add<V, Output = V>,
{
    let v = map.get(&key).map(|v| *v).unwrap_or(Default::default());
    map.insert(key, v + value);
}

#[inline]
fn n_digits(n: u64) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

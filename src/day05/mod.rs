use std::collections::HashMap;

use itertools::Itertools;

use crate::aoc_core::{AocResult, AocTask};

pub struct Day05;

type OrderingRules = HashMap<i32, Vec<i32>>;

struct Input {
    ordering_rules: OrderingRules,
    updates_list: Vec<Vec<i32>>,
}

fn parse_input(contents: String) -> Result<Input, String> {
    let mut lines = contents.lines().into_iter();
    let mut ordering_rules: OrderingRules = HashMap::new();
    loop {
        match lines.next() {
            Some(line) => {
                if line.trim().is_empty() {
                    break;
                }
                let (before, after) = line
                    .split("|")
                    .map(|n| n.parse().unwrap())
                    .collect_tuple::<(i32, i32)>()
                    .unwrap();
                ordering_rules.entry(before).or_default().push(after);
            }
            None => return Err(String::from("Unexpected EOF")),
        }
    }
    let mut updates_list: Vec<Vec<i32>> = Vec::new();
    loop {
        match lines.next() {
            Some(line) => {
                updates_list.push(line.split(",").map(|v| v.parse().unwrap()).collect_vec());
            }
            None => {
                break;
            }
        }
    }
    if updates_list.len() == 0 {
        return Err(String::from("Unexpected EOF"));
    }
    Ok(Input {
        ordering_rules,
        updates_list,
    })
}

fn is_correct_order(ordering_rules: &OrderingRules, updates: &Vec<i32>) -> bool {
    for i in 1..updates.len() {
        let page = updates[i];
        match ordering_rules.get(&page) {
            Some(afters) => {
                let before = &updates[0..i];
                if afters.iter().any(|after| before.contains(after)) {
                    return false;
                }
            }
            None => (),
        }
    }
    true
}

impl AocTask for Day05 {
    fn solve_a(&self, contents: String) -> AocResult {
        let Input {
            ordering_rules,
            updates_list,
        } = parse_input(contents)?;
        let mut result = 0;
        for updates in updates_list {
            if is_correct_order(&ordering_rules, &updates) {
                let middle_index = updates.len().div_ceil(2) - 1;
                result += updates.get(middle_index).unwrap();
            }
        }
        Ok(result as i64)
    }

    fn solve_b(&self, contents: String) -> AocResult {
        let Input {
            ordering_rules,
            ref mut updates_list,
        } = parse_input(contents)?;
        let mut result = 0;
        for updates in updates_list {
            let mut was_incorrect = false;
            let mut i = 1;
            let n_updates = updates.len();
            while i < n_updates {
                let page = updates[i];

                if let Some(afters) = ordering_rules.get(&page) {
                    let before = &updates[0..i];

                    if let Some(min_after_index) = afters
                        .iter()
                        .map(|after| {
                            before
                                .iter()
                                .enumerate()
                                .find(|&(_, b)| b == after)
                                .map(|(index, _)| index)
                        })
                        .filter_map(|o| o)
                        .min()
                    {
                        was_incorrect = true;
                        updates.remove(i);
                        updates.insert(min_after_index, page);
                        i = min_after_index;
                    }
                }
                i += 1;
            }
            if !was_incorrect {
                continue;
            }
            let middle_index = updates.len().div_ceil(2) - 1;
            result += updates.get(middle_index).unwrap();
        }
        Ok(result as i64)
    }
}

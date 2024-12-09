use std::num::TryFromIntError;

use itertools::Itertools;

use crate::aoc_core::{AocResult, AocTask};

pub struct Day09;

fn parse_input(contents: String) -> Result<Vec<(usize, usize)>, String> {
    let mut file_block = None;
    let mut disk_map = Vec::new();
    for c in contents.chars() {
        let i: usize = c
            .to_digit(10)
            .ok_or("Invalid char")?
            .try_into()
            .map_err(|e: TryFromIntError| e.to_string())?;
        if let Some(f) = file_block {
            disk_map.push((f, i));
            file_block = None;
        } else {
            file_block = Some(i);
        }
    }
    if let Some(f) = file_block {
        disk_map.push((f, 0));
    }
    Ok(disk_map)
}

fn compact_disk(disk_map: &mut Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut i = 0;
    let mut j = disk_map.len() - 1;
    let mut compacted_disk_map = Vec::new();
    let mut already_added_i = false;
    while j > i {
        let (fi, free) = disk_map[i];
        if free == 0 {
            compacted_disk_map.push((i, fi));
            i += 1;
            continue;
        }
        let (fj, _) = disk_map[j];
        if free == fj {
            if !already_added_i {
                compacted_disk_map.push((i, fi));
            }
            compacted_disk_map.push((j, fj));
            j -= 1;
            i += 1;
            already_added_i = false;
        } else if free > fj {
            if !already_added_i {
                compacted_disk_map.push((i, fi));
                already_added_i = true;
            }
            compacted_disk_map.push((j, fj));
            disk_map[i] = (fi, free - fj);
            j -= 1;
        } else {
            // free < fj
            if !already_added_i {
                compacted_disk_map.push((i, fi));
            }
            compacted_disk_map.push((j, free));
            disk_map[j] = (fj - free, 0);
            i += 1;
            already_added_i = false;
        }
    }
    if j == i && !already_added_i {
        compacted_disk_map.push((i, disk_map[i].0));
    }
    return compacted_disk_map;
}

fn compact_disk_without_fragmentation(
    disk_map: &Vec<(usize, usize)>,
) -> Vec<(usize, usize, usize)> {
    let mut disk_map =
        disk_map
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut map, (index, &(file, free))| {
                map.push((index, file, free));
                map
            });
    fn find_gap(
        disk_map: &Vec<(usize, usize, usize)>,
        min_size: usize,
        to: usize,
    ) -> Option<(usize, usize)> {
        for (i, &(_, _, free)) in disk_map.iter().enumerate() {
            if i >= to {
                return None;
            }
            if free < min_size {
                continue;
            }
            return Some((i, free - min_size));
        }
        None
    }
    let mut i = disk_map.len() - 1;
    let mut last_id = None;
    loop {
        let (id, file, free) = disk_map[i];
        if let Some(last_id) = last_id {
            if id >= last_id {
                if i == 0 {
                    break;
                }
                i -= 1;
                continue;
            }
        }
        last_id = Some(id);
        if let Some((index, remaining_free)) = find_gap(&disk_map, file, i) {
            let old: &mut _ = disk_map.get_mut(index).unwrap();
            *old = ((*old).0, (*old).1, 0);
            disk_map[i] = (0, 0, file + free);
            disk_map.insert(index + 1, (id, file, remaining_free));
            i += 1;
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
    return disk_map;
}

fn calculate_checksum<T: IntoIterator<Item = (usize, usize)>>(disk: T) -> i64 {
    let mut checksum = 0i64;
    let mut position = 0i64;
    for (id, span) in disk {
        for _ in 0..span {
            checksum += position * (id as i64);
            position += 1;
        }
    }
    checksum
}

fn calculate_checksum_b<T: IntoIterator<Item = (usize, usize, usize)>>(disk: T) -> i64 {
    let mut checksum = 0i64;
    let mut position = 0i64;
    for (id, span, free) in disk {
        for _ in 0..span {
            checksum += position * (id as i64);
            position += 1;
        }
        position += free as i64;
    }
    checksum
}

impl AocTask for Day09 {
    fn solve_a(&self, contents: String) -> AocResult {
        let mut disk_map = parse_input(contents)?;
        let compacted_disk_map = compact_disk(&mut disk_map);

        let checksum = calculate_checksum(compacted_disk_map);

        Ok(checksum)
    }

    fn solve_b(&self, contents: String) -> AocResult {
        let disk_map = parse_input(contents)?;

        let compacted_disk_map = compact_disk_without_fragmentation(&disk_map);

        let checksum = calculate_checksum_b(compacted_disk_map);
        Ok(checksum)
    }
}
#[allow(dead_code)]
fn print_disk(disk: &Vec<(usize, usize, usize)>) {
    println!(
        "{}",
        disk.iter()
            .map(|&(id, span, free)| id.to_string().repeat(span as usize) + &".".repeat(free))
            .join("")
    )
}
#[allow(dead_code)]
fn print_simple_disk(disk: &Vec<(usize, usize)>) {
    print_disk(
        &disk
            .iter()
            .enumerate()
            .map(|(id, &(span, free))| (id, span, free))
            .collect::<Vec<_>>(),
    );
}

#[cfg(test)]
mod test_a {
    use super::*;

    #[test]
    fn test_sample() -> Result<(), String> {
        let result = Day09.solve_a(String::from("2333133121414131402"))?;

        assert_eq!(result, 1928);

        Ok(())
    }

    #[test]
    fn test_simple() -> Result<(), String> {
        let mut disk = parse_input(String::from("22314"))?;

        let compacted = compact_disk(&mut disk);

        assert_eq!(compacted, vec![(0, 2), (2, 2), (1, 3), (2, 1), (2, 1)]);

        let result = calculate_checksum(compacted);

        assert_eq!(result, 4 + 6 + 4 + 5 + 6 + 14 + 16);

        Ok(())
    }

    #[test]
    fn test_simple_2() -> Result<(), String> {
        let mut disk = parse_input(String::from("223"))?;

        let compacted = compact_disk(&mut disk);

        assert_eq!(compacted, vec![(0, 2), (1, 2), (1, 1)]);

        let result = calculate_checksum(compacted);

        assert_eq!(result, 1 * 2 + 1 * 3 + 1 * 4);

        Ok(())
    }

    #[test]
    fn test_simple_even() -> Result<(), String> {
        let mut disk = parse_input(String::from("2233"))?;

        let compacted = compact_disk(&mut disk);

        assert_eq!(compacted, vec![(0, 2), (1, 2), (1, 1)]);

        let result = calculate_checksum(compacted);

        assert_eq!(result, 1 * 2 + 1 * 3 + 1 * 4);

        Ok(())
    }
}
#[cfg(test)]
mod test_b {
    use super::*;

    #[test]
    fn test_simple() {
        let actual =
            compact_disk_without_fragmentation(&vec![(2, 3), (2, 3), (2, 2), (3, 1), (2, 0)]);
        let expected = vec![
            (0, 2, 0),
            (4, 2, 1),
            (1, 2, 0),
            (3, 3, 0),
            (2, 2, 2),
            (0, 0, 4),
            (0, 0, 2),
        ];

        assert_eq!(actual, expected);
    }
}

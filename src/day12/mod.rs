use std::collections::{BTreeSet, HashSet};

use crate::aoc_core::{AocResult, AocTask};

pub struct Day12;

type Position = (usize, usize);

impl AocTask for Day12 {
    fn solve_a(&self, contents: String) -> AocResult {
        let map: Vec<Vec<_>> = contents.lines().map(|l| l.chars().collect()).collect();

        let mut regions = Vec::new();

        let mut seen = HashSet::new();

        let mut queue = map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, _)| (y, x)))
            .collect::<HashSet<_>>();

        while !queue.is_empty() {
            let next = *queue.iter().next().unwrap();
            queue.remove(&next);
            if seen.contains(&next) {
                continue;
            }

            let mut region = HashSet::new();
            scan_for_region(&map, &mut region, next);

            for pos in &region {
                seen.insert(*pos);
            }
            regions.push(region);
        }
        // for region in regions {
        //     println!("{:?}", region);
        // }
        let mut result = 0i64;
        for region in regions {
            let m = measure_region(&region);
            result += m.area as i64 * m.perimeter as i64;
        }
        Ok(result)
    }

    fn solve_b(&self, contents: String) -> AocResult {
        let map: Vec<Vec<_>> = contents.lines().map(|l| l.chars().collect()).collect();

        let mut regions = Vec::new();

        let mut seen = HashSet::new();

        let mut queue = map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, _)| (y, x)))
            .collect::<HashSet<_>>();

        while !queue.is_empty() {
            let next = *queue.iter().next().unwrap();
            queue.remove(&next);
            if seen.contains(&next) {
                continue;
            }

            let mut region = HashSet::new();
            scan_for_region(&map, &mut region, next);

            for pos in &region {
                seen.insert(*pos);
            }
            regions.push(region);
        }
        // for region in regions {
        //     println!("{:?}", region);
        // }
        let mut result = 0i64;
        for region in regions {
            let m = measure_region_b(&region);
            // println!("{:?}", region);
            // println!("{:?}", m);
            result += m.area as i64 * m.sides as i64;
        }
        Ok(result)
    }
}

struct RegionMeasurement {
    area: usize,
    perimeter: u32,
}

fn measure_region(region: &HashSet<Position>) -> RegionMeasurement {
    let mut perimeter = 0;
    for &(y, x) in region {
        if y == 0 || !region.contains(&(y - 1, x)) {
            perimeter += 1;
        }
        if !region.contains(&(y, x + 1)) {
            perimeter += 1;
        }
        if !region.contains(&(y + 1, x)) {
            perimeter += 1;
        }
        if x == 0 || !region.contains(&(y, x - 1)) {
            perimeter += 1;
        }
    }
    RegionMeasurement {
        area: region.len(),
        perimeter,
    }
}

fn scan_for_region(map: &Vec<Vec<char>>, region: &mut HashSet<Position>, position: Position) {
    if region.contains(&position) {
        return;
    }
    region.insert(position);
    let (y, x) = position;

    let plant = map[y][x];

    if y > 0 && map[y - 1][x] == plant {
        scan_for_region(map, region, (y - 1, x));
    }
    if y < map.len() - 1 && map[y + 1][x] == plant {
        scan_for_region(map, region, (y + 1, x));
    }
    let row = &map[y];
    if x > 0 && row[x - 1] == plant {
        scan_for_region(map, region, (y, x - 1));
    }

    if x < row.len() - 1 && row[x + 1] == plant {
        scan_for_region(map, region, (y, x + 1));
    }
}

#[derive(Debug)]
struct RegionMeasurementB {
    area: usize,
    sides: u32,
}

fn measure_region_b(region: &HashSet<Position>) -> RegionMeasurementB {
    let mut hsides = BTreeSet::new();
    let mut vsides = BTreeSet::new();
    for &(y, x) in region {
        if y == 0 || !region.contains(&(y - 1, x)) {
            hsides.insert((y, x, 0));
        }
        if !region.contains(&(y, x + 1)) {
            vsides.insert((x + 1, y, 1));
        }
        if !region.contains(&(y + 1, x)) {
            hsides.insert((y + 1, x, 1));
        }
        if x == 0 || !region.contains(&(y, x - 1)) {
            vsides.insert((x, y, 0));
        }
    }
    let mut sides = 0u32;
    let mut last = None;
    for &(y, x, side) in hsides.iter() {
        match last {
            None => {
                sides += 1;
            }
            Some((ly, lx, lside)) => {
                if y != ly || x > lx + 1 || side != lside {
                    sides += 1;
                }
            }
        }
        last = Some((y, x, side));
    }
    let mut last = None;
    for (x, y, side) in vsides {
        match last {
            None => {
                sides += 1;
            }
            Some((lx, ly, lside)) => {
                if x != lx || y > ly + 1 || side != lside {
                    sides += 1;
                }
            }
        }
        last = Some((x, y, side));
    }
    RegionMeasurementB {
        area: region.len(),
        sides,
    }
}

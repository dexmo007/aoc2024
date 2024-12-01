use std::{collections::HashMap, fs, iter::zip, path::PathBuf};

fn read_file(this_file: &str, name: &str) -> String {
    let binding = PathBuf::from(this_file);
    let file_path = binding.parent().unwrap();
    let input_file_path = file_path.join(name);
    let contents = fs::read_to_string(input_file_path).expect("Input file should be there");

    return contents;
}

fn run_a() -> Result<(), Box<dyn std::error::Error>> {
    let contents = read_file(file!(), "input.txt");
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in contents.lines() {
        let values: Vec<&str> = line.trim().split_whitespace().collect();
        left.push(values[0].parse::<i32>()?);
        right.push(values[1].parse::<i32>()?);
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

fn run_b() -> Result<(), Box<dyn std::error::Error>> {
    let contents = read_file(file!(), "input.txt");
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in contents.lines() {
        let values: Vec<&str> = line.trim().split_whitespace().collect();
        left.push(values[0].parse::<i32>()?);
        right.push(values[1].parse::<i32>()?);
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

fn main() {
    run_b().expect("Internal error")
}

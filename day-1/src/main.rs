use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn main() -> io::Result<()> {
    let file_path = "input";

    let (mut col1, mut col2) = parse_file_to_lists(file_path)?;

    let start = Instant::now();
    let distance = calculate_distance(&mut col1, &mut col2);
    println!("{distance}");
    let duration = start.elapsed();
    println!("Function took {:?} to execute", duration);

    let start = Instant::now();
    let similarity = calculate_similarity(&mut col1, &mut col2);
    println!("{similarity}");
    let duration = start.elapsed();
    println!("Function took {:?} to execute", duration);

    Ok(())
}

fn parse_file_to_lists(file_path: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if let (Some(first), Some(second)) = (parts.get(0), parts.get(1)) {
            col1.push(first.parse().unwrap_or_default());
            col2.push(second.parse().unwrap_or_default());
        }
    }

    Ok((col1, col2))
}

fn calculate_distance(col1: &mut Vec<i32>, col2: &mut Vec<i32>) -> i32 {
    let mut acc = 0;

    col1.par_sort();
    col2.par_sort();

    for (index, value) in col1.iter().enumerate() {
        acc += (value - col2.get(index).unwrap()).abs();
    }

    return acc;
}

fn calculate_similarity(col1: &mut Vec<i32>, col2: &mut Vec<i32>) -> i32 {
    let mut acc: i32 = 0;

    let default_value = 0;
    let mut score_map: HashMap<i32, i32> = HashMap::new();

    for key in col1 {
        score_map
            .entry(*key)
            .and_modify(|value| *value += 1)
            .or_insert(default_value);
    }

    for key in col2 {
        score_map.entry(*key).and_modify(|value| *value += 1);
    }

    for (key, value) in score_map {
        acc += key * value;
    }

    return acc;
}

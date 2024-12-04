use std::collections::HashMap;
use std::fs;
use std::io::{self};
use std::time::Instant;

fn main() -> io::Result<()> {
    let mut reports = parse_file_to_lists("input-edge");

    let start = Instant::now();
    let safe_levels = validate_reports(&mut reports.unwrap());
    println!("There is {:?} safe levels", safe_levels);
    let duration = start.elapsed();
    println!("Function took {:?} to execute", duration);

    Ok(())
}

fn parse_file_to_lists(file_path: &str) -> io::Result<Vec<Vec<i32>>> {
    let file_content = fs::read_to_string(file_path)?;

    let parse_content = file_content
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    return Ok(parse_content);
}

fn validate_reports(reports: &mut Vec<Vec<i32>>) -> i32 {
    let mut safe_levels = 0;
    println!("{:?}", reports);
    // adjacent level differ by min 1 and max 3
    // either incresing or decrecing

    for (i, report) in reports.iter().enumerate() {
        let mut unsafe_level = 0;

        println!("Level {i}");
        for (j, level) in report.iter().enumerate() {
            println!("{level}");
            if j < report.len() - 1 {
                let next_level = report.get(j + 1).unwrap();

                if j < report.len() + 3 {
                    if is_both_ascending_and_descending(&report[j..j + 2]) {
                        println!("both in and dec");

                        unsafe_level += 1;
                        continue;
                    }
                }
                let abs_diff = next_level.abs_diff(*level);

                if abs_diff == 0 {
                    println!("equal 0");
                    unsafe_level += 1;
                    continue;
                }
                if abs_diff > 3 {
                    if j == 0 || j == report.len() - 2 {
                        println!("edge > 3");
                        unsafe_level += 1;
                        continue;
                    } else {
                        if j != 0 {
                            let abs_diff_gap = (report.get(j - 1).unwrap() - next_level).abs();
                            if (abs_diff_gap >= 1 && abs_diff_gap <= 3) {
                                println!("check if gap can be removed");
                                unsafe_level += 1;
                                continue;
                            } else {
                                println!("do not respect condition");
                                break;
                            }
                        }
                    }
                }
            } else if unsafe_level <= 1 {
                println!("{i} is safe");
                safe_levels += 1;
            } else {
                println!("{unsafe_level} unsafe_level")
            }
        }
    }

    return safe_levels;
}

fn is_both_ascending_and_descending(vec: &[i32]) -> bool {
    if vec.len() != 3 {
        return false; // Ensure the vector always has exactly 3 elements
    }

    let (a, b, c) = (vec[0], vec[1], vec[2]);

    (b > a && b > c) || (b < a && b < c)
}

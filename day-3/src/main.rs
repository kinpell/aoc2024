use regex::Regex;
use std::fs;
use std::time::Instant;

fn main() {
    let input_to_handle = parse_file_to_string("input");

    // Part 1
    // Start timer
    let start = Instant::now();
    // Execute Part1
    let total = extract_and_compute_mul_part1(&input_to_handle);
    // Stop Timer
    let duration = start.elapsed();
    println!("The Total for Part 1 is --> {total}");
    println!("Part 1 took {:?} to execute", duration);

    // Part 2
    // Start Timer
    let start = Instant::now();
    // Execute Part2
    let total_part2 = extract_and_compute_mul_part2(&input_to_handle);
    // Stop Timer
    let duration = start.elapsed();
    println!("The Total for Part 2 is --> {total_part2}");
    println!("Part 2 took {:?} to execute", duration);
}

fn parse_file_to_string(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Should have read the file");
}

fn extract_and_compute_mul_part1(input: &String) -> i32 {
    let total = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
        .unwrap()
        .captures_iter(&input)
        .map(|mul| mul[1].parse::<i32>().unwrap() * mul[2].parse::<i32>().unwrap())
        .sum();

    return total;
}

fn extract_and_compute_mul_part2(input: &String) -> i64 {
    let regex = Regex::new(r"do\(\)|don't\(\)").unwrap();
    let mut last_end = 0;
    let mut good_muls = Vec::new();
    let mut last_command: String = "do()".to_string();

    for line in regex.find_iter(input) {
        let command: String = line.as_str().to_string();
        // println!("======");
        // println!("on going Command: {command}");
        // println!("LAST Command: {last_command}");

        if last_end != line.start() && last_command == "do()" {
            let good_mul: String = input[last_end..line.start()].to_string();
            good_muls.push(good_mul);
        }
        last_command = command;
        last_end = line.end();
    }

    // Push any remaining text after the last match
    if last_end < input.len() && last_command == "do()" {
        let last_good = input[last_end..].to_string();
        good_muls.push(last_good);
    }

    let mut total: i64 = 0;
    for good_mul in good_muls {
        total += extract_and_compute_mul_part1(&good_mul) as i64;
    }

    return total;
}

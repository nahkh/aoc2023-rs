use crate::input_files::get_current_day;
use crate::input_files::read_content;
use std::path::Path;

fn calculate_calibration_value(row: &str) -> u32 {
    let mut first_value = 10;
    let mut last_value = 10;
    for c in row.chars() {
        if let Some(n) = c.to_digit(10) {
            if first_value > 9 {
                first_value = n;
            }
            last_value = n;
        }
    }

    10 * first_value + last_value
}

fn find_first_digit(row: &str) -> u32 {
    let mut buffer = row.to_string();
    while buffer.len() > 0 {
        if buffer.starts_with("one") {
            return 1;
        }
        if buffer.starts_with("two") {
            return 2;
        }
        if buffer.starts_with("three") {
            return 3;
        }
        if buffer.starts_with("four") {
            return 4;
        }
        if buffer.starts_with("five") {
            return 5;
        }
        if buffer.starts_with("six") {
            return 6;
        }
        if buffer.starts_with("seven") {
            return 7;
        }
        if buffer.starts_with("eight") {
            return 8;
        }
        if buffer.starts_with("nine") {
            return 9;
        }

        let first_char = buffer.chars().next().unwrap();
        if first_char.is_digit(10) {
            return first_char.to_digit(10).unwrap();
        }
        buffer = buffer[1..].to_string();
    }

    0
}

fn find_last_digit(row: &str) -> u32 {
    let mut buffer = row.to_string();
    while buffer.len() > 0 {
        if buffer.ends_with("one") {
            return 1;
        }
        if buffer.ends_with("two") {
            return 2;
        }
        if buffer.ends_with("three") {
            return 3;
        }
        if buffer.ends_with("four") {
            return 4;
        }
        if buffer.ends_with("five") {
            return 5;
        }
        if buffer.ends_with("six") {
            return 6;
        }
        if buffer.ends_with("seven") {
            return 7;
        }
        if buffer.ends_with("eight") {
            return 8;
        }
        if buffer.ends_with("nine") {
            return 9;
        }

        let last_char = buffer[buffer.len() - 1..].chars().next().unwrap();
        if last_char.is_digit(10) {
            return last_char.to_digit(10).unwrap();
        }
        buffer = buffer[..buffer.len() - 1].to_string();
    }

    0
}

fn calculate_calibration_value_part2(row: &str) -> u32 {
    10 * find_first_digit(row) + find_last_digit(row)
}

fn part1(content: &str) {
    let mut total = 0;
    for row in content.lines() {
        total += calculate_calibration_value(&row);
    }
    println!("Part 1: Sum of all calibration values is {}", total);
}

fn part2(content: &str) {
    let mut total = 0;
    for row in content.lines() {
        total += calculate_calibration_value_part2(&row);
    }
    println!("Part 2: Sum of all calibration values is {}", total);
}

pub fn execute() {
    let current_day = get_current_day(file!());
    let input_file = format!("data\\day{:02}.txt", current_day);
    if !Path::new(&input_file).exists() {
        println!("No input file for day {}", current_day);
        return;
    }
    let content = read_content(&input_file);

    part1(&content);
    part2(&content);
}

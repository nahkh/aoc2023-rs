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



fn part1(content: &str) {
    let mut total = 0;
    for row in content.lines() {
        total += calculate_calibration_value(&row);
    }
    println!("Part 1: Sum of all calibration values is {}", total);
}

fn part2(content: &str) {
    println!("Part 2 not implemented");
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

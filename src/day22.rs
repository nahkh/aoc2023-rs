use crate::input_files::get_current_day;
use crate::input_files::read_content;
use std::path::Path;

fn part1(_content: &str) {
    println!("Part 1 not implemented");
}

fn part2(_content: &str) {
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

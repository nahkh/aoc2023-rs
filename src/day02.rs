use crate::input_files::get_current_day;
use crate::input_files::read_content;
use std::path::Path;
use regex::Regex;

#[derive(Debug, PartialEq)]
struct Observation {
    red: u32,
    green: u32,
    blue: u32,
}

impl Observation {
    fn parse(content: &str) -> Observation {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for part in content.trim().split(',') {
            let re = Regex::new(r"\s*(\d+) (\w+)").unwrap();
            let captures = re.captures(part).unwrap();
            let number = captures.get(1).unwrap().as_str().parse::<u32>().ok().unwrap();
            let color = captures.get(2).unwrap().as_str();
            match color {
                "red" => red = number,
                "green" => green = number,
                "blue" => blue = number,
                _ => panic!("Unparseable color {} in {}", color, part)
            }
        }
        Observation {red, green, blue}
    }

    fn is_possible(&self, actual_red: u32, actual_green: u32, actual_blue: u32) -> bool {
        actual_red >= self.red && actual_green >= self.green && actual_blue >= self.blue
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    observations: Vec<Observation>
}

impl Game {
    fn parse(content: &str) -> Game {
        let mut parts = content.trim().split(':');
        let id = parts.next().unwrap()[5..].parse::<u32>().ok().unwrap();
        let mut observations = Vec::new();
        for observation_part in parts.next().unwrap().split(';') {
            observations.push(Observation::parse(observation_part));
        }

        Game {id, observations}
    }

    fn is_possible(&self, actual_red: u32, actual_green: u32, actual_blue: u32) -> bool {
        for observation in self.observations.iter() {
            if !observation.is_possible(actual_red, actual_green, actual_blue) {
                return false;
            }
        }

        true
    }
}

fn part1(content: &str) {
    let mut total = 0;
    for line in content.lines() {
        let game = Game::parse(line);
        if game.is_possible(12, 13, 14) {
            total += game.id;
        }
    }
    println!("Part 1: The sum of ids of possible games is {}", total);    
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

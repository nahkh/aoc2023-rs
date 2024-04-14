use crate::input_files::get_current_day;
use crate::input_files::read_content;
use std::path::Path;


fn distance_traveled(duration: u64, charge_time: u64) -> u64 {
    if charge_time == 0 {
        return 0;
    }
    if charge_time >= duration {
        return 0;
    }
    
    charge_time * (duration - charge_time)
}

#[derive(Debug, PartialEq)]
struct Race {
    duration: u64,
    record: u64,
}

impl Race {
    fn new(duration: u64, record: u64) -> Race {
        Race {
            duration,
            record,
        }
    }

    fn possible_winning_strategies(&self) -> u64 {
        let mut output = 0;

        for i in 1..self.duration {
            if distance_traveled(self.duration, i) > self.record {
                output += 1;
            }
        }


        output
    }
}

fn parse_input(content: &str) -> Vec<Race> {
    let mut races = Vec::new();
    let lines = content.lines().collect::<Vec<&str>>(); 
    let times = lines.get(0).expect("There should be at least one line in the input").split_whitespace().collect::<Vec<&str>>();
    let distances = lines.get(1).expect("There should be at least two lines in the input").split_whitespace().collect::<Vec<&str>>();
    for i in 1..times.len() {
        races.push(Race::new(times.get(i).unwrap().parse::<u64>().ok().unwrap(), distances.get(i).unwrap().parse::<u64>().ok().unwrap()));
    }

    races
}

fn number_of_digits(value: u64) -> u64 {
    value.to_string().len().try_into().unwrap()
}

fn concat_digits(a: u64, b: u64) -> u64 {
    a * (10^number_of_digits(a)) + b
}


fn part1(content: &str) {
   let mut output = 1;
   for race in parse_input(content) {
     output *= race.possible_winning_strategies();
   }

   println!("Part 1: {} possible solutions", output);
}

fn part2(content: &str) {
   let mut time = 0;
   let mut record = 0;
   for race in parse_input(content) {
     time = concat_digits(time, race.duration);
     record = concat_digits(record, race.record);
   }

   let combined_race = Race::new(48938595, 296192812361391);
   println!("{:?}", combined_race);

   let output = combined_race.possible_winning_strategies();

   println!("Part 2: {} possible solutions", output);
}

pub fn execute() {
    let current_day = get_current_day(file!());
    let input_file = format!("data{}day{:02}.txt", std::path::MAIN_SEPARATOR_STR, current_day);
    if !Path::new(&input_file).exists() {
        println!("No input file for day {}", current_day);
        return;
    }
    let content = read_content(&input_file);

    part1(&content);
    part2(&content);
}

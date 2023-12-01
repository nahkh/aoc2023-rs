use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod input_files;
mod position;
mod position3;

fn execute_day(day: usize) {
    println!("Day {}", day);
    match day {
        1 => day01::execute(),
        2 => day02::execute(),
        3 => day03::execute(),
        4 => day04::execute(),
        5 => day05::execute(),
        6 => day06::execute(),
        7 => day07::execute(),
        8 => day08::execute(),
        9 => day09::execute(),
        10 => day10::execute(),
        11 => day11::execute(),
        12 => day12::execute(),
        13 => day13::execute(),
        14 => day14::execute(),
        15 => day15::execute(),
        16 => day16::execute(),
        17 => day17::execute(),
        18 => day18::execute(),
        19 => day19::execute(),
        20 => day20::execute(),
        21 => day21::execute(),
        22 => day22::execute(),
        23 => day23::execute(),
        24 => day24::execute(),
        25 => day25::execute(),
        _ => panic!("Day {} not implemented", day),
    }
    println!("");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        for i in 1..26 {
            execute_day(i);
        }
    } else {
        let day_arg = &args[1];
        let target_day = day_arg.parse::<usize>().unwrap();
        execute_day(target_day);
    }
}

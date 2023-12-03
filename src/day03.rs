use crate::input_files::get_current_day;
use crate::input_files::read_content;
use crate::position::Position;
use std::collections::HashSet;
use std::path::Path;

#[derive(Debug, Hash, PartialEq, Clone)]
struct SchematicNumber {
    id: u32,
    positions: Vec<Position>,
}

impl SchematicNumber {
    fn new(id: u32, positions: Vec<Position>) -> SchematicNumber {
        SchematicNumber { id, positions }
    }
}

struct Component {
    symbol: char,
    position: Position,
    schematic_numbers: Vec<SchematicNumber>,
}

impl Component {
    fn new(symbol: char, position: Position, schematic_numbers: Vec<SchematicNumber>) -> Component {
        Component {
            symbol,
            position,
            schematic_numbers,
        }
    }

    fn is_gear(&self) -> bool {
        self.symbol == '*' && self.schematic_numbers.len() == 2
    }

    fn get_gear_ratio(&self) -> u32 {
        if !self.is_gear() {
            return 0;
        }

        self.schematic_numbers.get(0).unwrap().id * self.schematic_numbers.get(1).unwrap().id
    }
}

fn find_potential_numbers(content: &str) -> Vec<SchematicNumber> {
    let mut output = Vec::new();
    let mut current_number = 0;
    let mut current_positions = Vec::new();
    for (y, line) in content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                current_positions.push(Position::new(x.try_into().unwrap(), y.try_into().unwrap()));
                current_number = current_number * 10 + c.to_digit(10).unwrap();
            } else {
                if current_number > 0 {
                    output.push(SchematicNumber::new(
                        current_number,
                        current_positions.clone(),
                    ));
                    current_number = 0;
                    current_positions.clear();
                }
            }
        }
        if current_number > 0 {
            output.push(SchematicNumber::new(
                current_number,
                current_positions.clone(),
            ));
            current_number = 0;
            current_positions.clear();
        }
    }

    output
}

fn find_symbols(content: &str) -> HashSet<Position> {
    let mut output = HashSet::new();
    for (y, line) in content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if !(c == '.' || c == '\n' || c.is_digit(10)) {
                let position = Position::new(x.try_into().unwrap(), y.try_into().unwrap());
                output.insert(position);
            }
        }
    }

    output
}

fn find_adjacent_schematic_numbers(
    position: Position,
    schematic_numbers: &Vec<SchematicNumber>,
) -> Vec<SchematicNumber> {
    let mut output = Vec::new();
    for schematic_number in schematic_numbers {
        for neighbor in position.all_neighbors() {
            if schematic_number.positions.contains(&neighbor) {
                output.push(schematic_number.clone());
                break;
            }
        }
    }
    output
}

fn find_components(content: &str) -> Vec<Component> {
    let mut output = Vec::new();
    let schematic_numbers = find_schematic_numbers(content);
    for (y, line) in content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if !(c == '.' || c == '\n' || c.is_digit(10)) {
                let position = Position::new(x.try_into().unwrap(), y.try_into().unwrap());
                let adjacent_numbers =
                    find_adjacent_schematic_numbers(position, &schematic_numbers);
                output.push(Component::new(c, position, adjacent_numbers));
            }
        }
    }

    output
}

fn find_schematic_numbers(content: &str) -> Vec<SchematicNumber> {
    let engine_symbols = find_symbols(content);
    let mut output = Vec::new();
    for potential_number in find_potential_numbers(content) {
        let mut number_is_valid = false;
        for position in &potential_number.positions {
            for neighbor in &position.all_neighbors() {
                if engine_symbols.contains(&neighbor) {
                    number_is_valid = true;
                }
            }
        }
        if number_is_valid {
            output.push(potential_number.clone());
        }
    }

    output
}

fn find_sum_of_schematic_numbers(content: &str) -> u32 {
    let mut sum = 0;
    for schematic_number in find_schematic_numbers(content) {
        sum += schematic_number.id;
    }

    sum
}

fn find_gear_ratios(content: &str) -> u32 {
    let mut sum = 0;
    for component in find_components(content) {
        sum += component.get_gear_ratio();
    }

    sum
}

fn part1(content: &str) {
    let sum = find_sum_of_schematic_numbers(content);

    println!(
        "Part 1: The sum of part numbers in the engine schematic is {}",
        sum
    );
}

fn part2(content: &str) {
    let sum = find_gear_ratios(content);

    println!(
        "Part 2: The sum of gear ratios in the engine schematic is {}",
        sum
    );
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schematic_number_sum() {
        let test_content = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..\n";

        assert_eq!(find_sum_of_schematic_numbers(&test_content), 4361);
    }

    #[test]
    fn test_gear_ratio_sum() {
        let test_content = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..\n";

        assert_eq!(find_gear_ratios(&test_content), 467835);
    }
}

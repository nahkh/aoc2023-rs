use crate::input_files::get_current_day;
use crate::input_files::read_content;
use std::collections::HashSet;
use std::path::Path;

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    actual_numbers: HashSet<u32>,
    instance_count: u64,
}

impl Card {
    fn parse_line(line: &str) -> Card {
        let mut parts = line.trim().split(':');
        let id = read_id(parts.next().unwrap());
        let mut number_part = parts.next().unwrap().split('|');
        let winning_numbers = read_numbers(number_part.next().unwrap());
        let actual_numbers = read_numbers(number_part.next().unwrap());

        Card {
            id,
            winning_numbers,
            actual_numbers,
            instance_count: 1,
        }
    }

    fn get_value(&self) -> u32 {
        match self.get_match_count() {
            0 => 0,
            1 => 1,
            n => 2 << (n - 2),
        }
    }

    fn get_match_count(&self) -> u32 {
        self.winning_numbers
            .intersection(&self.actual_numbers)
            .map(|_x| 1)
            .sum()
    }
}

fn read_id(data: &str) -> u32 {
    data[5..].trim().parse::<u32>().ok().unwrap()
}

fn read_numbers(data: &str) -> HashSet<u32> {
    let mut output = HashSet::new();
    for part in data.split(' ') {
        if let Ok(value) = part.trim().parse::<u32>() {
            output.insert(value);
        }
    }

    output
}

fn get_points(content: &str) -> u32 {
    let mut points = 0;
    for line in content.lines() {
        let card = Card::parse_line(line);
        points += card.get_value();
    }

    points
}

fn count_card_instances(content: &str) -> u64 {
    let mut cards = Vec::new();
    for line in content.lines() {
        cards.push(Card::parse_line(line));
    }

    for i in 0..cards.len() {
        let card = &cards[i];
        let matches: usize = card.get_match_count().try_into().unwrap();
        let instances = card.instance_count;
        for j in (i + 1)..(i + 1 + matches) {
            if let Some(next_card) = cards.get_mut(j) {
                next_card.instance_count += instances;
            }
        }
    }

    let mut card_instances = 0;
    for card in cards {
        card_instances += card.instance_count;
    }

    card_instances
}

fn part1(content: &str) {
    let points = get_points(content);
    println!("Part 1: The cards are worth {} points", points);
}

fn part2(content: &str) {
    let card_instances = count_card_instances(content);
    println!(
        "Part 2: The total number of scratchcards is {}",
        card_instances
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
    fn test_get_points() {
        let test_content = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(get_points(&test_content), 13);
    }

    #[test]
    fn test_count_card_instances() {
        let test_content = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(count_card_instances(&test_content), 30);
    }
}

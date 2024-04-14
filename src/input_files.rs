use regex::Regex;

pub fn read_content(filename: &str) -> String {
    std::fs::read_to_string(filename).expect("Should have been able to read file")
}

pub fn get_current_day(code_file: &str) -> i32 {
    let re = Regex::new(format!(r"src{}day(\d\d).rs", std::path::MAIN_SEPARATOR_STR).as_str()).unwrap();
    let captures = re.captures(code_file).unwrap();

    captures
        .get(1)
        .unwrap()
        .as_str()
        .parse::<i32>()
        .ok()
        .unwrap()
}

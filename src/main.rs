use std::fs;

use advent_of_code::days;
use advent_of_code::days::day_2::Pull;
use advent_of_code::utilities;

fn main() {
    // day_1();
    // day_2();
    // day_3();
    day_4();
}

fn day_1() {
    let file_path = "src/input_files/day_1.txt";
    let lines = utilities::read_file_by_line(file_path)
        .unwrap()
        .iter()
        .map(|s| String::from(s))
        .collect();
    let result = days::day_1::sum_all_lines(lines);
    println!("Result for day 1 is {}", result);
}

fn day_2() {
    let lines = utilities::read_file_by_line("src/input_files/day_2.txt").unwrap();
    let max_pull = Pull::new(12, 13, 14);
    let mut result = days::day_2::sum_valid_ids_from_lines(&lines, &max_pull);
    println!("Result for day 2 part 1 is {}", result);
    result = days::day_2::power_from_lines(&lines);
    println!("Result for day 2 part 2 is {}", result);
}

fn day_3() {
    let content = fs::read_to_string("src/input_files/day_3.txt").unwrap();
    let engine = days::day_3::Engine::from_string(content);
    let part1 = engine.sum_valid_nums();
    println!("Result for day 3 part 1 is {}", part1);
    println!("Result for day 3 part 2 is {}", engine.sum_gear_ratios());
}

fn day_4() {
    let lines = utilities::read_file_by_line("src/input_files/day_4.txt").unwrap();
    let result = days::day_4::sum_all_cards(&lines);
    println!("Result for day 4 part 1 is {}", result);
    let result = days::day_4::sum_all_cards_with_new_rules(&lines);
    println!("Result for day 4 part 2 is {}", result);
}

use advent_of_code::utilities;
use advent_of_code::days;
use advent_of_code::days::day_2::Pull;

fn main() {
    day_1();
    day_2();
}

fn day_1() {
    let file_path = "src/input_files/day_1.txt";
    let lines = utilities::read_file_by_line(file_path).unwrap().iter().map(|s| String::from(s)).collect();
    let result = days::day_1::sum_all_lines(lines);
    println!("Result for day 1 is {}", result);
}

fn day_2() {
    let lines = utilities::read_file_by_line("src/input_files/day_2.txt").unwrap();
    let max_pull = Pull::new(
        12,
        13,
        14
    );
    let mut result = days::day_2::sum_valid_ids_from_lines(&lines, &max_pull );
    println!("Result for day 2 part 1 is {}", result);
    result = days::day_2::power_from_lines(&lines);
    println!("Result for day 2 part 2 is {}", result);




}

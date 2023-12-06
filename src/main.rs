use advent_of_code::utilities;
use advent_of_code::days;

fn main() {
    day_1();
}

fn day_1() {
    let file_path = "src/input_files/day_1.txt";
    let lines = utilities::read_file_by_line(file_path).unwrap();
    let result = days::day_1::sum_all_lines(lines);
    println!("Result for day 1 is {}", result);
}

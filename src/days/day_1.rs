pub trait StringHelpers {
    fn get_first_last_number(&self) -> u32;
    fn replace_string_numbers(&self) -> String;
}

impl StringHelpers for str {
    fn get_first_last_number(&self) -> u32 {
        let string_with_digits = self.replace_string_numbers();
        let digits: Vec<char> = string_with_digits.chars().filter(|c| c.is_digit(10)).collect();
        match digits.len() {
            0 => 0,
            1 => format!("{}{}", digits[0], digits[0]).parse().unwrap(),
            _ => format!("{}{}", digits[0], digits[digits.len() - 1]).parse().unwrap(),
        }
    }

    fn replace_string_numbers(&self) -> String {
        let number_strings = vec![
            ("one", "o1ne"),
            ("two", "t2wo"),
            ("three", "thr3ee"),
            ("four", "f4our"),
            ("five", "f5ive"),
            ("six", "s6ix"),
            ("seven", "se7ven"),
            ("eight", "ei8ght"),
            ("nine", "ni9ne"),
        ];

        let mut copy = String::from(self);
        for num_pair in number_strings {
            copy = copy.replace(num_pair.0, num_pair.1);
        }
        copy
    }
}


pub fn sum_all_lines(lines: Vec<String>) -> u32 {
    let sum = lines.iter().fold(0, |accumulator, s| {
        let line_sum = s.get_first_last_number();
        accumulator + line_sum
    });
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_first_last_number_should_work() {
        let s = "abc123";
        assert_eq!(s.get_first_last_number(), 13);
    }


    #[test]
    fn day_1_works_with_multiple_lines() {
        let test_val = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        let lines = test_val.lines().map(|s| s.to_string()).collect();
        assert_eq!(sum_all_lines(lines), 142);
    }

    #[test]
    fn day_1_works_with_words() {
        let test_val = "\
        two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let lines = test_val.lines().map(|s| s.to_string()).collect();
        assert_eq!(sum_all_lines(lines), 281);
    }
}

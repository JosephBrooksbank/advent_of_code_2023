use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug)]
pub struct Engine {
    schematic: Vec<Vec<char>>,
    gears: HashMap<Pos, Vec<NumberInLine>>,
}

#[derive(Debug, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, EnumIter, PartialEq)]
enum Direction {
    LUp,
    Up,
    RUp,
    L,
    R,
    LDown,
    Down,
    RDown,
    Current,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Current
    }
}

#[derive(Debug)]
struct NumberInLine {
    value: String,
    x: usize,
}

fn create_number_in_line(buffer: &mut Vec<&char>, index: usize) -> NumberInLine {
    let value = buffer.iter().cloned().collect::<String>();
    let len = value.len();
    buffer.clear();
    NumberInLine {
        value,
        x: index - len,
    }
}

fn parse_line_into_numbers(line: &Vec<char>) -> Vec<NumberInLine> {
    let mut buffer = vec![];
    let mut numbers = vec![];

    for (index, c) in line.into_iter().enumerate() {
        if c.is_digit(10) {
            buffer.push(c)
        } else if !buffer.is_empty() {
            numbers.push(create_number_in_line(&mut buffer, index));
        }
    }
    if !buffer.is_empty() {
        numbers.push(create_number_in_line(&mut buffer, line.len()));
    }
    numbers
}

impl Engine {
    pub fn from_string(input: String) -> Engine {
        let schematic: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        Engine {
            schematic,
            gears: HashMap::new(),
        }
    }

    pub fn sum_valid_nums(&self) -> usize {
        let mut sum: usize = 0;
        for (i, line) in self.schematic.iter().enumerate() {
            let nums = parse_line_into_numbers(line);
            for num_in_line in nums {
                if self.test_around_number(num_in_line.x, i, &num_in_line.value) {
                    // print!(" {} ", num_in_line.value);
                    sum += num_in_line.value.parse::<usize>().unwrap();
                } else {
                    print!(" {} ", num_in_line.value);
                }
            }
            println!();
        }
        sum
    }

    fn get_surrounding_number<'a>(
        &'a self,
        pos: Pos,
        val: &'a String,
    ) -> impl Iterator<Item = char> + 'a {
        val.chars()
            .enumerate()
            .flat_map(move |(index, _)| self.get_surrounding(pos.x + index, pos.y).into_iter())
    }

    fn test_around_number(&self, x: usize, y: usize, val: &String) -> bool {
        let pos = Pos { x, y };
        self.get_surrounding_number(pos, val)
            .any(|c| (!c.is_digit(10) && c != '.'))
    }

    fn get_coord(&self, x: usize, y: usize, dir: Direction) -> char {
        let mut x: i128 = x.try_into().unwrap();
        let mut y: i128 = y.try_into().unwrap();

        match dir {
            Direction::LUp => {
                x -= 1;
                y -= 1
            }
            Direction::Up => y -= 1,
            Direction::RUp => {
                x += 1;
                y -= 1
            }
            Direction::L => x -= 1,
            Direction::R => x += 1,
            Direction::LDown => {
                x -= 1;
                y += 1
            }
            Direction::RDown => {
                x += 1;
                y += 1
            }
            Direction::Down => y += 1,
            Direction::Current => (),
        }

        if x < 0 || y < 0 {
            return '.';
        }

        let x: usize = x.unsigned_abs().try_into().unwrap();
        let y: usize = y.unsigned_abs().try_into().unwrap();

        let row = match self.schematic.get(y) {
            Some(val) => val,
            None => return '.',
        };

        match row.get(x) {
            Some(val) => val.clone(),
            None => '.',
        }
    }

    fn get_surrounding(&self, x: usize, y: usize) -> Vec<char> {
        let mut surrounding = vec![];
        for current_dir in Direction::iter() {
            match current_dir {
                Direction::Current => (),
                dir => surrounding.push(self.get_coord(x, y, dir)),
            }
        }
        surrounding
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_engine() -> Engine {
        let input = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
........50"
            .to_string();
        let engine = Engine::from_string(input);
        engine
    }

    #[test]
    fn part_1_should_work() {
        let engine = gen_engine();
        let sum = engine.sum_valid_nums();
        assert_eq!(sum, 4361);
    }

    #[test]
    fn test_surrounding_should_work() {
        let engine = gen_engine();
        assert_eq!(engine.test_around_number(5, 0, &"114".to_string()), false);
        assert_eq!(engine.test_around_number(2, 2, &"35".to_string()), true);
    }

    #[test]
    fn numbers_from_line_should_work() {
        let line = "467..114..".chars().collect();
        let numbs = parse_line_into_numbers(&line);
        assert_eq!(numbs.len(), 2);
        assert_eq!(numbs[0].value, "467");
        assert_eq!(numbs[1].x, 5);
    }

    #[test]
    fn get_surrounding_should_work() {
        let engine = gen_engine();
        assert_eq!(
            engine.get_surrounding(2, 2),
            vec!['.', '.', '*', '.', '5', '.', '.', '.']
        )
    }

    #[test]
    fn get_coord_should_work() {
        let engine = gen_engine();
        assert_eq!(engine.get_coord(1, 0, Direction::Current), '6');
        assert_eq!(engine.get_coord(1, 0, Direction::R), '7')
    }
}

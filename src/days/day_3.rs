use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug)]
pub struct Engine {
    schematic: Vec<Vec<char>>,
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

fn get_numbers_from_line(line: &Vec<char>) -> Vec<NumberInLine> {
    let mut buffer = vec![];
    let mut out = vec![];

    for (index, c) in line.into_iter().enumerate() {
        if c.is_digit(10) {
            buffer.push(c)
        } else {
            if buffer.len() > 0 {
                let val = buffer.iter().cloned().collect::<String>();
                let len = val.len();
                out.push(NumberInLine {
                    value: val,
                    x: index - len,
                });
                buffer.clear();
            }
        }
    }
    if buffer.len() > 0 {
        let val = buffer.iter().cloned().collect::<String>();
        let len = val.len();
        out.push(NumberInLine {
            value: val,
            x: line.len() - len,
        });
        buffer.clear();
    }

    out
}

impl Engine {
    pub fn from_string(input: String) -> Engine {
        let mut schematic: Vec<Vec<char>> = vec![];
        let lines = input.lines();
        for line in lines {
            let current_line = line.chars().collect();
            schematic.push(current_line);
        }

        Engine { schematic }
    }

    pub fn sum_valid_nums(&self) -> usize {
        let mut sum: usize = 0;
        for (i, line) in self.schematic.iter().enumerate() {
            let nums = get_numbers_from_line(line);
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

    fn test_around_number(&self, x: usize, y: usize, val: &String) -> bool {
        for (index, _) in val.chars().enumerate() {
            let surrounding = self
                .get_surrounding(x + index, y)
                .iter()
                .any(|c| (!c.is_digit(10) && c != &'.'));
            if surrounding {
                return surrounding;
            }
        }
        return false;
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
        let numbs = get_numbers_from_line(&line);
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

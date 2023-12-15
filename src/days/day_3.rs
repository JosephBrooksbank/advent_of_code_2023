use std::collections::HashMap;
use std::fmt::Debug;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone)]
struct OutOfBoundsError;

#[derive(Debug)]
pub struct Engine {
    schematic: Vec<Vec<char>>,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
struct CharPos {
    c: char,
    pos: Pos,
}

impl CharPos {
    fn default() -> CharPos {
        CharPos {
            c: '.',
            pos: Pos { x: 0, y: 0 },
        }
    }
}

impl Default for CharPos {
    fn default() -> Self {
        Self::default()
    }
}

impl Clone for CharPos {
    fn clone(&self) -> Self {
        CharPos {
            c: self.c.clone(),
            pos: Pos {
                x: self.pos.x.clone(),
                y: self.pos.y.clone(),
            },
        }
    }
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

enum PositionInNumber {
    Start,
    Middle,
    End,
    Single,
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
    pub fn sum_gear_ratios(&self) -> usize {
        let gears = self.find_gears();
        let mut sum = 0;
        for (_pos, gear) in gears {
            if gear.len() == 2 {
                sum += gear[0].parse::<usize>().unwrap() * gear[1].parse::<usize>().unwrap();
            }
        }
        sum
    }

    pub fn from_string(input: String) -> Engine {
        let schematic: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        Engine { schematic }
    }

    fn find_gears(&self) -> HashMap<Pos, Vec<String>> {
        let mut gears = HashMap::new();

        for (i, line) in self.schematic.iter().enumerate() {
            for num_in_line in parse_line_into_numbers(line) {
                let pos = Pos {
                    x: num_in_line.x,
                    y: i,
                };
                self.find_gear_around_number(&mut gears, &pos, &num_in_line.value);
            }
        }
        gears
    }

    pub fn sum_valid_nums(&self) -> usize {
        self.schematic
            .iter()
            .enumerate()
            .flat_map(|(i, line)| {
                parse_line_into_numbers(line)
                    .into_iter()
                    .filter(move |num_in_line| {
                        self.test_around_number(num_in_line.x, i, &num_in_line.value)
                    })
            })
            .map(|num| num.value.parse::<usize>().unwrap())
            .sum()
    }

    fn get_characters_surrounding_number<'a>(&'a self, pos: &Pos, val: &'a String) -> Vec<CharPos> {
        //special case for single digit numbers
        if val.len() == 1 {
            return self.get_surrounding(pos.x, pos.y, PositionInNumber::Single);
        }

        let mut surrounding = vec![];
        surrounding.extend(self.get_surrounding(pos.x, pos.y, PositionInNumber::Start));
        surrounding.extend(self.get_surrounding(
            pos.x + val.len() - 1,
            pos.y,
            PositionInNumber::End,
        ));
        for i in 1..val.len() - 1 {
            surrounding.extend(self.get_surrounding(pos.x + i, pos.y, PositionInNumber::Middle));
        }

        surrounding
    }

    fn test_around_number(&self, x: usize, y: usize, val: &String) -> bool {
        let pos = Pos { x, y };
        self.get_characters_surrounding_number(&pos, val)
            .iter()
            .any(|c| !c.c.is_digit(10) && c.c != '.')
    }

    fn find_gear_around_number(
        &self,
        gears: &mut HashMap<Pos, Vec<String>>,
        pos: &Pos,
        val: &String,
    ) {
        for c_pos in self.get_characters_surrounding_number(pos, val) {
            if c_pos.c == '*' {
                self.add_gear(gears, &c_pos.pos, val);
            }
        }
    }

    fn add_gear(&self, gears: &mut HashMap<Pos, Vec<String>>, pos: &Pos, val: &String) {
        match gears.contains_key(&pos) {
            true => {
                gears.get_mut(&pos).unwrap().push(val.clone());
            }
            false => {
                gears.insert(pos.clone(), vec![val.clone()]);
            }
        }
    }

    fn get_coord(&self, x: usize, y: usize, dir: Direction) -> Result<CharPos, OutOfBoundsError> {
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
            return Err(OutOfBoundsError);
        }

        let x: usize = x.unsigned_abs().try_into().unwrap();
        let y: usize = y.unsigned_abs().try_into().unwrap();

        let row = match self.schematic.get(y) {
            Some(val) => val,
            None => return Err(OutOfBoundsError),
        };

        match row.get(x) {
            Some(val) => Ok(CharPos {
                c: val.clone(),
                pos: Pos { x, y },
            }),
            None => Err(OutOfBoundsError),
        }
    }

    fn get_surrounding(
        &self,
        x: usize,
        y: usize,
        position_in_number: PositionInNumber,
    ) -> Vec<CharPos> {
        let mut surrounding = vec![];
        match position_in_number {
            PositionInNumber::Start => {
                surrounding.push(self.get_coord(x, y, Direction::L).unwrap_or_default());
                surrounding.push(self.get_coord(x, y, Direction::LUp).unwrap_or_default());
                surrounding.push(self.get_coord(x, y, Direction::Up).unwrap_or_default());
                surrounding.push(self.get_coord(x, y, Direction::LDown).unwrap_or_default());
                surrounding.push(self.get_coord(x, y, Direction::Down).unwrap_or_default());
            }
            PositionInNumber::Middle => {
                surrounding.push(self.get_coord(x, y, Direction::Up).unwrap_or_default());
                surrounding.push(self.get_coord(x, y, Direction::Down).unwrap_or_default());
            }
            PositionInNumber::End => {
                surrounding.push(self.get_coord(x, y, Direction::R).unwrap_or_default());
                surrounding.push(self.get_coord(x, y, Direction::RUp).unwrap_or_default());
                surrounding.push(self.get_coord(x, y, Direction::Up).unwrap_or_default());
                surrounding.push(self.get_coord(x, y, Direction::RDown).unwrap_or_default());
                surrounding.push(self.get_coord(x, y, Direction::Down).unwrap_or_default());
            }
            PositionInNumber::Single => {
                surrounding = Direction::iter()
                    .filter(|dir| dir != &Direction::Current)
                    .map(|dir| {
                        self.get_coord(x, y, dir).unwrap_or(CharPos {
                            c: '.'.to_string().chars().next().unwrap(),
                            pos: Pos { x: 0, y: 0 },
                        })
                    })
                    .collect()
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
    fn sum_gear_ratios_with_edge_cases_should_work() {
        let input = "\
.2.
.*.
585
"
        .to_string();
        let engine = Engine::from_string(input);
        assert_eq!(engine.sum_gear_ratios(), 1170);
    }
    #[test]
    fn sum_gear_ratios_should_work() {
        let engine = gen_engine();
        assert_eq!(engine.sum_gear_ratios(), 467835);
    }

    #[test]
    fn find_gears_should_work() {
        let engine = gen_engine();
        let gears = engine.find_gears();
        dbg!(gears);
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
            engine
                .get_surrounding(2, 2, PositionInNumber::Single)
                .into_iter()
                .map(|c_pos| c_pos.c)
                .collect::<Vec<char>>(),
            vec!['.', '.', '*', '.', '5', '.', '.', '.']
        )
    }

    #[test]
    fn get_coord_should_work() {
        let engine = gen_engine();
        assert_eq!(engine.get_coord(1, 0, Direction::Current).unwrap().c, '6');
        assert_eq!(engine.get_coord(1, 0, Direction::R).unwrap().c, '7')
    }
}

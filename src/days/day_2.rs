
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Pull {
    blue: u32,
    red: u32,
    green: u32,
}

impl Pull {

    pub fn new(red: u32, green: u32, blue: u32) -> Pull {
        Pull {
            red,
            green,
            blue,
        }
    }

    fn from_string(pull_text: &str) -> Pull {
        let mut pull = Pull {
            blue: 0,
            red: 0,
            green: 0,
        };

        for color_text in pull_text.split(",") {
            let components: Vec<&str> = color_text.trim().split(" ").collect();
            let value = components[0].parse().unwrap();
            let color = components[1];
            match color {
                "blue" => pull.blue = value,
                "red" => pull.red = value,
                "green" => pull.green = value,
                c => panic!("Unknown color in pull {}", c),
            }
        }

        pull
    }
}


struct Game {
    id: u32,
    pulls: Vec<Pull>,
}
impl Game {

    fn min_cubes(&self) -> Pull {
        let mut min_pull = Pull::new(0,0,0);
        for pull in &self.pulls {
            if pull.red > min_pull.red {
                min_pull.red = pull.red;
            }

            if pull.green > min_pull.green {
                min_pull.green = pull.green;
            }
             if pull.blue > min_pull.blue {
                 min_pull.blue = pull.blue;
             }
        }
        min_pull
    }

    fn is_valid(&self, max_pull: &Pull) -> bool {
        for pull in self.pulls.iter() {
        if pull.green > max_pull.green || pull.red > max_pull.red || pull.blue > max_pull.blue {
                return false;
            }
        }
        return true
    }
    fn from_line(line: &String) -> Game {
        let mut id_and_pulls = line.split(":");
        let id: u32 = id_and_pulls.next()
            .unwrap()
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap();

        let pulls: Vec<Pull> = id_and_pulls
            .next()
            .unwrap()
            .split(";")
            .map(|p| Pull::from_string(p)).collect();


        Game { id, pulls }
    }
}

pub fn sum_valid_ids_from_lines(lines: &Vec<String>, max_pull: &Pull) -> u32 {
    let mut sum = 0;
    for line in lines {
        let game = Game::from_line(line);
        if game.is_valid(max_pull) {
            sum += game.id;
        }
    }
    sum
}

pub fn power_from_lines(lines: &Vec<String>) -> u32 {
    let mut total = 0;
    for line in lines {
        let game = Game::from_line(line).min_cubes();
        let power = game.blue * game.red * game.green;
        total += power;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_game_should_be_correct() {
        let line = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let game = Game::from_line(&line);

        assert_eq!(game.min_cubes(), Pull::new(4, 2, 6));
    }

    #[test]
    fn valid_lines_should_be_summed() {
        let lines: Vec<String> = "\
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .lines().map(|s| s.to_string()).collect();

        let max_pull = Pull {
            red: 12,
            blue: 14,
            green: 13
        };

        let sum = sum_valid_ids_from_lines(&lines, &max_pull );
        assert_eq!(sum, 8);
    }

    #[test]
    fn test_game_should_identify_impossible_games() {
        let line = String::from("Game 1: 3 blue, 1 red; 2 red, 2 green, 6 blue; 2 green");
        let game = Game::from_line(&line);
        let max_pull = Pull {
            red: 1,
            green: 13,
            blue: 14
        };
        let valid_pull = Pull {
            red: 10,
            green: 13,
            blue: 14,
        };

        assert_eq!(game.is_valid(&max_pull), false);
        assert_eq!(game.is_valid(&valid_pull), true);
    }

    #[test]
    fn parse_line_should_return_games_and_id() {
        let line = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let game = Game::from_line(&line);


        assert_eq!(game.pulls.len(), 3);
        assert_eq!(game.id, 1);
        assert_eq!(game.pulls[0], Pull {
            blue: 3,
            red: 4,
            green: 0
        });
    }
}
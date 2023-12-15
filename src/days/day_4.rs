use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
struct Card {
    id: usize,
    winning: Vec<u32>,
    picks: Vec<u32>,
}

trait StringSliceHelpers {
    fn parse_numbers_from_space_separated(&self) -> Vec<u32>;
}

impl StringSliceHelpers for str {
    fn parse_numbers_from_space_separated(&self) -> Vec<u32> {
        self.split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
    }
}

pub fn sum_all_cards(lines: &Vec<String>) -> u32 {
    let mut sum = 0;
    for line in lines {
        let card = Card::from_line(&line);
        sum += card.calculate_score();
    }
    sum
}

#[derive(Clone, Debug, PartialEq)]
struct NumberOfCards {
    num_cards: u32,
    card: Card,
}

impl NumberOfCards {
    pub fn new(card: Card) -> NumberOfCards {
        NumberOfCards { num_cards: 1, card }
    }
}

pub fn sum_all_cards_with_new_rules(lines: &Vec<String>) -> u32 {
    let mut cards = BTreeMap::new();
    for line in lines {
        let card = Card::from_line(&line);
        cards.insert(card.id, NumberOfCards::new(card));
    }

    let mut sum = 0;

    // cards start at index 1
    // can't use an iterator here, because we need to both grab a card and mutate others
    for index in 1..=cards.len() {
        let card_pile = cards.remove(&index).unwrap();
        let num_wins = card_pile.card.calculate_number_of_wins();
        if num_wins > 0 {
            for i in 1..=num_wins {
                let future_card_pile = cards.get_mut(&(index + i as usize)).unwrap();
                future_card_pile.num_cards += card_pile.num_cards;
            }
        }
        sum += card_pile.num_cards;
    }

    sum
}

impl Card {
    fn from_line(line: &str) -> Card {
        //                         Winning       Picks
        // example line: Card 1: 32 48 49 10 | 83 4 32 1 10
        let parts = line
            .split(":")
            .filter(|l| !l.is_empty())
            .collect::<Vec<&str>>();
        let id = parts[0]
            .trim()
            .split(" ")
            .filter(|l| !l.is_empty())
            .collect::<Vec<&str>>()[1]
            .parse::<usize>()
            .unwrap();

        let numbers = parts[1];

        let winning =
            numbers.split("|").collect::<Vec<&str>>()[0].parse_numbers_from_space_separated();
        let picks =
            numbers.split("|").collect::<Vec<&str>>()[1].parse_numbers_from_space_separated();

        Card { id, winning, picks }
    }

    fn calculate_number_of_wins(&self) -> u32 {
        let mut wins = 0;
        for num in self.winning.iter() {
            if self.picks.contains(num) {
                wins += 1;
            }
        }
        wins
    }
    fn calculate_score(&self) -> u32 {
        let mut score = 0;
        for num in self.winning.iter() {
            if self.picks.contains(num) {
                match score {
                    0 => score = 1,
                    _ => score *= 2,
                }
            }
        }
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_card_from_line() {
        let line = "Card 1: 32 48 49 10 | 83 4 32 1 10";
        let card = Card::from_line(line);
        assert_eq!(card.winning, vec![32, 48, 49, 10]);
        assert_eq!(card.picks, vec![83, 4, 32, 1, 10]);
        assert_eq!(card.id, 1);
    }

    #[test]
    fn test_calculate_score() {
        let card = Card {
            winning: vec![32, 48, 49, 10],
            picks: vec![83, 4, 32, 1, 10],
            id: 1,
        };
        assert_eq!(card.calculate_score(), 2);
    }

    #[test]
    fn test_sum_all_cards() {
        let input = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let lines = input
            .split("\n")
            .map(|s| String::from(s))
            .collect::<Vec<String>>();
        assert_eq!(sum_all_cards(&lines), 13);
    }

    #[test]
    fn test_sum_all_cards_with_new_rules() {
        let input = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let lines = input
            .split("\n")
            .map(|s| String::from(s))
            .collect::<Vec<String>>();
        assert_eq!(sum_all_cards_with_new_rules(&lines), 30);
    }
}

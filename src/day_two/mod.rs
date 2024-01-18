#[allow(dead_code)]
use crate::read_file::read_lines;

pub fn solve() {
    part_one();
    part_two()
}

#[derive(Copy, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn score(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn determine_winner(them: &Hand, me: Hand) -> Outcome {
        match them {
            Hand::Rock => match me {
                Hand::Rock => Outcome::Draw(them.clone()),
                Hand::Paper => Outcome::Lose(them.clone()),
                Hand::Scissors => Outcome::Win(them.clone()),
            },
            Hand::Paper => match me {
                Hand::Rock => Outcome::Win(them.clone()),
                Hand::Paper => Outcome::Draw(them.clone()),
                Hand::Scissors => Outcome::Lose(them.clone()),
            },
            Hand::Scissors => match me {
                Hand::Rock => Outcome::Lose(them.clone()),
                Hand::Paper => Outcome::Win(them.clone()),
                Hand::Scissors => Outcome::Draw(them.clone()),
            },
        }
    }

    fn from_str_part_one(str: &str) -> Hand {
        match str {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => panic!(),
        }
    }

    fn from_str_part_two(str: &[&str]) -> Outcome {
        match str[0] {
            "A" => match str[1] {
                "X" => Outcome::Lose(Hand::Scissors),
                "Y" => Outcome::Draw(Hand::Rock),
                "Z" => Outcome::Win(Hand::Paper),
                _ => panic!(),
            },
            "B" => match str[1] {
                "X" => Outcome::Lose(Hand::Rock),
                "Y" => Outcome::Draw(Hand::Paper),
                "Z" => Outcome::Win(Hand::Scissors),
                _ => panic!(),
            },
            "C" => match str[1] {
                "X" => Outcome::Lose(Hand::Paper),
                "Y" => Outcome::Draw(Hand::Scissors),
                "Z" => Outcome::Win(Hand::Rock),
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
}

enum Outcome {
    Win(Hand),
    Lose(Hand),
    Draw(Hand),
}

impl Outcome {
    fn score_match(&self) -> u32 {
        match self {
            Outcome::Win(hand) | Outcome::Lose(hand) | Outcome::Draw(hand) => {
                self.score() + hand.score()
            }
        }
    }

    fn score(&self) -> u32 {
        match self {
            Outcome::Win(_) => 6,
            Outcome::Lose(_) => 0,
            Outcome::Draw(_) => 3,
        }
    }
}

fn part_one() {
    if let Ok(lines) = read_lines("src/day_two/input.txt") {
        let res = lines
            .flatten()
            .map(|line| {
                line.split_whitespace()
                    .map(|str| Hand::from_str_part_one(str))
                    .collect::<Vec<Hand>>()
                    .chunks(2)
                    .map(|a| Hand::determine_winner(&a[1], a[0]))
                    .map(|o| o.score_match())
                    .sum::<u32>()
            })
            .sum::<u32>();
        println!("Part one: {:?}", res);
    }
}

fn part_two() {
    if let Ok(lines) = read_lines("src/day_two/input.txt") {
        let res = lines
            .flatten()
            .map(|line| {
                line.split_whitespace()
                    .collect::<Vec<&str>>()
                    .chunks(2)
                    .map(|str| Hand::from_str_part_two(str))
                    .map(|o| o.score_match())
                    .sum::<u32>()
            })
            .sum::<u32>();
        println!("Part two: {:?}", res);
    }
}

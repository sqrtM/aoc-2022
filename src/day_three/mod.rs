#![allow(dead_code)]
use crate::read_file::read_lines;

pub fn solve() {
    part_one();
    part_two()
}

struct Rucksack {
    a: Vec<Item>,
    b: Vec<Item>,
}

impl Rucksack {
    fn new(input: &str) -> Self {
        let (a, b) = input
            .chars()
            .zip(input.chars().skip(input.len() / 2))
            .map(|(a, b)| (Item::new(&a), Item::new(&b)))
            .unzip();
        Self { a, b }
    }

    fn find_matching_item(&self) -> &Item {
        self.a
            .iter()
            .find(|i| self.b.contains(i))
            .expect("Could not find match!")
    }

    fn all_items(&self) -> impl Iterator<Item = &Item> {
        self.a.iter().chain(&self.b)
    }

    fn find_badge<'a>(one: &'a Rucksack, two: &'a Rucksack, three: &'a Rucksack) -> &'a Item {
        one.all_items()
            .filter(|x| two.all_items().collect::<Vec<_>>().contains(x))
            .find(|y| three.all_items().collect::<Vec<_>>().contains(y))
            .expect("Could not find matching item!")
    }
}

#[derive(PartialEq)]
struct Item(char);

impl Item {
    fn new(ch: &char) -> Self {
        if ch.is_digit(36) {
            Item(*ch)
        } else {
            panic!("Non-Alphanumeric character passed as Item.")
        }
    }

    fn get_priority(&self) -> u32 {
        let mut value = self
            .0
            .to_digit(36)
            .expect("Failed trying to read value of non-alphanumeric.")
            - 9;

        if self.0.is_uppercase() {
            value += 26;
        }

        value
    }
}

fn part_one() {
    if let Ok(lines) = read_lines("src/day_three/input.txt") {
        println!(
            "Part one: {}",
            lines
                .flatten()
                .map(|line| Rucksack::new(&line).find_matching_item().get_priority())
                .sum::<u32>()
        );
    }
}

fn part_two() {
    if let Ok(lines) = read_lines("src/day_three/input.txt") {
        println!(
            "Part two: {}",
            lines
                .flatten()
                .map(|line| Rucksack::new(&line))
                .collect::<Vec<Rucksack>>()
                .chunks(3)
                .map(|i| Rucksack::find_badge(&i[0], &i[1], &i[2]).get_priority())
                .sum::<u32>()
        );
    }
}

#![allow(dead_code)]
#![allow(dead_code)]

use crate::read_file::read_lines;
use std::ops::RangeInclusive;

pub fn solve() {
    part_one();
    part_two()
}

#[derive(Clone, Debug)]
struct Rooms(RangeInclusive<u32>);

struct Team {
    a: Rooms,
    b: Rooms,
}

impl Team {
    fn from_str(str: &str) -> Self {
        let a = str
            .split(',')
            .flat_map(|s| {
                s.split('-')
                    .flat_map(|t| t.parse::<u32>())
                    .collect::<Vec<u32>>()
                    .chunks(2)
                    .map(|t| (t[0], t[1]))
                    .collect::<Vec<(u32, u32)>>()
            })
            .map(|r| Rooms(r.0..=r.1))
            .collect::<Vec<Rooms>>();

        Self {
            a: a[0].clone(),
            b: a[1].clone(),
        }
    }

    fn fully_overlaps(a: &Rooms, b: &Rooms) -> bool {
        a.0.start() >= b.0.start() && a.0.end() <= b.0.end()
    }

    fn partially_overlaps(a: &Rooms, b: &Rooms) -> bool {
        a.0.start() <= b.0.end() && a.0.end() >= b.0.start()
    }
}

fn part_one() {
    if let Ok(lines) = read_lines("src/day_four/input.txt") {
        println!(
            "Part one: {}",
            lines
                .flatten()
                .map(|line| Team::from_str(&line))
                .filter(|x| Team::fully_overlaps(&x.a, &x.b) || Team::fully_overlaps(&x.b, &x.a))
                .count()
        )
    }
}

fn part_two() {
    if let Ok(lines) = read_lines("src/day_four/input.txt") {
        println!(
            "Part two: {}",
            lines
                .flatten()
                .map(|line| Team::from_str(&line))
                .filter(|x| Team::partially_overlaps(&x.a, &x.b))
                .count()
        )
    }
}

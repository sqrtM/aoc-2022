use std::fs::File;
use std::io::{BufReader, Lines};
use crate::read_file::read_lines;

pub fn solve() {
    part_one();
    part_two();
}

fn part_one() {
    let mut totals = vec![];
    let total = 0;
    if let Ok(lines) = read_lines("src/day_one/input.txt") {
        parse(&mut totals, total, lines);
        println!("Part one: {:?}", totals.iter().max().unwrap());
    }
}

fn part_two() {
    let mut totals = vec![];
    let total = 0;
    if let Ok(lines) = read_lines("src/day_one/input.txt") {
        parse(&mut totals, total, lines);
    }

    let mut top_three = vec![];
    while top_three.len() < 3 {
        top_three.push(totals.remove(totals.iter().position(|x| x == totals.iter().max().unwrap()).unwrap()));
    }
    println!("Part two: {:?}", top_three.iter().sum::<i32>());
}

fn parse(mut totals: &mut Vec<i32>, mut total: i32, lines: Lines<BufReader<File>>) {
    lines.flatten().for_each(|line| {
        if let Ok(v) = line.parse::<i32>() {
            total += v;
        } else {
            totals.push(total);
            total = 0;
        }
    });
}
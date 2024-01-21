#![allow(dead_code)]

use crate::read_file::read_lines;
use regex::Regex;

pub fn solve() {
    let mut stack_input: String = String::new();
    let mut all_commands = Vec::new();
    let command_regex = Regex::new(r"(?m)\b(\d{1,3})\b").unwrap();

    if let Ok(lines) = read_lines("src/day_five/input.txt") {
        let mut parsing_stack = true;
        for line in lines.flatten() {
            if line.is_empty() {
                parsing_stack = false;
            } else if !parsing_stack {
                let mut commands = vec![];
                command_regex
                    .captures_iter(&line)
                    .map(|y| y.extract::<1>())
                    .for_each(|z| commands.push(z.0.parse::<usize>().unwrap()));
                assert_eq!(commands.len(), 3);
                all_commands.push(commands);
            } else {
                stack_input.push_str(&format!("{}\n", &line));
            }
        }
    }
    let stack = parse_stack(&stack_input);

    part_one(&stack, &all_commands);
    part_two(&stack, &all_commands);
}

#[derive(Copy, Clone)]
struct Block<'a>(&'a str);

impl<'a> Block<'a> {
    fn new(str: &'a str) -> Self {
        Self(str)
    }
    fn get_str(&self) -> &'a str {
        self.0
    }
}

fn parse_stack(stack_input: &str) -> Vec<Vec<Block>> {
    let mut stacks: Vec<Vec<Block>> = vec![
        vec![];
        stack_input
            .trim_end()
            .chars()
            .rev()
            .find(|&c| !c.is_whitespace())
            .unwrap()
            .to_digit(10)
            .unwrap() as usize
    ];

    for line in stack_input.lines().rev().skip(1) {
        for (j, captures) in Regex::new(r"(?m)(?P<space> {3,4})|(?P<block>\[(?P<char>([A-Z]))])")
            .unwrap()
            .captures_iter(line)
            .enumerate()
        {
            if captures.name("block").is_some() {
                stacks[j].push(Block::new(captures.name("char").unwrap().as_str()));
            }
        }
    }

    stacks
}

fn execute_commands<'a>(
    cmds: &'a [Vec<usize>],
    stk: &'a [Vec<Block>],
    part_two: bool,
) -> Vec<Vec<Block<'a>>> {
    let mut return_vec = stk.to_owned();

    cmds.iter().for_each(|cmd| {
        let mut blocks = vec![];
        for _ in return_vec[cmd[1] - 1].len() - cmd[0]..return_vec[cmd[1] - 1].len() {
            blocks.push(return_vec[cmd[1] - 1].pop().unwrap());
        }

        if part_two {
            blocks.reverse();
        }

        return_vec[cmd[2] - 1].extend(blocks);
    });

    return_vec
}

fn get_top_of_stacks<'a>(stks: &'a [Vec<Block>]) -> Vec<Block<'a>> {
    stks.iter().map(|stk| *stk.last().unwrap()).collect()
}

fn part_one(stacks: &[Vec<Block>], cmds: &[Vec<usize>]) {
    let final_stack = execute_commands(cmds, stacks, false);
    println!(
        "Part one: {:?}",
        get_top_of_stacks(&final_stack)
            .iter()
            .map(|b| b.get_str())
            .collect::<String>()
    );
}

fn part_two(stacks: &[Vec<Block>], cmds: &[Vec<usize>]) {
    let final_stack = execute_commands(cmds, stacks, true);
    println!(
        "Part two: {:?}",
        get_top_of_stacks(&final_stack)
            .iter()
            .map(|b| b.get_str())
            .collect::<String>()
    );
}

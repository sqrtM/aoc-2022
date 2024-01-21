use crate::read_file::read_lines;
use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Lines};

pub fn solve() {
    let mut stacks = vec![];
    let mut stack_input: String = "".to_string();
    if let Ok(lines) = read_lines("src/day_five/input.txt") {
        for line in lines.flatten() {
            if line.is_empty() {
                break;
            } else {
                stack_input.push_str(&line);
            }
            stack_input.push('\n');
        }
        stacks = parse_stack(&stack_input);
    }

    //part_one(&stacks, stack_input.lines().count());
    part_two(&stacks, stack_input.lines().count());
}

#[derive(Copy, Clone, Debug)]
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
    let regex = Regex::new(r"(?m)(?P<space> {3,4})|(?P<block>\[(?P<char>([A-Z]))])").unwrap();

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
        for (j, captures) in regex.captures_iter(line).enumerate() {
            if captures.name("block").is_some() {
                stacks[j].push(Block::new(captures.name("char").unwrap().as_str()));
            }
        }
    }

    stacks
}

fn execute_commands_one<'a>(cmds: &'a [Vec<u8>], stk: &'a [Vec<Block>]) -> Vec<Vec<Block<'a>>> {
    let mut return_vec = stk.to_owned();

    cmds.iter().skip(1).for_each(|cmd| {
        let mut blocks = vec![];

        for _ in return_vec[cmd[1] as usize - 1].len() - cmd[0] as usize
            ..return_vec[cmd[1] as usize - 1].len()
        {
            blocks.push(return_vec[cmd[1] as usize - 1].pop().unwrap());
        }

        for item in blocks {
            return_vec[cmd[2] as usize - 1].push(item)
        }
    });
    return_vec
}

fn execute_commands_two<'a>(cmds: &'a [Vec<u8>], stk: &'a [Vec<Block>]) -> Vec<Vec<Block<'a>>> {
    let mut return_vec = stk.to_owned();

    cmds.iter().skip(1).for_each(|cmd| {
        let mut blocks = vec![];

        for _ in return_vec[cmd[1] as usize - 1].len() - cmd[0] as usize
            ..return_vec[cmd[1] as usize - 1].len()
        {
            blocks.push(return_vec[cmd[1] as usize - 1].pop().unwrap());
        }

        for item in blocks.iter().rev() {
            return_vec[cmd[2] as usize - 1].push(*item)
        }
    });
    return_vec
}

fn get_top_of_stacks<'a>(stks: &'a [Vec<Block>]) -> Vec<Block<'a>> {
    let mut tops = vec![];
    for stk in stks {
        tops.push(*stk.last().unwrap());
    }
    tops
}

fn extract_commands(stack_height: usize, lines: Lines<BufReader<File>>) -> Vec<Vec<u8>> {
    let regex = Regex::new(r"(?m)\b(\d{1,3})\b").unwrap();
    let mut all_commands = vec![];
    lines.flatten().skip(stack_height).for_each(|x| {
        let mut commands = vec![];
        regex
            .captures_iter(&x)
            .map(|y| y.extract::<1>())
            .for_each(|z| commands.push(z.0.parse::<u8>().unwrap()));
        all_commands.push(commands);
    });
    all_commands
}

fn part_one(stacks: &[Vec<Block>], stack_height: usize) {
    if let Ok(lines) = read_lines("src/day_five/input.txt") {
        let all_commands = extract_commands(stack_height, lines);
        let final_stack = execute_commands_one(&all_commands, stacks);
        println!(
            "Part one: {:?}",
            get_top_of_stacks(&final_stack)
                .iter()
                .map(|b| b.get_str())
                .collect::<String>()
        );
    }
}

fn part_two(stacks: &[Vec<Block>], stack_height: usize) {
    if let Ok(lines) = read_lines("src/day_five/input.txt") {
        let all_commands = extract_commands(stack_height, lines);
        let final_stack = execute_commands_two(&all_commands, stacks);
        println!(
            "Part two: {:?}",
            get_top_of_stacks(&final_stack)
                .iter()
                .map(|b| b.get_str())
                .collect::<String>()
        );
    }
}

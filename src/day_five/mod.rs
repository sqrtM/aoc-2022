use crate::read_file::read_lines;
use regex::Regex;

pub fn solve() {
    part_one();
    //part_two()
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

fn execute_commands<'a>(cmds: &'a [Vec<u8>], stk: &'a [Vec<Block>]) -> Vec<Vec<Block<'a>>> {
    let mut temp = stk.to_owned();

    cmds.iter().skip(1).for_each(|cmd| {
        let mut blocks = vec![];

        for _ in temp[cmd[1] as usize - 1].len() - cmd[0] as usize..temp[cmd[1] as usize - 1].len()
        {
            blocks.push(temp[cmd[1] as usize - 1].pop().unwrap());
        }

        for item in blocks {
            temp[cmd[2] as usize - 1].push(item)
        }
    });
    temp
}

fn get_top_of_stacks<'a>(stks: &'a [Vec<Block>]) -> Vec<Block<'a>> {
    let mut tops = vec![];
    for stk in stks {
        tops.push(*stk.last().unwrap());
    }
    tops
}

fn part_one() {
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

    if let Ok(lines) = read_lines("src/day_five/input.txt") {
        let regex = Regex::new(r"(?m)\b(\d{1,3})\b").unwrap();
        let stack_m = stacks;

        let mut all_caps = vec![];
        lines
            .flatten()
            .skip(stack_input.lines().count())
            .for_each(|x| {
                let mut caps = vec![];
                regex
                    .captures_iter(&x)
                    .map(|y| y.extract::<1>())
                    .for_each(|z| caps.push(z.0.parse::<u8>().unwrap()));
                all_caps.push(caps);
            });
        let x = execute_commands(&all_caps, &stack_m);
        println!(
            "Part one: {:?}",
            get_top_of_stacks(&x)
                .iter()
                .map(|b| b.get_str())
                .collect::<String>()
        );
    }
}

use itertools::Itertools;
use regex::Regex;

fn main() {
    if let Err(e) = do_main() {
        eprintln!("{}", e);
    }
}

type Stack = Vec<char>;

fn do_main() -> shared::Result<()> {
    let contents = shared::read_file_from_args()?;
    let mut lines = contents.lines();

    // Parse the part of the input that has the stack layout
    let mut stack_lines = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(line_to_stack_vecs)
        .collect_vec();

    // Remove row listing stack numbers
    stack_lines.pop();

    let mut stacks = stack_lines_to_stacks(stack_lines);
    let directions: Vec<Direction> = lines.clone().map(parse_direction).collect();

    let mut stacks_pt_2 = stacks.clone();
    let directions_pt_2: Vec<Direction> = lines.map(parse_direction).collect();

    for direction in &directions {
        move_one_at_a_time(&mut stacks, direction);
    }

    let last = stacks
        .iter_mut()
        .map(|stack| stack.pop().unwrap())
        .collect_vec();

    println!("Part 1: {:?}", last);

    for direction in &directions_pt_2 {
        move_as_group(&mut stacks_pt_2, direction);
    }

    let last = stacks_pt_2
        .iter_mut()
        .filter_map(|stack| stack.last())
        .collect_vec();

    println!("Part 2: {:?}", last);

    Ok(())
}

/// Takes a line like:
/// `    [B] [C]`
/// and converts it into:
/// `None, Some('B'), Some('C')`
fn line_to_stack_vecs(line: &str) -> Vec<Option<char>> {
    line.chars()
        .chunks(4)
        .into_iter()
        .flat_map(|mut chunk| chunk.nth(1))
        .map(|crate_type| match crate_type {
            ' ' => None,
            ct => Some(ct),
        })
        .collect_vec()
}

/// Takes a Vec of parsed lines and builds a Vec of Stacks.
fn stack_lines_to_stacks(mut stack_lines: Vec<Vec<Option<char>>>) -> Vec<Stack> {
    let stack_count: usize = stack_lines[0].len();
    let mut stacks: Vec<Stack> = Vec::new();

    // Set up the initial stacks.
    for _i in 0..stack_count {
        stacks.push(Vec::new());
    }

    stack_lines.reverse();

    for line in stack_lines {
        for (i, crate_type) in line.into_iter().enumerate() {
            match crate_type {
                Some(ct) => stacks[i].push(ct),
                None => {}
            }
        }
    }

    stacks
}

#[derive(Debug)]
struct Direction {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_direction(line: &str) -> Direction {
    // TODO: Make this static
    let re: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let capture = re.captures(line).unwrap();
    let count = capture.get(1).unwrap().as_str().parse().unwrap();
    let from = capture.get(2).unwrap().as_str().parse().unwrap();
    let to = capture.get(3).unwrap().as_str().parse().unwrap();

    Direction { count, from, to }
}

fn move_one_at_a_time(stacks: &mut Vec<Stack>, direction: &Direction) {
    for _i in 0..direction.count {
        let crate_type = stacks[direction.from - 1].pop().unwrap();
        stacks[direction.to - 1].push(crate_type);
    }
}

fn move_as_group(stacks: &mut Vec<Stack>, direction: &Direction) {
    let split_idx = stacks[direction.from - 1].len() - direction.count;
    let mut removed = stacks[direction.from - 1].split_off(split_idx);

    stacks[direction.to - 1].append(&mut removed);
}

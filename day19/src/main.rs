use clap::Parser;
use itertools::Itertools;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
    #[arg(long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let workflows: HashMap<&str, Vec<&str>> = lines
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (name, remainder) = line.split("{").collect_tuple().unwrap();
            let remainder = remainder.trim_end_matches("}");
            let steps = remainder.split(",").collect_vec();
            (name, steps)
        })
        .collect();

    let parts = lines
        .iter()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            let (x, m, a, s) = line
                .trim_end_matches("}")
                .trim_start_matches("{")
                .split(",")
                .collect_tuple()
                .unwrap();
            (
                x.split_once("=").unwrap().1.parse::<i64>().unwrap(),
                m.split_once("=").unwrap().1.parse::<i64>().unwrap(),
                a.split_once("=").unwrap().1.parse::<i64>().unwrap(),
                s.split_once("=").unwrap().1.parse::<i64>().unwrap(),
            )
        })
        .collect_vec();

    let part1 = parts
        .iter()
        .map(|part| process_part(*part, &workflows))
        .filter(|result| result.is_some())
        .map(|result| result.unwrap())
        .sum::<i64>();
    println!("Part 1: {}", part1);

    part2(&workflows);
}

fn process_part(part: (i64, i64, i64, i64), workflows: &HashMap<&str, Vec<&str>>) -> Option<i64> {
    let mut workflow = workflows.get("in").unwrap();
    let mut workflow_index = 0;

    loop {
        let step = workflow[workflow_index];
        if step == "A" {
            return Some(part.0 + part.1 + part.2 + part.3);
        }
        if step == "R" {
            return None;
        }
        if workflow_index == workflow.len() - 1 {
            workflow_index = 0;
            workflow = workflows.get(step).unwrap();
            continue;
        }
        let compare_value = match step.chars().nth(0).unwrap() {
            'x' => part.0,
            'm' => part.1,
            'a' => part.2,
            's' => part.3,
            _ => panic!("Unexpected var"),
        };

        let target_value = step.split_once(":").unwrap().0[2..].parse::<i64>().unwrap();

        let passes = match step.chars().nth(1).unwrap() {
            '<' => compare_value < target_value,
            '>' => compare_value > target_value,
            _ => panic!("Bad comparator"),
        };

        if passes {
            workflow_index = 0;
            let next_workflow = step.split_once(":").unwrap().1;
            if next_workflow == "R" {
                return None;
            }
            if next_workflow == "A" {
                return Some(part.0 + part.1 + part.2 + part.3);
            }
            workflow = workflows.get(step.split_once(":").unwrap().1).unwrap();
        } else {
            workflow_index += 1;
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct PossibleValue {
    min: u64,
    max: u64,
}

fn part2(workflows: &HashMap<&str, Vec<&str>>) {
    let initial_part_range = vec![
        PossibleValue { min: 1, max: 4000 },
        PossibleValue { min: 1, max: 4000 },
        PossibleValue { min: 1, max: 4000 },
        PossibleValue { min: 1, max: 4000 },
    ];

    let ranges = part2_inner(initial_part_range, "in", 0, workflows);
    let part2 = ranges
        .iter()
        .map(|range| {
            (range[0].max + 1 - range[0].min)
                * (range[1].max + 1 - range[1].min)
                * (range[2].max + 1 - range[2].min)
                * (range[3].max + 1 - range[3].min)
        })
        .sum::<u64>();

    println!("Part 2: {}", part2);
}

fn part2_inner(
    part_range: Vec<PossibleValue>,
    workflow_name: &str,
    workflow_index: usize,
    workflows: &HashMap<&str, Vec<&str>>,
) -> Vec<Vec<PossibleValue>> {
    let workflow = workflows.get(workflow_name).unwrap();
    let step = workflow[workflow_index];

    if step == "A" {
        return vec![part_range];
    }
    if step == "R" {
        return Vec::new();
    }
    if workflow_index == workflow.len() - 1 {
        return part2_inner(part_range, step, 0, workflows);
    }

    let compare_index = match step.chars().nth(0).unwrap() {
        'x' => 0,
        'm' => 1,
        'a' => 2,
        's' => 3,
        _ => panic!("Unexpected var"),
    };
    let target_value = step.split_once(":").unwrap().0[2..].parse::<u64>().unwrap();

    let next_workflow_name = step.split_once(":").unwrap().1;
    let mut possible_results = Vec::new();
    if step.chars().nth(1).unwrap() == '>' {
        if part_range[compare_index].max > target_value {
            let mut sub_range = part_range.clone();
            sub_range[compare_index].min = sub_range[compare_index].min.max(target_value + 1);
            if next_workflow_name == "A" {
                possible_results.push(sub_range);
            } else if next_workflow_name != "R" {
                possible_results.append(&mut part2_inner(
                    sub_range,
                    next_workflow_name,
                    0,
                    workflows,
                ));
            }
        }
        if part_range[compare_index].min <= target_value {
            let mut sub_range = part_range.clone();
            sub_range[compare_index].max = sub_range[compare_index].max.min(target_value);
            possible_results.append(&mut part2_inner(
                sub_range,
                workflow_name,
                workflow_index + 1,
                workflows,
            ));
        }
    }
    if step.chars().nth(1).unwrap() == '<' {
        if part_range[compare_index].min < target_value {
            let mut sub_range = part_range.clone();
            sub_range[compare_index].max = sub_range[compare_index].max.min(target_value - 1);
            if next_workflow_name == "A" {
                possible_results.push(sub_range);
            } else if next_workflow_name != "R" {
                possible_results.append(&mut part2_inner(
                    sub_range,
                    next_workflow_name,
                    0,
                    workflows,
                ));
            }
        }
        if part_range[compare_index].max >= target_value {
            let mut sub_range = part_range.clone();
            sub_range[compare_index].min = sub_range[compare_index].min.max(target_value);
            possible_results.append(&mut part2_inner(
                sub_range,
                workflow_name,
                workflow_index + 1,
                workflows,
            ));
        }
    }

    return possible_results;
}

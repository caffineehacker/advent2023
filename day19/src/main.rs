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

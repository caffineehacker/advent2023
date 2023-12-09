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

    let instructions = lines[0].chars().collect_vec();

    let mut graph = HashMap::new();

    for line in lines.iter().skip(2) {
        let (start, dest) = line.split(" = ").collect_tuple().unwrap();
        let (left, right) = dest
            .trim_matches('(')
            .trim_matches(')')
            .split(", ")
            .collect_tuple()
            .unwrap();

        graph.insert(start.to_string(), (left.to_string(), right.to_string()));
    }

    let mut instruction_index = 0;
    let mut current_node = "AAA".to_string();
    let mut steps = 0;
    if args.debug {
        print!("{}", current_node);
    }
    while current_node != "ZZZ" {
        steps += 1;
        let destinations = graph.get(&current_node).unwrap();

        current_node = if instructions[instruction_index] == 'L' {
            destinations.0.to_owned()
        } else {
            destinations.1.to_owned()
        };
        if args.debug {
            print!(" -> {}", current_node);
        }
        instruction_index += 1;
        instruction_index %= instructions.len();
    }

    println!("\nPart 1: {}", steps);

    // Part 2
    let mut current_nodes = graph
        .keys()
        .filter(|key| key.ends_with("A"))
        .cloned()
        .collect_vec();
    let mut first_z_seen = Vec::new();
    first_z_seen.resize(current_nodes.len(), -1);

    instruction_index = 0;
    steps = 0;
    while first_z_seen.iter().any(|z| *z == -1) {
        steps += 1;

        current_nodes = current_nodes
            .iter()
            .map(|node| {
                let destinations = graph.get(node).unwrap();
                if instructions[instruction_index] == 'L' {
                    destinations.0.to_owned()
                } else {
                    destinations.1.to_owned()
                }
            })
            .collect_vec();

        if args.debug {
            println!("Step: {}\n\n{:?}\n\n", steps, current_nodes);
        }

        for i in 0..current_nodes.len() {
            if current_nodes[i].ends_with("Z") && first_z_seen[i] == -1 {
                first_z_seen[i] = steps;
            }
        }

        instruction_index += 1;
        instruction_index %= instructions.len();
    }

    if args.debug {
        println!("First Z's: {:?}", first_z_seen);
    }

    println!("Part 2: {}", lcm(first_z_seen));
}

pub fn lcm(nums: Vec<i64>) -> i64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(nums.iter().skip(1).cloned().collect_vec());
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

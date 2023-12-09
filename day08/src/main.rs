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
}

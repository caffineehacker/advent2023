use clap::Parser;
use itertools::Itertools;
use std::{
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

    let part1: u64 = lines
        .get(0)
        .unwrap()
        .split(",")
        .map(|step| hash(step))
        .sum();

    println!("Part 1: {}", part1);

    let mut boxes: Vec<Vec<(&str, usize)>> = Vec::new();
    boxes.resize(256, Vec::new());
    for operation in lines.get(0).unwrap().split(",") {
        if operation.ends_with("-") {
            let b = &mut boxes[hash(operation.trim_end_matches("-")) as usize];
            for i in 0..b.len() {
                if b[i].0 == operation.trim_end_matches("-") {
                    b.remove(i);
                    break;
                }
            }
        } else {
            let (name, value) = operation.split("=").collect_tuple().unwrap();
            let value = value.parse::<usize>().unwrap();
            let b = &mut boxes[hash(name) as usize];
            let mut was_set = false;
            for i in 0..b.len() {
                if b[i].0 == name {
                    b[i].1 = value;
                    was_set = true;
                    break;
                }
            }

            if !was_set {
                b.push((name, value));
            }
        }

        if args.debug {
            println!("After {}", operation);
            for i in 0..boxes.len() {
                if !boxes[i].is_empty() {
                    println!("Box {}: {:?}", i, boxes[i]);
                }
            }
        }
    }

    let part2 = boxes
        .iter()
        .enumerate()
        .map(|(index, b)| {
            (1 + index)
                * b.iter()
                    .enumerate()
                    .map(|(lens_index, l)| (lens_index + 1) * l.1)
                    .sum::<usize>()
        })
        .sum::<usize>();

    println!("Part 2: {}", part2);
}

fn hash(text: &str) -> u64 {
    let mut value = 0;
    for char in text.chars() {
        let ascii_code = char as u8;
        value += ascii_code as u64;
        value *= 17;
        value %= 256;
    }

    return value;
}

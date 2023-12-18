use clap::Parser;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
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

    let instructions = lines
        .iter()
        .map(|line| {
            let (dir, count, color) = line.split_ascii_whitespace().collect_tuple().unwrap();
            let color = color.trim_start_matches("(").trim_end_matches(")");
            return (dir, count.parse::<i64>().unwrap(), color);
        })
        .collect_vec();

    let mut x = 0;
    let mut y = 0;

    let mut dug = HashMap::new();

    for instruction in instructions.iter() {
        let direction: (isize, isize) = match instruction.0 {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => panic!("Unexpected direction"),
        };

        for _ in 0..instruction.1 {
            dug.insert((x, y), (direction, instruction.2));
            x += direction.0;
            y += direction.1;
        }
    }

    println!("Trench: {}", dug.len());
    let max_x = dug.keys().map(|k| k.0).max().unwrap();
    let min_x = dug.keys().map(|k| k.0).min().unwrap();
    let max_y = dug.keys().map(|k| k.1).max().unwrap();
    let min_y = dug.keys().map(|k| k.1).min().unwrap();

    // Flood fill the outside and then subtract that from the total area
    let mut to_process = Vec::new();
    let mut seen = HashSet::new();
    to_process.push((min_x - 1, min_y - 1));

    while !to_process.is_empty() {
        let point = to_process.pop().unwrap();
        if point.0 < min_x - 1 || point.1 < min_y - 1 || point.0 > max_x + 1 || point.1 > max_y + 1
        {
            continue;
        }
        if seen.contains(&point) {
            continue;
        }
        if dug.contains_key(&point) {
            continue;
        }
        seen.insert(point.clone());
        to_process.push((point.0 + 1, point.1));
        to_process.push((point.0 - 1, point.1));
        to_process.push((point.0, point.1 + 1));
        to_process.push((point.0, point.1 - 1));
    }

    if args.debug {
        for y in (min_y - 1)..=(max_y + 1) {
            for x in (min_x - 1)..=(max_x + 1) {
                if seen.contains(&(x, y)) {
                    print!("!");
                } else if dug.contains_key(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }

    println!("Filled points = {}", seen.len());
    println!("Min: {}, {}   Max: {}, {}", min_x, min_y, max_x, max_y);
    let part1 = ((max_x + 3 - min_x) * (max_y + 3 - min_y)) - seen.len() as isize;
    println!("Part 1: {}", part1);
}

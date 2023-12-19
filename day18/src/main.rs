use clap::Parser;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    iter,
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

    println!(
        "Part 1: {}",
        solve(
            instructions
                .iter()
                .map(|(dir, count, _)| (*dir, *count))
                .collect_vec(),
            args.debug
        )
    );

    let instructions = instructions
        .iter()
        .map(|(_, _, color)| {
            let color = color.trim_start_matches("#");
            let dir = color.chars().last().unwrap();
            let dir = match dir {
                '0' => "R",
                '1' => "D",
                '2' => "L",
                '3' => "U",
                _ => panic!("Unexpected direction code"),
            };

            let count = i64::from_str_radix(color.get(0..(color.len() - 1)).unwrap(), 16).unwrap();

            (dir, count)
        })
        .collect_vec();
    println!("Part 2: {}", solve(instructions, args.debug));
}

fn solve(instructions: Vec<(&str, i64)>, debug: bool) -> isize {
    let mut x = 0;
    let mut y = 0;

    let mut dug = Vec::new();

    let mut previous_direction = (0, 0);
    for instruction in instructions.iter() {
        let direction: (isize, isize) = match instruction.0 {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => panic!("Unexpected direction"),
        };

        let new_x = x + direction.0 * instruction.1 as isize;
        let new_y = y + direction.1 * instruction.1 as isize;
        dug.push((
            (x, y),
            previous_direction,
            direction,
            instruction.1 as isize,
        ));
        previous_direction = direction;
        x = new_x;
        y = new_y;
    }

    let before_origin = dug.iter().find(|entry| {
        entry.0 .0 + entry.2 .0 * entry.3 == 0 && entry.0 .1 + entry.2 .1 * entry.3 == 0
    });
    dug[0].1 = before_origin.unwrap().2;

    // Shoelace method
    let left = dug
        .iter()
        .tuple_windows()
        .map(|(left, right)| left.0 .0 * right.0 .1)
        .sum::<isize>()
        + dug.last().unwrap().0 .0 * dug[0].0 .1;
    let right = dug
        .iter()
        .tuple_windows()
        .map(|(left, right)| left.0 .1 * right.0 .0)
        .sum::<isize>()
        + dug.last().unwrap().0 .1 * dug[0].0 .0;
    let shoelace = (left - right).abs() / 2;

    // Pick's Theorom
    let perimeter = dug
        .iter()
        .map(|(_, _, _, distance)| *distance)
        .sum::<isize>();
    return shoelace + perimeter / 2 + 1;
}

// Notes for tomorrow: We need to just record the lengths of everything and can key them to the min x and min y value to make everything easier
// Then we just carry previously seen vertical pieces with us as we walk down the layers and drop them once they are past their length

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

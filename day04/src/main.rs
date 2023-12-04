use clap::Parser;
use itertools::Itertools;
use std::{
    collections::HashSet,
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

    let cards = lines
        .iter()
        .map(|line| {
            let line = line.replace("Card", "");
            let (card_number, remainder) = line.trim().split(":").collect_tuple().unwrap();
            let (winning_numbers, my_numbers) = remainder.split("|").collect_tuple().unwrap();
            let winning_numbers = winning_numbers
                .trim()
                .split_ascii_whitespace()
                .map(|number| number.parse::<u32>().unwrap())
                .sorted()
                .collect_vec();
            let my_numbers = my_numbers
                .trim()
                .split_ascii_whitespace()
                .map(|number| number.parse::<u32>().unwrap())
                .sorted()
                .collect_vec();

            (card_number.to_string(), winning_numbers, my_numbers)
        })
        .collect_vec();

    let scores: Vec<usize> = cards
        .iter()
        .map(|(_, winning, my_numbers)| {
            let my_numbers: HashSet<u32> = HashSet::from_iter(my_numbers.iter().cloned());
            let winning: HashSet<u32> = HashSet::from_iter(winning.iter().cloned());
            let count: usize = my_numbers.intersection(&winning).count();
            if count == 0 {
                0
            } else {
                (2 as usize).pow(count as u32 - 1)
            }
        })
        .collect();

    println!("Part 1: {}", scores.iter().sum::<usize>());
}

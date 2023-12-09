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

    let sequences = lines
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|entry| entry.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let new_entries = sequences
        .iter()
        .map(|sequence| {
            let mut history = Vec::new();
            history.push(sequence.clone());

            while !history.last().unwrap().iter().all(|entry| *entry == 0) {
                history.push(
                    history
                        .last()
                        .unwrap()
                        .iter()
                        .tuple_windows()
                        .map(|(a, b)| b - a)
                        .collect_vec(),
                );
            }

            let mut history_index = history.len() - 1;

            while history_index > 0 {
                let upper_row_last = *history[history_index - 1].last().unwrap();
                let current_row_last = *history[history_index].last().unwrap_or(&0);
                let upper_row_first = *history[history_index - 1].first().unwrap();
                let current_row_first = *history[history_index].first().unwrap();
                history[history_index - 1].push(upper_row_last + current_row_last);
                history[history_index - 1].insert(0, upper_row_first - current_row_first);

                history_index -= 1;

                if args.debug {
                    println!("{:?}", history);
                }
            }

            (*history[0].first().unwrap(), *history[0].last().unwrap())
        })
        .collect_vec();

    let part1: i64 = new_entries.iter().map(|(_, second)| second).sum();
    let part2: i64 = new_entries.iter().map(|(first, _)| first).sum();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

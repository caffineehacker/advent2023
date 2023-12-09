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

    let times = lines[0]
        .split_ascii_whitespace()
        .skip(1)
        .map(|time| time.parse::<i32>().unwrap())
        .collect_vec();
    let distances = lines[1]
        .split_ascii_whitespace()
        .skip(1)
        .map(|distance| distance.parse::<i32>().unwrap())
        .collect_vec();

    let mut race_press_distance = Vec::new();
    for i in 0..times.len() {
        let time = times[i];

        let mut press_distance = Vec::new();
        for t in 0..=time {
            if args.debug {
                println!("{}: t={}, d={}", i, t, (time - t) * t);
            }
            press_distance.push((t, (time - t) * t));
        }

        race_press_distance.push(press_distance);
    }

    let mut part1 = 1;
    for i in 0..times.len() {
        let distance_to_beat = distances[i];

        let ways_beat = race_press_distance[i]
            .iter()
            .filter(|(_, distance)| *distance > distance_to_beat)
            .count();

        if args.debug {
            println!("{}: Beat {} by {} ways", i, distance_to_beat, ways_beat);
        }

        part1 *= ways_beat;
    }

    println!("Part 1: {}", part1);
}

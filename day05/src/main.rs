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

    let seeds = lines
        .get(0)
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect_vec();

    let mut source_dest_map: HashMap<String, String> = HashMap::new();
    // (Source, Dest) -> (Source Start, (DestStart, Length))
    let mut source_dest_value_map: HashMap<(String, String), HashMap<u64, (u64, u64)>> =
        HashMap::new();

    let mut source = "";
    let mut dest = "";
    for line in lines.iter().skip(1) {
        if line.is_empty() {
            continue;
        }
        if line.contains("-to-") {
            (source, dest) = line
                .split_ascii_whitespace()
                .collect_vec()
                .get(0)
                .unwrap()
                .split("-to-")
                .collect_tuple()
                .unwrap();
            source_dest_map.insert(source.to_string(), dest.to_string());
            source_dest_value_map.insert((source.to_string(), dest.to_string()), HashMap::new());
            continue;
        }

        let (dest_start, source_start, length) =
            line.split_ascii_whitespace().collect_tuple().unwrap();
        source_dest_value_map
            .get_mut(&(source.to_string(), dest.to_string()))
            .unwrap()
            .insert(
                source_start.parse::<u64>().unwrap(),
                (
                    dest_start.parse::<u64>().unwrap(),
                    length.parse::<u64>().unwrap(),
                ),
            );
    }

    let mut lowest_location = u64::MAX;

    'nextSeed: for seed in seeds.iter() {
        let mut source_category = "seed";
        let mut source_value = *seed;

        if args.debug {
            println!("Processing seed: {}", seed);
        }

        while source_category != "location" {
            let dest_category = source_dest_map.get(source_category).unwrap();
            let map_entry = source_dest_value_map
                .get(&(source_category.to_string(), dest_category.to_string()))
                .unwrap()
                .iter()
                .find(|entry| *entry.0 <= source_value && (*entry.0 + entry.1 .1) > source_value);
            if map_entry.is_none() {
                if args.debug {
                    println!(
                        "{} ({}) to {} ({})",
                        source_category, source_value, dest_category, source_value
                    );
                }
            } else {
                let map_entry = map_entry.unwrap();
                let new_source_value = map_entry.1 .0 + (source_value - map_entry.0);
                if args.debug {
                    println!(
                        "{} ({}) to {} ({})",
                        source_category, source_value, dest_category, new_source_value
                    );
                }
                source_value = new_source_value;
            }
            source_category = dest_category;
        }

        if source_value < lowest_location {
            lowest_location = source_value;
        }
    }

    println!("Part 1: {}", lowest_location);
}

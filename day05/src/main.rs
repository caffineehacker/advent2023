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

    for seed in seeds.iter() {
        let location_value =
            get_location_value(*seed, &source_dest_map, &source_dest_value_map, args.debug);

        if location_value < lowest_location {
            lowest_location = location_value;
        }
    }

    println!("Part 1: {}", lowest_location);

    // lowest_location = u64::MAX;
    // let mut seed_index = 0;
    // while seed_index < seeds.len() {
    //     let seed_start = seeds[seed_index];
    //     let seed_range_length = seeds[seed_index + 1];
    //     seed_index += 2;

    //     let pb = indicatif::ProgressBar::new(seed_range_length);
    //     for seed in seed_start..(seed_start + seed_range_length) {
    //         let location_value =
    //             get_location_value(seed, &source_dest_map, &source_dest_value_map, args.debug);

    //         if location_value < lowest_location {
    //             lowest_location = location_value;
    //         }
    //         pb.inc(1);
    //     }
    //     pb.finish_and_clear();
    // }

    println!(
        "Part 2: {}",
        fast_process_part2(&seeds, &source_dest_map, &source_dest_value_map, args.debug)
    );
}

fn get_location_value(
    seed: u64,
    source_dest_map: &HashMap<String, String>,
    source_dest_value_map: &HashMap<(String, String), HashMap<u64, (u64, u64)>>,
    debug: bool,
) -> u64 {
    let mut source_category = "seed";
    let mut source_value = seed;

    if debug {
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
            if debug {
                println!(
                    "{} ({}) to {} ({})",
                    source_category, source_value, dest_category, source_value
                );
            }
        } else {
            let map_entry = map_entry.unwrap();
            let new_source_value = map_entry.1 .0 + (source_value - map_entry.0);
            if debug {
                println!(
                    "{} ({}) to {} ({})",
                    source_category, source_value, dest_category, new_source_value
                );
            }
            source_value = new_source_value;
        }
        source_category = dest_category;
    }

    return source_value;
}

fn fast_process_part2(
    seeds: &Vec<u64>,
    source_dest_map: &HashMap<String, String>,
    source_dest_value_map: &HashMap<(String, String), HashMap<u64, (u64, u64)>>,
    debug: bool,
) -> u64 {
    let mut seed_index = 0;
    let mut source_indexes = Vec::new();
    while seed_index < seeds.len() {
        let seed_start = seeds[seed_index];
        let seed_range_length = seeds[seed_index + 1];
        source_indexes.push((seed_start, seed_range_length));
        seed_index += 2;
    }

    let mut source_type = "seed";
    while source_type != "location" {
        source_indexes = source_indexes
            .iter()
            .flat_map(|(source_start, source_length)| {
                map_range_to_range(
                    source_type,
                    *source_start,
                    *source_length,
                    source_dest_map,
                    source_dest_value_map,
                    debug,
                )
            })
            .collect_vec();
        source_type = source_dest_map.get(source_type).unwrap();
    }

    return source_indexes
        .iter()
        .map(|(start, _)| *start)
        .min()
        .unwrap();
}

fn map_range_to_range(
    source_category: &str,
    source_start: u64,
    source_length: u64,
    source_dest_map: &HashMap<String, String>,
    source_dest_value_map: &HashMap<(String, String), HashMap<u64, (u64, u64)>>,
    debug: bool,
) -> Vec<(u64, u64)> {
    let dest_category = source_dest_map.get(source_category).unwrap();
    let map_entries = source_dest_value_map
        .get(&(source_category.to_string(), dest_category.to_string()))
        .unwrap()
        .iter()
        .sorted_by_key(|entry| entry.0)
        .collect_vec();

    // (Source, Dest) -> (Source Start, (DestStart, Length))

    let mut output = Vec::new();
    let mut next_start = source_start;
    let end = source_start + source_length;
    for entry_index in 0..map_entries.len() {
        let (entry_source_start, (entry_dest_start, entry_length)) = map_entries[entry_index];

        if *entry_source_start > next_start {
            output.push((
                next_start,
                (entry_source_start - next_start).min(end - next_start),
            ));
            next_start = *entry_source_start;
        }

        if next_start >= end {
            break;
        }

        if *entry_source_start <= next_start && entry_source_start + entry_length > next_start {
            let output_length = (entry_length - (next_start - entry_source_start))
                .min(source_length - (next_start - source_start));
            output.push((
                entry_dest_start + next_start - entry_source_start,
                output_length,
            ));
            next_start += output_length;
        }

        if next_start >= end {
            break;
        }
    }

    if next_start < end {
        output.push((next_start, end - next_start));
    }

    output
}

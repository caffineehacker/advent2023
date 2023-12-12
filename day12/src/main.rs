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

    let mut memoization = HashMap::new();
    let arrangement_counts = lines
        .iter()
        .map(|line| {
            let (springs, groups) = line.split_ascii_whitespace().collect_tuple().unwrap();
            let springs = springs.chars().collect_vec();
            let groups = groups
                .split(",")
                .map(|group| group.parse::<i32>().unwrap())
                .collect_vec();

            let count = valid_count(springs, groups, &mut memoization, args.debug);

            if args.debug {
                println!("{} -> {}", line, count);
            }

            return count;
        })
        .collect_vec();

    println!("Part 1: {}", arrangement_counts.iter().sum::<u64>());

    // Part 2
    let arrangement_counts = lines
        .iter()
        .map(|line| {
            let (springs, groups) = line.split_ascii_whitespace().collect_tuple().unwrap();
            let mut springs = springs.chars().collect_vec();
            let groups = groups
                .split(",")
                .map(|group| group.parse::<i32>().unwrap())
                .collect_vec();

            springs.push('?');
            let mut springs = springs.repeat(5);
            springs.pop();
            let groups = groups.repeat(5);

            let count = valid_count(springs, groups, &mut memoization, args.debug);

            if args.debug {
                println!("{} -> {}", line, count);
            }

            return count;
        })
        .collect_vec();

    println!("Part 2: {}", arrangement_counts.iter().sum::<u64>());
}

fn valid_count(
    springs: Vec<char>,
    groups: Vec<i32>,
    memoization: &mut HashMap<(Vec<char>, Vec<i32>), u64>,
    debug: bool,
) -> u64 {
    if debug {
        println!("Processing {:?}, {:?}", springs, groups);
    }

    let memoization_key = (springs.clone(), groups.clone());

    if memoization.contains_key(&memoization_key) {
        return *memoization.get(&memoization_key).unwrap();
    }

    let mut return_value = 0;

    if springs.len() == 0 {
        if groups.len() == 0 {
            if debug {
                println!("+1");
            }
            return_value = 1;
        } else {
            return_value = 0;
        }
    } else if springs[0] == '.' {
        return_value = valid_count(springs.split_at(1).1.to_vec(), groups, memoization, debug);
    } else if springs[0] == '#' {
        return_value = valid_count_group(springs, groups, memoization, debug);
    } else if springs[0] == '?' {
        return_value = valid_count(
            springs.split_at(1).1.to_vec(),
            groups.clone(),
            memoization,
            debug,
        ) + valid_count_group(springs, groups, memoization, debug);
    }

    memoization.insert(memoization_key, return_value);
    return return_value;
}

fn valid_count_group(
    springs: Vec<char>,
    groups: Vec<i32>,
    memoization: &mut HashMap<(Vec<char>, Vec<i32>), u64>,
    debug: bool,
) -> u64 {
    if debug {
        println!("Processing group {:?}, {:?}", springs, groups);
    }
    if groups.len() == 0 {
        return 0;
    }
    let group = groups[0] as usize;
    if springs.len() < group {
        return 0;
    }

    for i in 1..group {
        if springs[i] == '.' {
            return 0;
        }
    }

    if springs.len() == group {
        if groups.len() == 1 {
            if debug {
                println!("+1");
            }
            return 1;
        }

        return 0;
    }

    if springs.len() > group {
        if springs[group] == '#' {
            return 0;
        }

        return valid_count(
            springs.split_at(group + 1).1.to_vec(),
            groups.split_at(1).1.to_vec(),
            memoization,
            debug,
        );
    }

    return 0;
}

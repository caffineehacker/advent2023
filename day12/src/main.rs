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

    let arrangement_counts = lines
        .iter()
        .map(|line| {
            let (springs, groups) = line.split_ascii_whitespace().collect_tuple().unwrap();
            let springs = springs.chars().collect_vec();
            let groups = groups
                .split(",")
                .map(|group| group.parse::<i32>().unwrap())
                .collect_vec();

            let count = valid_count(springs, groups, args.debug);

            if args.debug {
                println!("{} -> {}", line, count);
            }

            return count;
        })
        .collect_vec();

    println!("Part 1: {}", arrangement_counts.iter().sum::<u32>());
}

fn valid_count(springs: Vec<char>, groups: Vec<i32>, debug: bool) -> u32 {
    if debug {
        println!("Processing {:?}, {:?}", springs, groups);
    }
    if springs.len() == 0 {
        if groups.len() == 0 {
            if debug {
                println!("+1");
            }
            return 1;
        } else {
            return 0;
        }
    }

    if springs[0] == '.' {
        return valid_count(springs.split_at(1).1.to_vec(), groups, debug);
    }

    if springs[0] == '#' {
        return valid_count_group(springs, groups, debug);
    }

    if springs[0] == '?' {
        return valid_count(springs.split_at(1).1.to_vec(), groups.clone(), debug)
            + valid_count_group(springs, groups, debug);
    }

    panic!("Unexpected char");
}

fn valid_count_group(springs: Vec<char>, groups: Vec<i32>, debug: bool) -> u32 {
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
            debug,
        );
    }

    return 0;
}

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

    let grid = lines
        .iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    // Part 1 is just slide north, easiest to reverse the grid and do slide south
    let mut part1_grid = grid.clone();
    let mut part1 = 0;
    part1_grid.reverse();
    for y in 0..part1_grid.len() {
        for x in 0..part1_grid[0].len() {
            if part1_grid[y][x] == 'O' {
                if args.debug {
                    println!("Processing rock at {}, {}", x, y);
                }
                let mut additional_rocks = 0;
                let mut processed = false;
                for i in (y + 1)..part1_grid.len() {
                    if part1_grid[i][x] == 'O' {
                        additional_rocks += 1;
                    } else if part1_grid[i][x] == '#' {
                        if args.debug {
                            println!(
                                "{}, {} -> {}, {} with {} additional rocks",
                                x, y, x, i, additional_rocks
                            );
                        }
                        part1 += i - additional_rocks;
                        processed = true;
                        break;
                    }
                }

                if !processed {
                    if args.debug {
                        println!(
                            "{}, {} -> {}, {} with {} additional rocks",
                            x, y, x, 0, additional_rocks
                        );
                    }
                    part1 += part1_grid.len() - additional_rocks;
                }
            }
            if args.debug {
                println!("Score: {}", part1);
            }
        }
    }

    println!("Part 1: {}", part1);

    let part2: i32 = 0;
    println!("Part 2: {}", part2);
}
